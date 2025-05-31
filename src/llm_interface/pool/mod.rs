pub mod analyser;
pub mod builder;
use serde::{Deserialize, Serialize};
use tracing::error;

pub use crate::llm_interface::client::LlmClient;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime},
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Behavior {
    Distribute,
    Failover,
    Combination,
}

#[derive(Clone)]
pub struct PoolMember {
    priority: usize,
    client: LlmClient,
    last_error: Option<SystemTime>,
}

impl PoolMember {
    pub fn new(priority: usize, client: LlmClient) -> Self {
        Self {
            priority,
            client,
            last_error: None,
        }
    }
}

pub struct LlmPool {
    clients: HashMap<u64, PoolMember>,
    client_order: Vec<u64>,
    pub behavior: Behavior,
    round_robin_index: Arc<AtomicUsize>,
}

// Manual Clone implementation
impl Clone for LlmPool {
    fn clone(&self) -> Self {
        Self {
            clients: self.clients.clone(),
            client_order: self.client_order.clone(),
            behavior: self.behavior.clone(),
            round_robin_index: Arc::new(AtomicUsize::new(
                self.round_robin_index.load(Ordering::Relaxed),
            )),
        }
    }
}

impl LlmPool {
    pub fn new(clients: Vec<PoolMember>, behavior: Behavior) -> Self {
        let mut client_map = HashMap::new();
        let mut client_order = Vec::new();

        for member in clients {
            let client_id = member.client.id();
            client_order.push(client_id);
            client_map.insert(client_id, member);
        }

        Self {
            clients: client_map,
            client_order,
            behavior,
            round_robin_index: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Returns a client based on behavior
    pub fn get_client(&self) -> Arc<LlmClient> {
        if self.clients.is_empty() {
            panic!("No Configured Clients");
        }

        match self.behavior {
            Behavior::Distribute => self.get_distribute_client(),
            Behavior::Failover => self.get_failover_client(),
            Behavior::Combination => self.get_combination_client(),
        }
    }

    fn get_distribute_client(&self) -> Arc<LlmClient> {
        let current_index =
            self.round_robin_index.fetch_add(1, Ordering::Relaxed) % self.client_order.len();

        let client_id = self.client_order[current_index];
        let member = &self.clients[&client_id];
        Arc::new(member.client.clone())
    }

    fn get_failover_client(&self) -> Arc<LlmClient> {
        // Sort by priority (lower number = higher priority)
        let mut sorted_clients: Vec<_> = self.clients.values().collect();
        sorted_clients.sort_by_key(|member| member.priority);

        let now = SystemTime::now();
        const ERROR_COOLDOWN: Duration = Duration::from_secs(60);

        // Find the highest priority client that's not in error state
        for member in &sorted_clients {
            if let Some(last_error) = member.last_error {
                if now.duration_since(last_error).unwrap_or(Duration::ZERO) < ERROR_COOLDOWN {
                    continue;
                }
            }
            return Arc::new(member.client.clone());
        }

        // If all clients are in error state, return the highest priority one anyway
        sorted_clients
            .first()
            .map(|member| Arc::new(member.client.clone()))
            .expect("Should be at least one client")
    }

    fn get_combination_client(&self) -> Arc<LlmClient> {
        let now = SystemTime::now();
        const ERROR_COOLDOWN: Duration = Duration::from_secs(60);

        // Group by priority
        let mut priority_groups: std::collections::BTreeMap<usize, Vec<&PoolMember>> =
            std::collections::BTreeMap::new();

        for member in self.clients.values() {
            priority_groups
                .entry(member.priority)
                .or_default()
                .push(member);
        }

        // Try each priority group in order (lower priority number first)
        for (_, group) in priority_groups {
            // Filter out clients in error cooldown
            let available_clients: Vec<_> = group
                .into_iter()
                .filter(|member| {
                    member
                        .last_error
                        .map(|last_error| {
                            now.duration_since(last_error).unwrap_or(Duration::ZERO)
                                >= ERROR_COOLDOWN
                        })
                        .unwrap_or(true)
                })
                .collect();

            if !available_clients.is_empty() {
                // Distribute among available clients in this priority group
                let current_index = self.round_robin_index.fetch_add(1, Ordering::Relaxed)
                    % self.client_order.len();

                let selected_client = available_clients[current_index % available_clients.len()];
                return Arc::new(selected_client.client.clone());
            }
        }

        // If no clients are available, return the highest priority one anyway
        self.clients
            .values()
            .min_by_key(|member| member.priority)
            .map(|member| Arc::new(member.client.clone()))
            .expect("Should contain at least one client")
    }

    /// Mark a client as having an error - now uses client ID for O(1) lookup
    pub fn mark_error(&mut self, client: &LlmClient) {
        let client_id = client.id();
        if let Some(member) = self.clients.get_mut(&client_id) {
            member.last_error = Some(SystemTime::now());
        }
    }

    #[allow(dead_code)]
    /// Clear error state for a client - now uses client ID for O(1) lookup
    pub fn clear_error(&mut self, client: &LlmClient) {
        let client_id = client.id();
        if let Some(member) = self.clients.get_mut(&client_id) {
            member.last_error = None;
        }
    }

    #[allow(dead_code)]
    /// Add a new client to the pool
    pub fn add_client(&mut self, member: PoolMember) {
        let client_id = member.client.id();
        self.client_order.push(client_id);
        self.clients.insert(client_id, member);
    }

    #[allow(dead_code)]
    /// Remove a client from the pool
    pub fn remove_client(&mut self, client: &LlmClient) -> Option<PoolMember> {
        let client_id = client.id();
        self.client_order.retain(|&id| id != client_id);
        self.clients.remove(&client_id)
    }

    /// Get the number of clients in the pool
    pub fn len(&self) -> usize {
        self.clients.len()
    }

    #[allow(dead_code)]
    /// Check if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    /// Executes the request with the pool behavior
    pub async fn execute_request<T, F, Fut>(
        &self,
        request_fn: F,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(Arc<LlmClient>) -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
    {
        match self.behavior {
            Behavior::Distribute => {
                let client = self.get_client();
                request_fn(client).await
            }
            _ => {
                let mut last_error = None;
                let max_attempts = self.len();

                for attempt in 0..max_attempts {
                    let client = self.get_client();

                    match request_fn(client.clone()).await {
                        Ok(result) => return Ok(result),
                        Err(e) => {
                            error!("Attempt {} failed: {}", attempt + 1, e);
                            // Mark this client as errored
                            if let Ok(mut pool) = Arc::try_unwrap(Arc::new(self.clone())) {
                                pool.mark_error(&client);
                            }
                            last_error = Some(e);
                        }
                    }
                }

                error!("All pool attempts failed");
                Err(last_error.unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm_interface::models::ModelId; // Adjust this import path as needed
    use std::time::{Duration, SystemTime};

    // Helper to create test clients
    fn create_test_client(api_key: &str, model: ModelId) -> LlmClient {
        LlmClient::new(model, Some(api_key.to_string()), None, None)
    }

    fn create_pool_member(api_key: &str, model: ModelId, priority: usize) -> PoolMember {
        PoolMember {
            priority,
            client: create_test_client(api_key, model),
            last_error: None,
        }
    }

    fn create_pool_member_with_error(
        api_key: &str,
        model: ModelId,
        priority: usize,
        error_time: SystemTime,
    ) -> PoolMember {
        PoolMember {
            priority,
            client: create_test_client(api_key, model),
            last_error: Some(error_time),
        }
    }

    // Helper to identify clients by their ID
    fn client_id(client: &LlmClient) -> u64 {
        client.id()
    }

    #[test]
    #[should_panic(expected = "No Configured Clients")]
    fn test_empty_pool_panics() {
        let pool = LlmPool::new(vec![], Behavior::Distribute);
        pool.get_client();
    }

    #[test]
    fn test_pool_construction() {
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
        ];
        let pool = LlmPool::new(members, Behavior::Distribute);

        assert_eq!(pool.len(), 2);
        assert!(!pool.is_empty());
        assert_eq!(pool.client_order.len(), 2);
    }

    #[test]
    fn test_distribute_single_client() {
        let members = vec![create_pool_member("key1", ModelId::Gpt4o, 1)];
        let pool = LlmPool::new(members, Behavior::Distribute);

        let client1 = pool.get_client();
        let client2 = pool.get_client();

        assert_eq!(client_id(&client1), client_id(&client2));
    }

    #[test]
    fn test_distribute_round_robin() {
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
            create_pool_member("key3", ModelId::Gemini15Pro, 3),
        ];
        let pool = LlmPool::new(members, Behavior::Distribute);

        let client1 = pool.get_client();
        let client2 = pool.get_client();
        let client3 = pool.get_client();
        let client4 = pool.get_client(); // Should wrap around

        let id1 = client_id(&client1);
        let id2 = client_id(&client2);
        let id3 = client_id(&client3);
        let id4 = client_id(&client4);

        // Should cycle through all clients
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_eq!(id1, id4); // Should wrap around to first client
    }

    #[test]
    fn test_failover_priority_order() {
        let members = vec![
            create_pool_member("key3", ModelId::Gpt4o, 3), // Low priority
            create_pool_member("key1", ModelId::Claude35Sonnet, 1), // High priority
            create_pool_member("key2", ModelId::Gemini15Pro, 2), // Medium priority
        ];
        let pool = LlmPool::new(members, Behavior::Failover);

        let client = pool.get_client();
        // Should return the client with priority 1 (highest priority)
        let expected_client = create_test_client("key1", ModelId::Claude35Sonnet);
        assert_eq!(client_id(&client), client_id(&expected_client));
    }

    #[test]
    fn test_failover_skips_errored_clients() {
        let now = SystemTime::now();
        let members = vec![
            create_pool_member_with_error("key1", ModelId::Gpt4o, 1, now), // High priority but errored
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),        // Medium priority
            create_pool_member("key3", ModelId::Gemini15Pro, 3),           // Low priority
        ];
        let pool = LlmPool::new(members, Behavior::Failover);

        let client = pool.get_client();
        let expected_client = create_test_client("key2", ModelId::Claude35Sonnet);
        assert_eq!(client_id(&client), client_id(&expected_client));
    }

    #[test]
    fn test_failover_uses_errored_client_after_cooldown() {
        let old_error = SystemTime::now() - Duration::from_secs(120); // 2 minutes ago
        let members = vec![
            create_pool_member_with_error("key1", ModelId::Gpt4o, 1, old_error),
            create_pool_member("key2", ModelId::Gemini15Pro, 3),
        ];
        let pool = LlmPool::new(members, Behavior::Failover);

        let client = pool.get_client();
        let expected_client = create_test_client("key1", ModelId::Gpt4o);
        assert_eq!(client_id(&client), client_id(&expected_client));
    }

    #[test]
    fn test_failover_returns_highest_priority_when_all_errored() {
        let now = SystemTime::now();
        let members = vec![
            create_pool_member_with_error("key1", ModelId::Gpt4o, 1, now),
            create_pool_member_with_error("key2", ModelId::Gemini15Pro, 3, now),
        ];
        let pool = LlmPool::new(members, Behavior::Failover);

        let client = pool.get_client();
        let expected_client = create_test_client("key1", ModelId::Gpt4o);
        assert_eq!(client_id(&client), client_id(&expected_client));
    }

    #[test]
    fn test_combination_priority_groups() {
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 1),
            create_pool_member("key3", ModelId::Gemini15Pro, 2),
        ];
        let pool = LlmPool::new(members, Behavior::Combination);

        // Should only use high priority clients (priority 1)
        let client1 = pool.get_client();
        let client2 = pool.get_client();
        let client3 = pool.get_client();

        let id1 = client_id(&client1);
        let id2 = client_id(&client2);
        let id3 = client_id(&client3);

        let high_priority_id1 = client_id(&create_test_client("key1", ModelId::Gpt4o));
        let high_priority_id2 = client_id(&create_test_client("key2", ModelId::Claude35Sonnet));
        let low_priority_id = client_id(&create_test_client("key3", ModelId::Gemini15Pro));

        // Should distribute between high priority clients, never use low priority
        assert!(id1 == high_priority_id1 || id1 == high_priority_id2);
        assert!(id2 == high_priority_id1 || id2 == high_priority_id2);
        assert!(id3 == high_priority_id1 || id3 == high_priority_id2);

        // Should not use low priority client
        assert_ne!(id1, low_priority_id);
        assert_ne!(id2, low_priority_id);
        assert_ne!(id3, low_priority_id);
    }

    #[test]
    fn test_combination_falls_back_to_lower_priority() {
        let now = SystemTime::now();
        let members = vec![
            create_pool_member_with_error("key1", ModelId::Gpt4o, 1, now),
            create_pool_member_with_error("key2", ModelId::Claude35Sonnet, 1, now),
            create_pool_member("key3", ModelId::Gemini15Pro, 2),
            create_pool_member("key4", ModelId::DeepseekChat, 2),
        ];
        let pool = LlmPool::new(members, Behavior::Combination);

        let client1 = pool.get_client();
        let client2 = pool.get_client();

        let id1 = client_id(&client1);
        let id2 = client_id(&client2);

        let low_priority_id1 = client_id(&create_test_client("key3", ModelId::Gemini15Pro));
        let low_priority_id2 = client_id(&create_test_client("key4", ModelId::DeepseekChat));

        // Should use low priority clients since high priority are errored
        assert!(id1 == low_priority_id1 || id1 == low_priority_id2);
        assert!(id2 == low_priority_id1 || id2 == low_priority_id2);
        assert_ne!(id1, id2); // Should alternate between the two
    }

    #[test]
    fn test_mark_error() {
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
        ];
        let mut pool = LlmPool::new(members, Behavior::Failover);

        let client1 = pool.get_client();
        let expected_id = client_id(&create_test_client("key1", ModelId::Gpt4o));
        assert_eq!(client_id(&client1), expected_id);

        // Mark client1 as errored
        pool.mark_error(&client1);

        // Should now return client2
        let client2 = pool.get_client();
        let expected_id2 = client_id(&create_test_client("key2", ModelId::Claude35Sonnet));
        assert_eq!(client_id(&client2), expected_id2);
    }

    #[test]
    fn test_clear_error() {
        let now = SystemTime::now();
        let members = vec![
            create_pool_member_with_error("key1", ModelId::Gpt4o, 1, now),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
        ];
        let mut pool = LlmPool::new(members, Behavior::Failover);

        // Should return client2 due to client1 being errored
        let client = pool.get_client();
        let expected_id = client_id(&create_test_client("key2", ModelId::Claude35Sonnet));
        assert_eq!(client_id(&client), expected_id);

        // Clear error for client1
        let client1 = create_test_client("key1", ModelId::Gpt4o);
        pool.clear_error(&client1);

        // Should now return client1 (higher priority)
        let client = pool.get_client();
        let expected_id1 = client_id(&create_test_client("key1", ModelId::Gpt4o));
        assert_eq!(client_id(&client), expected_id1);
    }

    #[test]
    fn test_add_client() {
        let members = vec![create_pool_member("key1", ModelId::Gpt4o, 1)];
        let mut pool = LlmPool::new(members, Behavior::Distribute);

        assert_eq!(pool.len(), 1);

        let new_member = create_pool_member("key2", ModelId::Claude35Sonnet, 2);
        pool.add_client(new_member);

        assert_eq!(pool.len(), 2);
        assert_eq!(pool.client_order.len(), 2);
    }

    #[test]
    fn test_remove_client() {
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
        ];
        let mut pool = LlmPool::new(members, Behavior::Distribute);

        assert_eq!(pool.len(), 2);

        let client_to_remove = create_test_client("key1", ModelId::Gpt4o);
        let removed = pool.remove_client(&client_to_remove);

        assert!(removed.is_some());
        assert_eq!(pool.len(), 1);
        assert_eq!(pool.client_order.len(), 1);
    }

    #[test]
    fn test_round_robin_index_persistence() {
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
        ];
        let pool = LlmPool::new(members, Behavior::Distribute);

        let client1 = pool.get_client();
        let client2 = pool.get_client();
        let client3 = pool.get_client();

        let id1 = client_id(&client1);
        let id2 = client_id(&client2);
        let id3 = client_id(&client3);

        assert_ne!(id1, id2); // Should be different clients
        assert_eq!(id1, id3); // Should wrap around to first client
    }

    #[test]
    fn test_clone_preserves_state() {
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
        ];
        let pool = LlmPool::new(members, Behavior::Distribute);

        // Advance the round robin
        let _ = pool.get_client();

        let cloned_pool = pool.clone();
        let client = cloned_pool.get_client();

        // Both pools should have the same round robin state
        let original_client = pool.get_client();
        assert_eq!(client_id(&client), client_id(&original_client));
    }

    #[test]
    fn test_hashmap_efficiency() {
        // Test that we can efficiently find clients by ID
        let members = vec![
            create_pool_member("key1", ModelId::Gpt4o, 1),
            create_pool_member("key2", ModelId::Claude35Sonnet, 2),
            create_pool_member("key3", ModelId::Gemini15Pro, 3),
        ];
        let mut pool = LlmPool::new(members, Behavior::Distribute);

        let test_client = create_test_client("key2", ModelId::Claude35Sonnet);
        let client_id = test_client.id();

        // Verify the client exists in our HashMap
        assert!(pool.clients.contains_key(&client_id));

        // Mark error should work efficiently
        pool.mark_error(&test_client);
        let member = pool.clients.get(&client_id).unwrap();
        assert!(member.last_error.is_some());

        // Clear error should work efficiently
        pool.clear_error(&test_client);
        let member = pool.clients.get(&client_id).unwrap();
        assert!(member.last_error.is_none());
    }

    #[test]
    fn test_different_providers() {
        // Test with clients from different providers
        let members = vec![
            create_pool_member("openai_key", ModelId::Gpt4o, 1),
            create_pool_member("anthropic_key", ModelId::Claude35Sonnet, 1),
            create_pool_member("google_key", ModelId::Gemini15Pro, 1),
            create_pool_member("deepseek_key", ModelId::DeepseekChat, 1),
        ];
        let pool = LlmPool::new(members, Behavior::Combination);

        // All have same priority, so should distribute among all
        let mut seen_providers = std::collections::HashSet::new();
        for _ in 0..8 {
            let client = pool.get_client();
            seen_providers.insert(format!("{:?}", client.model.provider()));
        }

        // Should have used multiple providers
        assert!(seen_providers.len() > 1);
    }

    #[test]
    fn test_unique_client_ids() {
        // Test that different combinations of api_key + model create unique IDs
        let client1 = create_test_client("key1", ModelId::Gpt4o);
        let client2 = create_test_client("key2", ModelId::Gpt4o);
        let client3 = create_test_client("key1", ModelId::Claude35Sonnet);
        let client4 = create_test_client("key1", ModelId::Gpt4o); // Same as client1

        assert_ne!(client_id(&client1), client_id(&client2)); // Different keys
        assert_ne!(client_id(&client1), client_id(&client3)); // Different models
        assert_eq!(client_id(&client1), client_id(&client4)); // Same key + model
    }
}

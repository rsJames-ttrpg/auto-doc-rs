#![allow(dead_code)]
use super::{Behavior, LlmPool, PoolMember};
use crate::llm_interface::client::LlmClient;

/// Builder for constructing LlmPool instances
pub struct LlmPoolBuilder {
    members: Vec<PoolMember>,
    behavior: Option<Behavior>,
}

impl LlmPoolBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            behavior: None,
        }
    }

    /// Set the pool behavior
    pub fn behavior(mut self, behavior: Behavior) -> Self {
        self.behavior = Some(behavior);
        self
    }

    /// Add a client with default priority (0)
    pub fn add_client(mut self, client: LlmClient) -> Self {
        self.members.push(PoolMember {
            priority: 0,
            client,
            last_error: None,
        });
        self
    }

    /// Add a client with specified priority
    pub fn add_client_with_priority(mut self, client: LlmClient, priority: usize) -> Self {
        self.members.push(PoolMember {
            priority,
            client,
            last_error: None,
        });
        self
    }

    /// Add multiple clients with default priority
    pub fn add_clients<I>(mut self, clients: I) -> Self
    where
        I: IntoIterator<Item = LlmClient>,
    {
        for client in clients {
            self.members.push(PoolMember {
                priority: 0,
                client,
                last_error: None,
            });
        }
        self
    }

    /// Add multiple clients with the same priority
    pub fn add_clients_with_priority<I>(mut self, clients: I, priority: usize) -> Self
    where
        I: IntoIterator<Item = LlmClient>,
    {
        for client in clients {
            self.members.push(PoolMember {
                priority,
                client,
                last_error: None,
            });
        }
        self
    }

    /// Add a PoolMember directly (for advanced use cases)
    pub fn add_member(mut self, member: PoolMember) -> Self {
        self.members.push(member);
        self
    }

    /// Add multiple PoolMembers directly
    pub fn add_members<I>(mut self, members: I) -> Self
    where
        I: IntoIterator<Item = PoolMember>,
    {
        self.members.extend(members);
        self
    }

    /// Build the LlmPool
    ///
    /// # Errors
    /// Returns an error if:
    /// - No behavior is set
    /// - No clients are added
    pub fn build(self) -> Result<LlmPool, LlmPoolBuilderError> {
        let behavior = self.behavior.ok_or(LlmPoolBuilderError::MissingBehavior)?;

        if self.members.is_empty() {
            return Err(LlmPoolBuilderError::NoClients);
        }

        Ok(LlmPool::new(self.members, behavior))
    }

    /// Build the LlmPool with a default behavior if none is set
    /// Uses `Behavior::Failover` as the default
    pub fn build_with_default_behavior(self) -> Result<LlmPool, LlmPoolBuilderError> {
        if self.members.is_empty() {
            return Err(LlmPoolBuilderError::NoClients);
        }

        let behavior = self.behavior.unwrap_or(Behavior::Failover);
        Ok(LlmPool::new(self.members, behavior))
    }
}

impl Default for LlmPoolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur when building an LlmPool
#[derive(Debug, Clone, PartialEq)]
pub enum LlmPoolBuilderError {
    /// No behavior was specified
    MissingBehavior,
    /// No clients were added to the pool
    NoClients,
}

impl std::fmt::Display for LlmPoolBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmPoolBuilderError::MissingBehavior => {
                write!(f, "Pool behavior must be specified")
            }
            LlmPoolBuilderError::NoClients => {
                write!(f, "At least one client must be added to the pool")
            }
        }
    }
}

impl std::error::Error for LlmPoolBuilderError {}

// Convenience methods for LlmPool
impl LlmPool {
    /// Create a new builder for LlmPool
    pub fn builder() -> LlmPoolBuilder {
        LlmPoolBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_errors() {
        // Test missing behavior
        let result = LlmPool::builder().build();
        assert!(matches!(result, Err(LlmPoolBuilderError::MissingBehavior)));

        // Test no clients
        let result = LlmPool::builder().behavior(Behavior::Distribute).build();
        assert!(matches!(result, Err(LlmPoolBuilderError::NoClients)));
    }

    #[test]
    fn test_default_behavior_builder() {
        // Test build_with_default_behavior with no clients
        let result = LlmPool::builder().build_with_default_behavior();
        assert!(matches!(result, Err(LlmPoolBuilderError::NoClients)));
    }
}

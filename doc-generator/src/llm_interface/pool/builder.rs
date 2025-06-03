#![allow(dead_code)]
use super::{Behaviour, LlmPool, PoolMember};
use crate::llm_interface::client::LlmClient;

/// Builder for constructing LlmPool instances
pub struct LlmPoolBuilder {
    members: Vec<PoolMember>,
    behaviour: Option<Behaviour>,
}

impl LlmPoolBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            behaviour: None,
        }
    }

    /// Set the pool behaviour
    pub fn behaviour(mut self, behaviour: Behaviour) -> Self {
        self.behaviour = Some(behaviour);
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
    /// - No behaviour is set
    /// - No clients are added
    pub fn build(self) -> Result<LlmPool, LlmPoolBuilderError> {
        let behaviour = self
            .behaviour
            .ok_or(LlmPoolBuilderError::Missingbehaviour)?;

        if self.members.is_empty() {
            return Err(LlmPoolBuilderError::NoClients);
        }

        Ok(LlmPool::new(self.members, behaviour))
    }

    /// Build the LlmPool with a default behaviour if none is set
    /// Uses `behaviour::Failover` as the default
    pub fn build_with_default_behaviour(self) -> Result<LlmPool, LlmPoolBuilderError> {
        if self.members.is_empty() {
            return Err(LlmPoolBuilderError::NoClients);
        }

        let behaviour = self.behaviour.unwrap_or(Behaviour::Failover);
        Ok(LlmPool::new(self.members, behaviour))
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
    /// No behaviour was specified
    Missingbehaviour,
    /// No clients were added to the pool
    NoClients,
}

impl std::fmt::Display for LlmPoolBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmPoolBuilderError::Missingbehaviour => {
                write!(f, "Pool behaviour must be specified")
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
        // Test missing behaviour
        let result = LlmPool::builder().build();
        assert!(matches!(result, Err(LlmPoolBuilderError::Missingbehaviour)));

        // Test no clients
        let result = LlmPool::builder().behaviour(Behaviour::Distribute).build();
        assert!(matches!(result, Err(LlmPoolBuilderError::NoClients)));
    }

    #[test]
    fn test_default_behaviour_builder() {
        // Test build_with_default_behaviour with no clients
        let result = LlmPool::builder().build_with_default_behaviour();
        assert!(matches!(result, Err(LlmPoolBuilderError::NoClients)));
    }
}

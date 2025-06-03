# 📄 File Analysis: `./src/llm_interface/pool/builder.rs`

**Type:** `rs`

## Summary
This file implements a builder pattern for constructing LlmPool instances, providing a fluent API to configure pool behaviour and add LLM clients with priorities. It includes validation to ensure pools are built with required behaviour and at least one client, with comprehensive error handling for invalid configurations.

## 📚 External Dependencies
- `std::fmt`
- `std::error::Error`

## 🔌 Public Interfaces
- **LlmPoolBuilder** (`📦 Struct`)
  Builder pattern struct for constructing LlmPool instances with configurable behaviour and client members
- **new** (`🔧 Function`)
  Creates a new LlmPoolBuilder instance with empty members and no behaviour set
- **behaviour** (`🔧 Function`)
  Sets the pool behaviour (Failover, Distribute, etc.) for the LlmPool being built
- **add_client** (`🔧 Function`)
  Adds an LlmClient to the pool with default priority of 0
- **add_client_with_priority** (`🔧 Function`)
  Adds an LlmClient to the pool with a specified priority level
- **add_clients** (`🔧 Function`)
  Adds multiple LlmClients to the pool with default priority of 0
- **add_clients_with_priority** (`🔧 Function`)
  Adds multiple LlmClients to the pool with the same specified priority
- **add_member** (`🔧 Function`)
  Adds a PoolMember directly to the pool for advanced configuration use cases
- **add_members** (`🔧 Function`)
  Adds multiple PoolMembers directly to the pool
- **build** (`🔧 Function`)
  Builds the LlmPool, returning an error if no behaviour is set or no clients are added
- **build_with_default_behaviour** (`🔧 Function`)
  Builds the LlmPool using Failover as default behaviour if none is set
- **LlmPoolBuilderError** (`🗄️ Data Model`)
  Error enum for builder validation failures including Missingbehaviour and NoClients variants
- **builder** (`🔧 Function`)
  Convenience method on LlmPool to create a new LlmPoolBuilder instance

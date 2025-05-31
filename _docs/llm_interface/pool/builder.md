# 📄 File Analysis: `./src/llm_interface/pool/builder.rs`

**Type:** `rs`

## Summary
This file implements a builder pattern for constructing LlmPool instances, providing a fluent API to configure pool behavior and add LLM clients with priorities. It includes validation to ensure pools are built with required behavior and at least one client, with comprehensive error handling for invalid configurations.

## 📚 External Dependencies
- `std::fmt`
- `std::error::Error`

## 🔌 Public Interfaces
- **LlmPoolBuilder** (`📦 Struct`)
  Builder pattern struct for constructing LlmPool instances with configurable behavior and client members
- **new** (`🔧 Function`)
  Creates a new LlmPoolBuilder instance with empty members and no behavior set
- **behavior** (`🔧 Function`)
  Sets the pool behavior (Failover, Distribute, etc.) for the LlmPool being built
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
  Builds the LlmPool, returning an error if no behavior is set or no clients are added
- **build_with_default_behavior** (`🔧 Function`)
  Builds the LlmPool using Failover as default behavior if none is set
- **LlmPoolBuilderError** (`🗄️ Data Model`)
  Error enum for builder validation failures including MissingBehavior and NoClients variants
- **builder** (`🔧 Function`)
  Convenience method on LlmPool to create a new LlmPoolBuilder instance

# 📄 File Analysis: `./src/llm_interface/pool/mod.rs`

**Type:** `rs`

## Summary
This file implements an LLM client pool system that manages multiple LLM clients with different load balancing behaviours (distribute, failover, combination). It provides client selection strategies, error tracking with cooldown periods, and pool management operations for building resilient LLM integrations.

## 📚 External Dependencies
- `serde`
- `tracing`
- `std::collections::HashMap`
- `std::sync::Arc`
- `std::sync::atomic::AtomicUsize`
- `std::sync::atomic::Ordering`
- `std::time::Duration`
- `std::time::SystemTime`
- `std::collections::BTreeMap`
- `std::error::Error`
- `std::future::Future`
- `crate::llm_interface::client::LlmClient`

## 🔌 Public Interfaces
- **analyser** (`📁 Module`)
  Public module for pool analysis functionality
- **builder** (`📁 Module`)
  Public module for pool builder functionality
- **behaviour** (`📦 Struct`)
  Enum defining load balancing behaviours: Distribute (round-robin), Failover (priority-based), and Combination (priority groups with round-robin within groups)
- **PoolMember** (`📦 Struct`)
  Represents a client in the pool with priority, LLM client instance, and error tracking state
- **LlmPool** (`📦 Struct`)
  Main pool structure that manages multiple LLM clients with configurable load balancing behaviour, error tracking, and client lifecycle management
- **PoolMember::new** (`🔧 Function`)
  Creates a new pool member with specified priority and LLM client
- **LlmPool::new** (`🔧 Function`)
  Creates a new LLM pool with a list of pool members and specified behaviour strategy
- **LlmPool::get_client** (`🔧 Function`)
  Returns an LLM client based on the configured behaviour strategy (distribute, failover, or combination)
- **LlmPool::mark_error** (`🔧 Function`)
  Marks a specific client as having an error with timestamp for cooldown tracking
- **LlmPool::clear_error** (`🔧 Function`)
  Clears the error state for a specific client, making it available for selection again
- **LlmPool::add_client** (`🔧 Function`)
  Adds a new pool member to the existing pool
- **LlmPool::remove_client** (`🔧 Function`)
  Removes a client from the pool and returns the removed pool member if found
- **LlmPool::len** (`🔧 Function`)
  Returns the number of clients currently in the pool
- **LlmPool::is_empty** (`🔧 Function`)
  Returns true if the pool contains no clients
- **LlmPool::execute_request** (`🔧 Function`)
  Executes an async request function with automatic retry logic based on pool behaviour, handling client errors and failover

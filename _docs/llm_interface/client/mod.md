# 📄 File Analysis: `./src/llm_interface/client/mod.rs`

**Type:** `rs`

## Summary
This file implements the core LLM client interface that provides structured and simple response capabilities with retry logic and error handling. It serves as the main abstraction layer for interacting with various LLM providers through a unified API.

## 📚 External Dependencies
- `backoff`
- `llm`
- `schemars`
- `serde`
- `tokio`
- `tracing`
- `std::env`
- `std::hash`
- `std::time`
- `dotenv`

## 🔌 Public Interfaces
- **analyser** (`📁 Module`)
  Public module for analysis functionality
- **builder** (`📁 Module`)
  Public module for builder pattern implementations
- **RetryConfig** (`📦 Struct`)
  Configuration struct for retry behavior including max retries, intervals, and backoff settings
- **LlmClient** (`📦 Struct`)
  Main client struct for interacting with LLM providers, supporting structured and simple responses with retry capabilities
- **LlmRequestBuilder** (`📦 Struct`)
  Builder pattern implementation for constructing LLM requests with fluent API for setting prompts, tokens, and temperature
- **new** (`🔧 Function`)
  Constructor for LlmClient that accepts model, API key, max tokens, and temperature parameters
- **with_retry_config** (`🔧 Function`)
  Method to configure retry behavior for the LlmClient
- **get_structured_response** (`🔧 Function`)
  Async method to get structured JSON responses from LLM based on JSON schema
- **get_structured_response_with_retry** (`🔧 Function`)
  Async method to get structured responses with automatic retry logic and exponential backoff
- **get_simple_response** (`🔧 Function`)
  Async method to get plain text responses from LLM without structured formatting
- **request** (`🔧 Function`)
  Returns a LlmRequestBuilder for fluent API construction of LLM requests

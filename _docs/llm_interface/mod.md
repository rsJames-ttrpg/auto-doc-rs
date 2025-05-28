# 📄 File Analysis: `./src/llm_interface/mod.rs`

**Type:** `rs`

## Summary
This module provides a high-level interface for interacting with various LLM providers through structured and simple text responses. It includes error handling, JSON parsing strategies, and a builder pattern for configuring LLM requests with support for schema-based structured outputs.

## 📚 External Dependencies
- `llm`
- `schemars`
- `serde`
- `serde_json`
- `thiserror`
- `tracing`
- `tokio`
- `dotenv`
- `std::env`

## 🔌 Public Interfaces
- **extract_json** (`📁 Module`)
  Module containing JSON extraction utilities for parsing LLM responses
- **models** (`📁 Module`)
  Module containing model definitions and identifiers for different LLM providers
- **LlmError** (`📦 Struct`)
  Error enum for LLM operations including schema serialization, build, chat, and response parsing errors
- **LlmClient** (`📦 Struct`)
  Main client for interacting with LLM services, supporting multiple providers with configurable parameters
- **LlmClient::new** (`🔧 Function`)
  Constructor for LlmClient that accepts model, API key, max tokens, and temperature configuration
- **LlmClient::get_structured_response** (`🔧 Function`)
  Async method to get structured JSON responses from LLM using JSON schema validation
- **LlmClient::get_simple_response** (`🔧 Function`)
  Async method to get simple text responses from LLM without structured output
- **LlmClient::request** (`🔧 Function`)
  Returns a builder pattern instance for configuring LLM requests
- **LlmRequestBuilder** (`📦 Struct`)
  Builder pattern for ergonomic configuration of LLM requests with fluent API
- **try_parse** (`🔧 Function`)
  Internal function that attempts multiple JSON parsing strategies for LLM responses

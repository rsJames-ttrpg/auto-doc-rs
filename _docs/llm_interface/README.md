# 📁 Directory: `./src/llm_interface`

**Depth Level:** 2

## Summary
Provides a unified, high-level abstraction layer for interacting with multiple LLM providers through a single client interface. Handles provider-specific model capabilities, robust JSON extraction from responses, and supports both structured schema-based outputs and simple text responses with comprehensive error handling and async operations.

## 🎯 Key Components
- **LlmClient**
- **ModelId**
- **Provider**
- **LlmRequestBuilder**
- **extract_json_from_response**

## 📋 Child Summaries
1. models.rs: Defines model identifiers and provider enums for various LLM services with capability detection methods
2. extract_json.rs: Provides utilities for extracting JSON content from LLM response text using regex and brace balancing
3. mod.rs: Main module providing high-level LLM client interface with structured and simple response capabilities

## 📚 External Dependencies
- `clap::ValueEnum`
- `llm`
- `schemars`
- `serde`
- `serde_json`
- `thiserror`
- `tracing`
- `tokio`
- `dotenv`
- `once_cell::sync::Lazy`
- `regex::Regex`
- `strum_macros::EnumIter`

## 🔌 Public Interfaces
- **LlmClient** (`📦 Struct`)
  Primary interface for interacting with multiple LLM providers, supporting both structured JSON and simple text responses with configurable parameters
- **ModelId** (`📦 Struct`)
  Unified model identifier system supporting OpenAI, Anthropic, Google, DeepSeek, xAI, Ollama and other providers with capability detection
- **LlmRequestBuilder** (`📦 Struct`)
  Fluent builder pattern for configuring LLM requests with schema-based structured output support
- **extract_json_from_response** (`🔧 Function`)
  Robust JSON extraction from LLM responses handling various output formats including code blocks and embedded JSON

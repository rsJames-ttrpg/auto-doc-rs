# 📁 Directory: `./src/llm_interface/client`

**Depth Level:** 3

## Summary
This directory implements the core LLM client abstraction layer that provides a unified interface for interacting with various LLM providers. It combines structured and simple response capabilities with robust retry logic, builder pattern configuration, and specialized analysis functionality. The client serves as the primary bridge between the application's analysis framework and external LLM services, handling authentication, request construction, response parsing, and error recovery.

## 🎯 Key Components
- **LlmClient - Main client struct with structured/simple response methods**
- **LlmClientBuilder - Fluent builder pattern for client configuration**
- **RetryConfig/RetryConfigBuilder - Configurable retry logic with exponential backoff**
- **LlmAnalyzer trait implementation - Analysis-specific functionality**
- **LlmRequestBuilder - Request construction with fluent API**

## 📋 Child Summaries
1. analyser.rs: Implements LlmAnalyzer trait for LlmClient, providing async methods for file, directory, and project analysis using LLM-based structured analysis with prompt generation and response parsing
2. mod.rs: Core LLM client implementation with structured/simple response capabilities, retry logic, error handling, and unified API abstraction for multiple LLM providers
3. builder.rs: Fluent builder pattern implementation for LlmClient construction with configurable parameters, retry settings, and comprehensive error handling for missing required parameters

## 📚 External Dependencies
- `async_trait`
- `backoff`
- `dotenv`
- `llm`
- `schemars`
- `serde`
- `serde_json`
- `tokio`
- `tracing`
- `std::env`
- `std::error`
- `std::fmt`
- `std::hash`
- `std::path`
- `std::time`

## 🔌 Public Interfaces
- **LlmClient** (`📦 Struct`)
  Primary client interface for LLM interactions supporting both structured JSON responses and simple text responses with configurable retry behaviour and authentication
- **LlmClientBuilder** (`📦 Struct`)
  Fluent builder pattern for constructing LlmClient instances with configurable model, API key, tokens, temperature, and retry settings
- **LlmAnalyzer** (`🎯 Trait`)
  Analysis trait implementation that enables the client to perform structured code analysis on files, directories, and entire projects using LLM-based prompts
- **RetryConfig** (`📦 Struct`)
  Configuration interface for retry behaviour including maximum retries, intervals, backoff multipliers, and timeout settings
- **get_structured_response** (`🔧 Function`)
  Primary method for obtaining structured JSON responses from LLM providers based on JSON schema validation with optional retry logic
- **get_simple_response** (`🔧 Function`)
  Method for obtaining plain text responses from LLM providers without structured formatting requirements

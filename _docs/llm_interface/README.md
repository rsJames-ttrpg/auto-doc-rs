# 📁 Directory: `./src/llm_interface`

**Depth Level:** 2

## Summary
This directory provides a comprehensive abstraction layer for interacting with multiple LLM providers through a unified interface. It implements client pooling with load balancing strategies, structured response parsing with JSON schema validation, model capability detection, and robust error handling with retry logic. The interface supports both individual client operations and pooled operations for resilience, while providing specialized analysis capabilities for code projects.

## 🎯 Key Components
- **LlmClient - Primary interface for individual LLM interactions**
- **LlmPool - Multi-client pooling with load balancing strategies**
- **ModelId enum - Centralized model identification and capability mapping**
- **JsonSchemaConverter - Schema transformation for LLM consumption**
- **JSON extraction utilities - Response parsing and validation**
- **LlmAnalyzer trait - Code analysis capabilities for files/directories/projects**

## 📋 Child Summaries
1. client/ - Core LLM client implementation with builder pattern, retry logic, and analysis capabilities
2. pool/ - Sophisticated client pooling system with load balancing, failover, and distributed request handling
3. models.rs - Model identifiers and provider enums with capability detection for various LLM services
4. extract_json.rs - JSON extraction utilities for parsing structured responses from mixed LLM output
5. simplified_schema.rs - JSON Schema to SimplifiedSchema converter for LLM-friendly schema representation
6. exceptions.rs - Comprehensive error handling with retry logic and automatic error classification
7. mod.rs - Module organization exposing all LLM interface components

## 📚 External Dependencies
- `async_trait`
- `backoff`
- `clap`
- `dotenv`
- `llm`
- `once_cell`
- `regex`
- `schemars`
- `serde`
- `serde_json`
- `strum_macros`
- `thiserror`
- `tokio`
- `tracing`
- `std::collections`
- `std::env`
- `std::error`
- `std::fmt`
- `std::hash`
- `std::path`
- `std::str`
- `std::sync`
- `std::time`

## 🔌 Public Interfaces
- **LlmClient** (`📦 Struct`)
  Primary client for LLM interactions supporting structured JSON responses, simple text responses, and configurable retry behavior with authentication
- **LlmPool** (`📦 Struct`)
  Multi-client pool with load balancing strategies (distribute, failover, combination), error tracking, and automatic retry logic for resilient LLM operations
- **ModelId** (`🗄️ Data Model`)
  Comprehensive enum of supported LLM models with provider mapping and capability detection (structured output, reasoning, multimodal support)
- **LlmAnalyzer** (`🎯 Trait`)
  Analysis interface for structured code analysis on files, directories, and projects using LLM-based prompts with typed response parsing
- **SimplifiedSchema** (`📦 Struct`)
  LLM-friendly schema representation converted from JSON Schema with validation constraints and metadata preservation
- **extract_json_from_response** (`🔧 Function`)
  Utility for extracting valid JSON objects from mixed LLM response text using multiple parsing strategies
- **LlmError** (`🗄️ Data Model`)
  Comprehensive error handling with automatic retry classification and intelligent error detection from response strings

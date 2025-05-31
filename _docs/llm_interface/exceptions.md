# 📄 File Analysis: `./src/llm_interface/exceptions.rs`

**Type:** `rs`

## Summary
Defines a comprehensive error enum for LLM operations with automatic error classification and retry logic. Provides error categorization for schema serialization, LLM communication, rate limiting, and server errors with intelligent error detection from error strings.

## 📚 External Dependencies
- `thiserror`
- `serde_json`
- `super::simplified_schema`

## 🔌 Public Interfaces
- **LlmError** (`📦 Struct`)
  Main error enum for LLM operations with variants for schema serialization, build errors, chat errors, rate limits, and server errors. Implements automatic conversion from serde_json::Error and simplified_schema::ConversionError.
- **is_retryable** (`🔧 Function`)
  Method that determines if an LlmError instance represents a retryable error condition (rate limits or server errors). Returns boolean indicating retry feasibility.
- **from_error_string** (`🔧 Function`)
  Static method that creates an LlmError from a generic error string by analyzing the content to detect rate limiting (429 errors) or server errors (5xx) and categorizing appropriately.

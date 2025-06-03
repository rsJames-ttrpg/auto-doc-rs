# 📄 File Analysis: `./src/llm_interface/models.rs`

**Type:** `rs`

## Summary
This file defines model identifiers and provider enums for various LLM services including OpenAI, Anthropic, Google, DeepSeek, xAI, and Ollama. It provides a centralized mapping between model IDs and their capabilities like structured output, reasoning, and multimodal support.

## 📚 External Dependencies
- `clap::ValueEnum`
- `llm::builder::LLMBackend`
- `serde::{Deserialize, Serialize}`
- `strum_macros::EnumIter`
- `serde_json`
- `std::fmt`
- `std::str::FromStr`

## 🔌 Public Interfaces
- **ModelId** (`🗄️ Data Model`)
  Enum containing all supported LLM model identifiers with serialization support and provider mapping functionality
- **Provider** (`🗄️ Data Model`)
  Enum representing different LLM service providers like OpenAI, Anthropic, Google, etc.
- **provider** (`🔧 Function`)
  Method on ModelId that returns the LLMBackend provider for a given model
- **supports_structured_output** (`🔧 Function`)
  Method on ModelId that returns whether a model supports structured JSON output
- **has_reasoning** (`🔧 Function`)
  Method on ModelId that returns whether a model has advanced reasoning capabilities
- **is_multimodal** (`🔧 Function`)
  Method on ModelId that returns whether a model supports multimodal input like images and audio

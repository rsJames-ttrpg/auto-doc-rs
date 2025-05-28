# 📄 File Analysis: `./src/llm_interface/models.rs`

**Type:** `rs`

## Summary
Defines model identifiers and provider enums for various LLM services including OpenAI, Anthropic, Google, DeepSeek, xAI, and Ollama. Provides methods to determine provider, capabilities like structured output, reasoning, and multimodal support for each model.

## 📚 External Dependencies
- `clap::ValueEnum`
- `llm::builder::LLMBackend`
- `serde`
- `strum_macros::EnumIter`

## 🔌 Public Interfaces
- **ModelId** (`📦 Struct`)
  Enum representing various LLM model identifiers from different providers with serialization support and command-line argument parsing
- **Provider** (`📦 Struct`)
  Enum representing LLM service providers including OpenAI, Anthropic, Google, Azure, DeepSeek, xAI, Groq, Ollama, and ElevenLabs
- **provider** (`🔧 Function`)
  Returns the LLMBackend provider for a given ModelId instance
- **supports_structured_output** (`🔧 Function`)
  Returns boolean indicating if the model supports structured JSON output generation
- **has_reasoning** (`🔧 Function`)
  Returns boolean indicating if the model has advanced reasoning capabilities
- **is_multimodal** (`🔧 Function`)
  Returns boolean indicating if the model supports multimodal input like images and audio

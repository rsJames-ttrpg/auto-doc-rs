# 📄 File Analysis: `./src/llm_interface/client/builder.rs`

**Type:** `rs`

## Summary
This file implements a fluent builder pattern for constructing LlmClient instances with configurable parameters like model, API key, tokens, temperature, and retry settings. It provides a RetryConfigBuilder for detailed retry configuration and includes error handling for missing required parameters.

## 📚 External Dependencies
- `std::time::Duration`
- `std::fmt`
- `std::error::Error`

## 🔌 Public Interfaces
- **LlmClientBuilder** (`📦 Struct`)
  Builder struct for constructing LlmClient instances with fluent configuration methods including model, API key, max tokens, temperature, and retry settings
- **RetryConfigBuilder** (`📦 Struct`)
  Builder struct for configuring retry behavior with settings like max retries, intervals, multipliers, and elapsed time limits
- **LlmClientBuilderError** (`🗄️ Data Model`)
  Error enum representing possible errors during LlmClient construction, currently includes MissingModel variant
- **new** (`🔧 Function`)
  Creates a new LlmClientBuilder instance with default empty configuration
- **model** (`🔧 Function`)
  Sets the model ID for the LlmClient being built (required parameter)
- **api_key** (`🔧 Function`)
  Sets the API key for authentication with the LLM service
- **max_tokens** (`🔧 Function`)
  Sets the maximum number of tokens for LLM responses (default: 1500)
- **temperature** (`🔧 Function`)
  Sets the temperature parameter for LLM response randomness (default: 0.5)
- **retry_config** (`🔧 Function`)
  Sets a custom retry configuration for handling failed requests
- **with_retries** (`🔧 Function`)
  Enables retries with default retry configuration settings
- **retry_settings** (`🔧 Function`)
  Returns a RetryConfigBuilder for fluent retry configuration
- **build** (`🔧 Function`)
  Constructs the final LlmClient instance, returning an error if required parameters are missing
- **builder** (`🔧 Function`)
  Convenience method on LlmClient to create a new LlmClientBuilder instance
- **with_model** (`🔧 Function`)
  Convenience method on LlmClient to create a builder pre-configured with a specific model

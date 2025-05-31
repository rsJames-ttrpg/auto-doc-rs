# 📄 File Analysis: `./src/settings.rs`

**Type:** `rs`

## Summary
This file defines the configuration system for an application that processes files and interacts with LLMs. It provides serializable configuration structures for file crawling options and LLM settings, with support for loading from files or environment variables and writing default configurations in multiple formats.

## 📚 External Dependencies
- `std::fs::File`
- `std::io`
- `std::path::PathBuf`
- `crate::llm_interface::models::ModelId`
- `crate::llm_interface::pool::behaviour`
- `clap::ValueEnum`
- `config::Config`
- `config::ConfigError`
- `config::Environment`
- `serde::Deserialize`
- `serde::Serialize`
- `serde_json`
- `toml`
- `serde_yaml`

## 🔌 Public Interfaces
- **CrawlOptions** (`📦 Struct`)
  Configuration struct for file crawling options including depth limits, pattern matching, and git mode settings
- **LlmSettings** (`📦 Struct`)
  Configuration struct for LLM behaviour including retry policies, timeout settings, and model configurations
- **LlmModel** (`📦 Struct`)
  Configuration struct for individual LLM model settings including API keys, token limits, and temperature
- **Settings** (`📦 Struct`)
  Main configuration struct combining file crawling options and LLM settings
- **FileType** (`📦 Struct`)
  Enum for supported configuration file formats (Json, Toml, Yaml)
- **from_env** (`🔧 Function`)
  Creates Settings instance from environment variables with AUTODOC prefix
- **from_file** (`🔧 Function`)
  Creates Settings instance from configuration file with environment variable overrides
- **write_default_config** (`🔧 Function`)
  Writes default configuration to file or stdout in specified format (JSON, TOML, YAML)

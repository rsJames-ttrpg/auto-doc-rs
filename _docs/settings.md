# 📄 File Analysis: `./src/settings.rs`

**Type:** `rs`

## Summary
This file defines configuration structures and loading mechanisms for an autodoc application. It provides settings for file crawling options and LLM configurations, with support for loading from environment variables, configuration files (JSON/TOML/YAML), and writing default configurations.

## 📚 External Dependencies
- `std::fs::File`
- `std::io`
- `std::path::PathBuf`
- `crate::llm_interface::models::ModelId`
- `clap::ValueEnum`
- `config`
- `serde`
- `serde_json`
- `toml`
- `serde_yaml`
- `serial_test`
- `tempfile`

## 🔌 Public Interfaces
- **CrawlOptions** (`📦 Struct`)
  Configuration struct for file crawling behavior including depth limits, hidden file inclusion, pattern matching, and git mode
- **LlmSettings** (`📦 Struct`)
  Configuration struct for LLM settings including model selection, API keys, token limits, temperature, and prompt overrides
- **Settings** (`📦 Struct`)
  Main configuration struct containing file crawling options and multiple LLM settings configurations
- **FileType** (`📦 Struct`)
  Enum defining supported configuration file formats: JSON, TOML, and YAML
- **from_env** (`🔧 Function`)
  Static method to load Settings from environment variables with AUTODOC prefix
- **from_file** (`🔧 Function`)
  Static method to load Settings from a configuration file, supporting multiple formats
- **write_default_config** (`🔧 Function`)
  Static method to write default configuration to a file or stdout in specified format

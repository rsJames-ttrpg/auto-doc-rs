# 📄 File Analysis: `./src/settings.rs`

**Type:** `rs`

## Summary
This file defines the configuration system for the application, providing structures for file crawling options and LLM settings. It supports loading configuration from files (JSON, TOML, YAML) and environment variables with a hierarchical override system, and can generate default configuration files.

## 📚 External Dependencies
- `std::fs::File`
- `std::io`
- `std::path::PathBuf`
- `config`
- `serde`
- `clap::ValueEnum`
- `serde_json`
- `toml`
- `serde_yaml`

## 🔌 Public Interfaces
- **CrawlOptions** (`📦 Struct`)
  Configuration options for file crawling including depth limits, pattern matching, and git mode settings
- **LlmSettings** (`📦 Struct`)
  Configuration for LLM interactions including model selection, API keys, token limits, and temperature settings
- **Settings** (`📦 Struct`)
  Main application settings combining file crawl options and LLM configurations
- **FileType** (`📦 Struct`)
  Enum for supported configuration file formats (Json, Toml, Yaml) with clap ValueEnum support
- **Settings::from_env** (`🔧 Function`)
  Creates Settings from environment variables prefixed with AUTODOC, supporting nested configuration and list parsing
- **Settings::from_file** (`🔧 Function`)
  Loads Settings from a configuration file (JSON, TOML, or YAML) with environment variable override support
- **Settings::write_default_config** (`🔧 Function`)
  Writes default configuration to a file or stdout in the specified format (JSON, TOML, or YAML)

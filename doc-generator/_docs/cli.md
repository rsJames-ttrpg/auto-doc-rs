# 📄 File Analysis: `./src/cli.rs`

**Type:** `rs`

## Summary
This is the main CLI module for an auto-documentation tool that crawls project directories and generates documentation using LLM analysis. It provides commands for crawling files, generating documentation, creating configuration files, and listing supported models.

## 📚 External Dependencies
- `clap`
- `clap_complete`
- `dotenv`
- `indicatif`
- `std::io`
- `std::path::PathBuf`
- `std::time::Duration`
- `strum`
- `tracing`
- `tracing_subscriber`
- `crate::crawler::file`
- `crate::generate`
- `crate::llm_interface::client`
- `crate::llm_interface::models`
- `crate::llm_interface::pool`
- `crate::output::file_system`
- `crate::settings`

## 🔌 Public Interfaces
- **run_application** (`🔧 Function`)
  Main async entry point that parses CLI arguments and executes the appropriate command (crawl, generate, config, or models). Returns a Result indicating success or failure.
- **Cli** (`📦 Struct`)
  Main CLI structure that defines command-line arguments including subcommands, log level, JSON logging flag, config path, and shell completions.
- **Commands** (`📦 Struct`)
  Enum defining available CLI subcommands: Crawl (test file targeting), Generate (create documentation), Config (generate example config), and Models (list supported models).
- **LogLevel** (`📦 Struct`)
  Enum for setting logging verbosity levels (Trace, Debug, Info, Warn, Error) that converts to tracing::Level.

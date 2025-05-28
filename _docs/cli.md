# 📄 File Analysis: `./src/cli.rs`

**Type:** `rs`

## Summary
This file implements the command-line interface for an auto-documentation tool that crawls codebases and generates documentation using LLM analysis. It provides commands for crawling files, generating documentation, creating config files, and listing supported models.

## 📚 External Dependencies
- `clap`
- `dotenv`
- `indicatif`
- `std::path::PathBuf`
- `std::time::Duration`
- `strum`
- `tracing`
- `tracing_subscriber`

## 🔌 Public Interfaces
- **run_application** (`🔧 Function`)
  Main entry point for the CLI application that parses command-line arguments and executes the appropriate command (crawl, generate, config, or models)
- **Cli** (`📦 Struct`)
  Command-line interface structure that defines global options like log level, JSON logging, and config file path
- **Commands** (`📦 Struct`)
  Enum defining available CLI subcommands: Crawl (test file targeting), Generate (create documentation), Config (generate example config), and Models (list supported models)
- **LogLevel** (`📦 Struct`)
  Enum for setting logging verbosity levels from Trace to Error, with conversion to tracing::Level

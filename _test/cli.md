# 📄 File Analysis: `./src/cli.rs`

**Type:** `rs`

## Summary
Command-line interface implementation for the auto-doc tool, providing subcommands for crawling directories, generating documentation, managing configuration, and listing supported models. It handles argument parsing, logging setup, and orchestrates the main application flow including LLM-based code analysis and markdown documentation generation.

## 📚 External Dependencies
- `clap`
- `clap_complete`
- `dotenv`
- `indicatif`
- `strum`
- `tracing`
- `tracing_subscriber`

## 🔌 Public Interfaces
- **run_application** (`🔧 Function`)
  Main entry point for the CLI application that parses command-line arguments, initializes logging, loads configuration, and executes the appropriate subcommand (crawl, generate, config, or models)

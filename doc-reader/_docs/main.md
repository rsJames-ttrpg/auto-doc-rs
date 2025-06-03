# 📄 File Analysis: `./src/main.rs`

**Type:** `rs`

## Summary
Main entry point for a doc-reader CLI application that serves as an MCP (Model Context Protocol) server with documentation reading capabilities. It provides command-line interface for serving the MCP server, generating configuration files, and shell completions.

## 📚 External Dependencies
- `std::io`
- `std::path::PathBuf`
- `std::sync::OnceLock`
- `clap`
- `clap_complete`
- `mcp_core`
- `mcp_core_macros`
- `tracing`
- `tracing_subscriber`
- `tokio`
- `anyhow`

## 🔌 Public Interfaces
- **get_global_settings** (`🔧 Function`)
  Returns a static reference to the global Settings instance, panics if settings are not initialized
- **echo_tool** (`🔧 Function`)
  MCP tool function that echoes back the provided message, used for testing the tool system
- **main** (`🔧 Function`)
  Async main function that handles CLI parsing, initializes tracing and settings, and executes the appropriate command (serve, config, or completions)

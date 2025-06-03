# 📁 Directory: `./src/tools`

**Depth Level:** 2

## Summary
The tools directory implements a documentation retrieval system that provides programmatic access to project component contents. It serves as a bridge between the MCP (Model Context Protocol) framework and the file system, enabling automated extraction and formatting of source code and documentation files with configurable depth traversal and filtering capabilities.

## 🎯 Key Components
- **GetDocumentationTool**
- **get_docs module**
- **File traversal engine**
- **Content formatting system**

## 📋 Child Summaries
1. Module declaration exposing get_docs submodule as the primary interface for tool functionality
2. Complete documentation retrieval implementation with MCP integration, file system traversal, content formatting, and configurable filtering options

## 📚 External Dependencies
- `std::fs`
- `std::path`
- `std::collections::VecDeque`
- `mcp_core`
- `serde_json`
- `tempfile`

## 🔌 Public Interfaces
- **get_docs** (`📁 Module`)
  Primary module exposing documentation retrieval functionality to the rest of the crate
- **GetDocumentationTool** (`📦 Struct`)
  Main tool implementation that orchestrates file discovery, content extraction, and formatting operations
- **tool** (`🔧 Function`)
  Factory function that creates MCP Tool configurations with JSON schema validation for documentation retrieval parameters
- **call** (`🔧 Function`)
  Tool handler that processes requests and returns formatted documentation content as text responses

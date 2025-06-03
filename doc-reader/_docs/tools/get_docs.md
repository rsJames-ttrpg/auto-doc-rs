# 📄 File Analysis: `./src/tools/get_docs.rs`

**Type:** `rs`

## Summary
This file implements a documentation retrieval tool that allows users to extract and format file contents from project components. It provides configurable depth traversal of directory structures and can filter for summary files only, integrating with the MCP (Model Context Protocol) framework for tool execution.

## 📚 External Dependencies
- `std::fs`
- `std::path::Path`
- `std::path::PathBuf`
- `std::collections::VecDeque`
- `mcp_core::tool_text_response`
- `mcp_core::tools::ToolHandlerFn`
- `mcp_core::types::CallToolRequest`
- `mcp_core::types::Tool`
- `serde_json::json`
- `tempfile::TempDir`

## 🔌 Public Interfaces
- **GetDocumentationTool** (`📦 Struct`)
  Main struct that provides documentation retrieval functionality for project components
- **tool** (`🔧 Function`)
  Creates and returns a Tool configuration for the documentation retrieval functionality with JSON schema for component_name, depth, and summary_only parameters
- **call** (`🔧 Function`)
  Returns a ToolHandlerFn that processes documentation retrieval requests, extracts files from component paths up to specified depth, and formats file contents
- **get_files_to_depth** (`🔧 Function`)
  Recursively traverses directory structure up to specified depth using breadth-first search and returns vector of file paths
- **format_file_contents** (`🔧 Function`)
  Takes vector of file paths and formats their contents into markdown with file headers, optional size truncation, and error handling for unreadable files

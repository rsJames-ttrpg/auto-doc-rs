# 📄 File Analysis: `./src/output/file_system.rs`

**Type:** `rs`

## Summary
This file implements a markdown documentation generator that converts code analysis results into structured markdown files. It provides configurable output through MarkdownConfig and MarkdownConfigBuilder, and uses MarkdownGenerator to create documentation files with proper directory structure based on project analysis data.

## 📚 External Dependencies
- `anyhow::Result`
- `std::path::{Path, PathBuf}`
- `tokio::fs`
- `crate::analysis::summary::{ChildAnalysis, DirectoryAnalysis, FileAnalysis, ProjectAnalysis}`

## 🔌 Public Interfaces
- **MarkdownConfig** (`📦 Struct`)
  Configuration struct for markdown generation with output directory, directory index settings, and project root path options
- **MarkdownConfigBuilder** (`📦 Struct`)
  Builder pattern implementation for MarkdownConfig with fluent API methods for configuring markdown generation options
- **MarkdownGenerator** (`📦 Struct`)
  Main generator that converts project analysis data into markdown documentation files with configurable output structure
- **new** (`🔧 Function`)
  Constructor for MarkdownGenerator that takes a MarkdownConfig and returns a configured generator instance
- **generate_documentation** (`🔧 Function`)
  Async function that generates complete markdown documentation from ProjectAnalysis and ChildAnalysis data, creating directory structure and files

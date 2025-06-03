# 📁 Directory: `./src/output`

**Depth Level:** 2

## Summary
The output directory provides markdown documentation generation capabilities that convert code analysis results into structured documentation files. It serves as the final stage of the documentation pipeline, taking analyzed project data and producing human-readable markdown files with configurable output formats and directory structures.

## 🎯 Key Components
- **MarkdownGenerator - Core documentation generation engine**
- **MarkdownConfig - Configuration system for output customization**
- **file_system module - File system operations and markdown creation**

## 📋 Child Summaries
1. file_system.rs: Implements markdown documentation generator with configurable output through MarkdownConfig and MarkdownGenerator, converting code analysis results into structured markdown files
2. mod.rs: Module declaration file exposing file_system submodule as public interface for organizational structure

## 📚 External Dependencies
- `anyhow::Result`
- `std::path::{Path, PathBuf}`
- `tokio::fs`
- `crate::analysis::summary::{ChildAnalysis, DirectoryAnalysis, FileAnalysis, ProjectAnalysis}`

## 🔌 Public Interfaces
- **MarkdownGenerator** (`📦 Struct`)
  Main interface for converting project analysis data into markdown documentation with configurable output structure and formatting
- **MarkdownConfig** (`📦 Struct`)
  Configuration interface that allows customization of output directory, directory indexing, and project root path settings
- **generate_documentation** (`🔧 Function`)
  Primary async interface for generating complete markdown documentation from analysis results, handling file creation and directory structure
- **file_system** (`📁 Module`)
  Module interface providing file system operations and markdown generation functionality to other system components

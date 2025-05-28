# 📄 File Analysis: `./src/output/file_system.rs`

**Type:** `rs`

## Summary
This file implements a markdown documentation generator that converts project analysis data into structured markdown files. It provides configuration options for output directory structure and generates documentation files for projects, directories, and individual files using their Display implementations.

## 📚 External Dependencies
- `anyhow::Result`
- `config::File`
- `std::path::{Path, PathBuf}`
- `tokio::fs`

## 🔌 Public Interfaces
- **MarkdownConfig** (`📦 Struct`)
  Configuration struct for markdown generation with output directory, directory index settings, and project root path options
- **MarkdownConfigBuilder** (`📦 Struct`)
  Builder pattern implementation for constructing MarkdownConfig with fluent API methods for setting various configuration options
- **MarkdownGenerator** (`📦 Struct`)
  Main generator struct that converts project analysis data into markdown documentation files with configurable output structure
- **new** (`🔧 Function`)
  Constructor for MarkdownConfigBuilder that creates a new builder with default values
- **output_dir** (`🔧 Function`)
  Builder method to set the output directory where markdown files will be written
- **create_directory_indices** (`🔧 Function`)
  Builder method to configure whether directory index files should be created
- **no_directory_indices** (`🔧 Function`)
  Convenience builder method to disable directory index file creation
- **directory_index_name** (`🔧 Function`)
  Builder method to set the filename for directory index files
- **use_index_md** (`🔧 Function`)
  Convenience builder method to use 'index.md' instead of 'README.md' for directory indices
- **project_root** (`🔧 Function`)
  Builder method to set the project root path for creating relative file paths in output structure
- **no_project_root** (`🔧 Function`)
  Builder method to clear the project root setting
- **build** (`🔧 Function`)
  Builder method that constructs and returns the final MarkdownConfig instance
- **builder** (`🔧 Function`)
  Static method on MarkdownConfig that creates a new MarkdownConfigBuilder instance
- **new** (`🔧 Function`)
  Constructor for MarkdownGenerator that takes a MarkdownConfig and creates a new generator instance
- **generate_documentation** (`🔧 Function`)
  Async method that generates complete markdown documentation from ProjectAnalysis and child analyses, creating all necessary files and directories

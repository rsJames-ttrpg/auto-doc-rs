# 📁 Directory: `./src/output`

**Depth Level:** 2

## Summary
The output directory serves as the documentation generation subsystem, responsible for converting analyzed project data into structured markdown documentation. It provides a configurable markdown generator that can create comprehensive documentation files with customizable directory structures, file naming conventions, and output formats. This directory acts as the final stage in the documentation pipeline, transforming internal analysis representations into human-readable markdown files that can be consumed by documentation systems or version control platforms.

## 🎯 Key Components
- **MarkdownGenerator**
- **MarkdownConfig**
- **MarkdownConfigBuilder**
- **file_system module**

## 📋 Child Summaries
1. file_system.rs implements a markdown documentation generator with configurable output structure, converting project analysis data into structured markdown files
2. mod.rs serves as the module entry point, exposing file_system functionality to other parts of the codebase

## 📚 External Dependencies
- `anyhow::Result`
- `config::File`
- `std::path::{Path, PathBuf}`
- `tokio::fs`

## 🔌 Public Interfaces
- **MarkdownGenerator** (`📦 Struct`)
  Primary interface for converting project analysis data into structured markdown documentation with configurable output formats
- **MarkdownConfig** (`📦 Struct`)
  Configuration interface for customizing markdown generation behavior including output directory structure and file naming conventions
- **generate_documentation** (`🔧 Function`)
  Main entry point for generating complete markdown documentation from project analysis data, creating all necessary files and directories asynchronously
- **file_system** (`📁 Module`)
  Module interface providing file system operations and markdown generation capabilities for documentation output

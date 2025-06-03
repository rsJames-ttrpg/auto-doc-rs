# 📄 File Analysis: `./src/generate/mod.rs`

**Type:** `rs`

## Summary
This module provides a comprehensive analysis crawler that combines file system traversal with LLM-based code analysis. It crawls project directories, filters analyzable files based on extension and size constraints, and orchestrates analysis of individual files, directories, and entire projects using configurable LLM analyzers.

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `tracing::{debug, error, warn}`
- `thiserror::Error`
- `tokio::task::JoinError`
- `crate::analysis::summary`
- `crate::crawler::file`
- `mockall::mock`
- `tempfile::TempDir`
- `async_trait::async_trait`

## 🔌 Public Interfaces
- **AnalysisCrawlOptions** (`📦 Struct`)
  Configuration struct for analysis crawling operations, containing file system crawling options, analysis context, analyzable file extensions, and maximum file size limits
- **AnalysisCrawlError** (`📦 Struct`)
  Error enum for analysis crawling operations, wrapping crawl errors, analysis errors, IO errors, and async join errors
- **AnalysisCrawler** (`📦 Struct`)
  Main crawler struct that orchestrates file system crawling and LLM-based code analysis for entire project directories
- **analyze_project** (`🔧 Function`)
  Async function that crawls a project directory and performs comprehensive analysis, returning project analysis and child analyses
- **analyze_file_tree** (`🔧 Function`)
  Recursive async function that analyzes a file tree node and all its children, handling both files and directories
- **preview_analysis** (`🔧 Function`)
  Function that provides a preview of what would be analyzed without actually performing the analysis, useful for planning and validation
- **AnalysisPreview** (`📦 Struct`)
  Summary struct containing statistics about files to be analyzed, including counts, sizes, file types, and oversized files

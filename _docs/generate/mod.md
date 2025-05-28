# 📄 File Analysis: `./src/generate/mod.rs`

**Type:** `rs`

## Summary
This module provides functionality for crawling and analyzing project directories using LLM-based analysis. It combines file system crawling with concurrent LLM analysis to generate comprehensive project documentation and summaries.

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `tokio::task::JoinSet`
- `tracing::{debug, info}`
- `thiserror::Error`
- `crate::analysis::summary`
- `crate::crawler::file`

## 🔌 Public Interfaces
- **AnalysisCrawlOptions** (`📦 Struct`)
  Configuration struct for analysis crawling operations including file system options, analysis context, concurrency limits, and file filtering criteria
- **AnalysisCrawlError** (`📦 Struct`)
  Error enum for analysis crawling operations that wraps crawl errors, analysis errors, IO errors, and join errors
- **AnalysisCrawler** (`📦 Struct`)
  Main crawler struct that orchestrates directory crawling and LLM-based analysis of files and directories
- **AnalysisPreview** (`📦 Struct`)
  Summary statistics struct showing what files would be analyzed including counts, sizes, file types, and oversized files
- **analyze_project** (`🔧 Function`)
  Async function that crawls a project directory and performs LLM analysis on all eligible files, returning project analysis and child analyses
- **analyze_file_tree** (`🔧 Function`)
  Recursive async function that analyzes a file tree node and all its children, handling both files and directories
- **preview_analysis** (`🔧 Function`)
  Function that provides a preview of what would be analyzed without actually performing the analysis, useful for planning and validation
- **print_summary** (`🔧 Function`)
  Method on AnalysisPreview that prints a formatted summary of analysis statistics to stdout

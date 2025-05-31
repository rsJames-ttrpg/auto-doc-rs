# 📁 Directory: `./src/generate`

**Depth Level:** 2

## Summary
The generate directory provides the core orchestration layer for automated code analysis, combining file system crawling with LLM-based analysis to process entire project directories. It serves as the main entry point for comprehensive project analysis workflows, handling file discovery, filtering, and coordinated analysis of both individual files and directory structures.

## 🎯 Key Components
- **AnalysisCrawler - Main orchestration component for project-wide analysis**
- **AnalysisCrawlOptions - Configuration system for analysis parameters**
- **Project analysis workflow - End-to-end analysis pipeline**
- **File tree analysis - Recursive directory structure analysis**
- **Analysis preview system - Pre-analysis validation and planning**

## 📋 Child Summaries
1. mod.rs: Comprehensive analysis crawler that orchestrates file system traversal with LLM-based code analysis, providing project-wide analysis capabilities with configurable filtering and preview functionality

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
- **analyze_project** (`🔧 Function`)
  Primary entry point for analyzing entire project directories, orchestrating crawling and LLM analysis to produce comprehensive project insights
- **AnalysisCrawler** (`📦 Struct`)
  Main orchestration component that coordinates file system traversal with LLM-based code analysis for project-wide analysis workflows
- **AnalysisCrawlOptions** (`⚙️ Configuration`)
  Configuration interface for customizing analysis behavior including file filtering, size limits, and analysis context parameters
- **preview_analysis** (`🔧 Function`)
  Analysis planning interface that provides statistics and validation before executing full analysis, enabling cost estimation and workflow optimization

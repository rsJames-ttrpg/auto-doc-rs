# 📁 Directory: `./src/generate`

**Depth Level:** 2

## Summary
The generate module provides the core orchestration layer for LLM-powered project analysis, combining file system crawling with concurrent AI analysis to produce comprehensive project documentation. It serves as the main entry point for analyzing entire project directories, managing the workflow from initial file discovery through final analysis output.

## 🎯 Key Components
- **AnalysisCrawler - Main orchestration engine**
- **AnalysisCrawlOptions - Configuration management**
- **analyze_project - Primary analysis workflow**
- **AnalysisPreview - Analysis planning and validation**

## 📋 Child Summaries
1. Module providing LLM-based project analysis orchestration with concurrent file processing, analysis preview capabilities, and comprehensive error handling for directory crawling and AI analysis workflows

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs`
- `std::path`
- `tokio::task::JoinSet`
- `tracing`
- `thiserror::Error`
- `crate::analysis::summary`
- `crate::crawler::file`

## 🔌 Public Interfaces
- **analyze_project** (`🔧 Function`)
  Primary entry point for project-wide LLM analysis that crawls directories and analyzes all eligible files concurrently
- **AnalysisCrawler** (`📦 Struct`)
  Main orchestration engine that coordinates file system crawling with LLM-based analysis workflows
- **AnalysisCrawlOptions** (`📦 Struct`)
  Configuration interface for controlling analysis behavior including concurrency, filtering, and analysis context
- **preview_analysis** (`🔧 Function`)
  Analysis planning interface that provides statistics and validation before executing expensive LLM operations

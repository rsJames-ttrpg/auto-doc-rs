# 📁 Directory: `./src/analysis`

**Depth Level:** 1

## Summary
This directory implements a comprehensive LLM-powered code analysis system that provides hierarchical documentation generation from individual files up to executive-level project summaries. It serves as the core analysis engine that transforms raw code into structured, human-readable documentation through template-driven prompt engineering and standardized data models.

## 🎯 Key Components
- **summary.rs**
- **prompt/**
- **display.rs**
- **mod.rs**

## 📋 Child Summaries
1. Display trait implementations providing formatted markdown output for all analysis data structures with emoji icons and structured presentation
2. Core data structures and LlmAnalyzer trait defining the analysis framework with FileAnalysis, DirectoryAnalysis, and ProjectAnalysis types plus error handling
3. Module organization file exposing display, prompt, and summary submodules as public interfaces
4. Template-driven prompt engineering system providing structured LLM instructions for multi-level code analysis with context injection capabilities

## 📚 External Dependencies
- `async_trait`
- `schemars`
- `serde`
- `std::fmt`
- `std::path`
- `thiserror`
- `crate::llm_interface::LlmClient`

## 🔌 Public Interfaces
- **LlmAnalyzer** (`🎯 Trait`)
  Primary async trait for coordinating LLM-based analysis of files, directories, and projects with structured JSON output
- **FileAnalysis** (`📦 Struct`)
  Structured analysis results for individual files including dependencies, interfaces, and summaries
- **DirectoryAnalysis** (`📦 Struct`)
  Hierarchical analysis results for directories with consolidated child summaries and architectural patterns
- **ProjectAnalysis** (`📦 Struct`)
  Executive-level project documentation including architecture overview, technologies, and business considerations
- **PromptTemplates** (`📦 Struct`)
  Template engine for generating context-specific LLM prompts across different analysis abstraction levels
- **AnalysisContext** (`📦 Struct`)
  Configuration structure controlling project type, target audience, and analysis depth parameters
- **Display Implementations** (`🎯 Trait`)
  Formatted output capabilities for all analysis data structures with markdown and emoji presentation

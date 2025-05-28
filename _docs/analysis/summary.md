# 📄 File Analysis: `./src/analysis/summary.rs`

**Type:** `rs`

## Summary
This file defines the core data structures and traits for a hierarchical code analysis system that uses LLMs to analyze files, directories, and entire projects. It provides structured analysis types (FileAnalysis, DirectoryAnalysis, ProjectAnalysis) and implements an LlmAnalyzer trait that coordinates with LLM clients to perform automated code documentation and summarization.

## 📚 External Dependencies
- `async_trait`
- `schemars`
- `serde`
- `std::fmt`
- `std::path`
- `thiserror`
- `crate::analysis::prompt::PromptTemplates`
- `crate::llm_interface::LlmClient`

## 🔌 Public Interfaces
- **FileAnalysis** (`📦 Struct`)
  Data structure containing analysis results for a single file including path, type, summary, dependencies, and public interfaces
- **DirectoryAnalysis** (`📦 Struct`)
  Data structure containing analysis results for a directory including path, depth, summary, child summaries, key components, and consolidated dependencies
- **ProjectAnalysis** (`📦 Struct`)
  Data structure containing high-level project analysis including overview, architecture summary, core technologies, main interfaces, development considerations, extension points, and risk factors
- **Interface** (`📦 Struct`)
  Data structure representing a code interface with name, type category, and description of functionality
- **InterfaceType** (`📦 Struct`)
  Enum categorizing different types of interfaces (Function, Struct, Trait, Module, Api, Configuration, DataModel)
- **AnalysisError** (`📦 Struct`)
  Error type for analysis operations with variants for LLM communication failures and parsing errors
- **LlmAnalyzer** (`🎯 Trait`)
  Async trait defining methods for analyzing files, directories, and projects using LLM clients with structured output
- **AnalysisContext** (`📦 Struct`)
  Configuration structure containing project type, target audience, and analysis depth settings
- **ProjectType** (`📦 Struct`)
  Enum categorizing different types of software projects (WebApplication, Library, CliTool, SystemService, DeveloperTool, Unknown)
- **AnalysisAudience** (`📦 Struct`)
  Enum specifying the intended audience for analysis results (LlmConsumption, HumanDeveloper, TechnicalDocumentation)
- **AnalysisDepth** (`📦 Struct`)
  Enum controlling the depth of analysis performed (Surface, Standard, Deep)
- **ChildAnalysis** (`📦 Struct`)
  Tagged union enum containing either FileAnalysis or DirectoryAnalysis for hierarchical analysis aggregation

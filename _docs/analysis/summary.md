# 📄 File Analysis: `./src/analysis/summary.rs`

**Type:** `rs`

## Summary
This file defines the core data structures and traits for a code analysis system that uses LLMs to analyze files, directories, and entire projects. It provides structured schemas for capturing analysis results at different levels of granularity and defines the main LlmAnalyzer trait for performing async analysis operations.

## 📚 External Dependencies
- `async_trait`
- `schemars`
- `serde`
- `thiserror`
- `std::fmt`
- `std::path`

## 🔌 Public Interfaces
- **FileAnalysis** (`📦 Struct`)
  Data structure representing analysis results for a single file, including file path, type, summary, dependencies, and public interfaces
- **DirectoryAnalysis** (`📦 Struct`)
  Data structure representing analysis results for a directory, including path, depth, summary, child summaries, key components, and consolidated dependencies
- **ProjectAnalysis** (`📦 Struct`)
  Data structure representing high-level project analysis including overview, architecture summary, core technologies, main interfaces, and risk factors
- **Interface** (`📦 Struct`)
  Data structure representing a public interface with name, type classification, and description of functionality
- **InterfaceType** (`🗄️ Data Model`)
  Enum categorizing different types of interfaces (Function, Struct, Trait, Module, Api, Configuration, DataModel)
- **AnalysisError** (`🗄️ Data Model`)
  Error enum for analysis operations with LLM communication and parsing error variants
- **LlmAnalyzer** (`🎯 Trait`)
  Main async trait defining methods for analyzing files, directories, and projects using LLM services
- **AnalysisContext** (`📦 Struct`)
  Configuration structure specifying project type, target audience, and analysis depth for LLM analysis operations
- **ProjectType** (`🗄️ Data Model`)
  Enum categorizing different types of software projects (WebApplication, Library, CliTool, etc.)
- **AnalysisAudience** (`🗄️ Data Model`)
  Enum specifying the intended audience for analysis results (LlmConsumption, HumanDeveloper, TechnicalDocumentation)
- **AnalysisDepth** (`🗄️ Data Model`)
  Enum controlling the depth of analysis performed (Surface, Standard, Deep)
- **ChildAnalysis** (`🗄️ Data Model`)
  Tagged union enum that can hold either FileAnalysis or DirectoryAnalysis results for hierarchical analysis processing

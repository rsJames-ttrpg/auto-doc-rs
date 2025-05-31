# 📁 Directory: `./src/analysis`

**Depth Level:** 2

## Summary
This directory implements a comprehensive LLM-powered code analysis and documentation generation system. It provides structured analysis capabilities at multiple architectural levels (file, directory, project) with template-driven prompt generation, formatted output display, and strongly-typed data models for capturing analysis results.

## 🎯 Key Components
- **LlmAnalyzer trait**
- **Analysis data structures (FileAnalysis, DirectoryAnalysis, ProjectAnalysis)**
- **PromptTemplates system**
- **Display formatting implementations**
- **Template-based analysis frameworks**

## 📋 Child Summaries
1. Display trait implementations for formatted output of analysis results with emoji-enhanced presentation
2. Module organization exposing display, prompt, and summary functionality as public interfaces
3. Core data structures and traits defining the analysis system architecture with LLM integration capabilities
4. Template-driven prompt generation system for creating context-aware analysis prompts at multiple architectural levels
5. Template configurations defining structured analysis frameworks for automated code documentation generation

## 📚 External Dependencies
- `async_trait`
- `schemars`
- `serde`
- `thiserror`
- `std::fmt`
- `std::path`

## 🔌 Public Interfaces
- **LlmAnalyzer** (`🎯 Trait`)
  Main async trait defining methods for analyzing files, directories, and projects using LLM services with structured output schemas
- **FileAnalysis** (`🗄️ Data Model`)
  Structured data model capturing comprehensive file-level analysis including dependencies, interfaces, and architectural context
- **DirectoryAnalysis** (`🗄️ Data Model`)
  Hierarchical analysis model that synthesizes child components into directory-level architectural understanding
- **ProjectAnalysis** (`🗄️ Data Model`)
  Executive-level project analysis model capturing business value, technical architecture, and system-wide characteristics
- **PromptTemplates** (`📦 Struct`)
  Template management system providing context-aware prompt generation for multi-level code analysis operations
- **AnalysisContext** (`⚙️ Configuration`)
  Configuration structure specifying project type, target audience, and analysis depth for customizing LLM analysis behavior

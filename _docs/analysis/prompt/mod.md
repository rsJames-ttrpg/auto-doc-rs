# 📄 File Analysis: `./src/analysis/prompt/mod.rs`

**Type:** `rs`

## Summary
This module provides prompt template management for code analysis, containing a PromptTemplates struct that loads template files and builds customized prompts for file analysis, directory synthesis, and project analysis. It serves as a template engine that injects context-specific information into predefined prompt templates.

## 📚 External Dependencies
- `std::path::Path`
- `super::summary::AnalysisContext`
- `templates/file_analysis.txt`
- `templates/directory_analysis.txt`
- `templates/project_analysis.txt`

## 🔌 Public Interfaces
- **PromptTemplates** (`📦 Struct`)
  Main struct containing three template strings for different types of analysis prompts with methods to build customized prompts
- **build_file_analysis_prompt** (`🔧 Function`)
  Takes a file path and analysis context to generate a customized file analysis prompt by replacing template placeholders
- **build_directory_synthesis_prompt** (`🔧 Function`)
  Takes a directory path and analysis context to generate a customized directory synthesis prompt
- **build_project_analysis_prompt** (`🔧 Function`)
  Takes a project root path and analysis context to generate a customized project analysis prompt

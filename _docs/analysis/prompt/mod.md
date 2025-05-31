# 📄 File Analysis: `./src/analysis/prompt/mod.rs`

**Type:** `rs`

## Summary
This module provides a prompt template system for code analysis operations. It defines the PromptTemplates struct that loads template files and provides methods to generate customized prompts for file, directory, and project-level analysis by substituting context-specific values into predefined templates.

## 📚 External Dependencies
- `std::path::Path`
- `super::summary::AnalysisContext`
- `include_str! macro`
- `templates/file_analysis.txt`
- `templates/directory_analysis.txt`
- `templates/project_analysis.txt`

## 🔌 Public Interfaces
- **PromptTemplates** (`📦 Struct`)
  A struct containing template strings for different types of analysis prompts (file, directory, and project analysis). Provides methods to build customized prompts by replacing placeholders with context-specific values.
- **build_file_analysis_prompt** (`🔧 Function`)
  Creates a customized file analysis prompt by replacing placeholders in the template with the provided file path and analysis context information.
- **build_directory_synthesis_prompt** (`🔧 Function`)
  Creates a customized directory synthesis prompt by replacing placeholders with the directory path and project type from the analysis context.
- **build_project_analysis_prompt** (`🔧 Function`)
  Creates a customized project analysis prompt by replacing placeholders with the project root path and project type from the analysis context.

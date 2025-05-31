# 📁 Directory: `./src/analysis/prompt`

**Depth Level:** 3

## Summary
This directory implements a template-driven prompt generation system for automated code documentation. It provides a structured framework for creating context-aware prompts that guide LLMs through multi-level code analysis, from individual files to entire projects, ensuring consistent documentation quality and architectural understanding across different scales of analysis.

## 🎯 Key Components
- **PromptTemplates struct**
- **Template substitution system**
- **Multi-level analysis templates**
- **Context-aware prompt generation**

## 📋 Child Summaries
1. Template configurations that define structured analysis frameworks for automated code documentation generation at file, directory, and project levels
2. Rust module providing a prompt template system that loads template files and generates customized prompts by substituting context-specific values into predefined templates

## 📚 External Dependencies
- `std::path::Path`
- `super::summary::AnalysisContext`
- `include_str! macro`

## 🔌 Public Interfaces
- **PromptTemplates** (`📦 Struct`)
  Central prompt template management system that loads analysis templates and provides methods to generate customized prompts for different analysis levels by substituting context-specific values
- **build_file_analysis_prompt** (`🔧 Function`)
  Generates context-aware prompts for individual file analysis by combining template structure with specific file path and analysis context information
- **build_directory_synthesis_prompt** (`🔧 Function`)
  Creates directory-level analysis prompts that guide synthesis of child components into cohesive architectural documentation
- **build_project_analysis_prompt** (`🔧 Function`)
  Produces executive-level project analysis prompts that focus on business value, technical architecture, and system-wide characteristics

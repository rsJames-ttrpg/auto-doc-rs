# 📁 Directory: `./src/analysis/prompt`

**Depth Level:** 2

## Summary
This directory implements a template-driven prompt engineering system for multi-level code analysis. It provides structured LLM prompts that enable hierarchical documentation generation from individual files up to executive-level project summaries, serving as the core prompt management infrastructure for the analysis system.

## 🎯 Key Components
- **mod.rs**
- **templates/**

## 📋 Child Summaries
1. Template management module providing PromptTemplates struct with methods to build customized analysis prompts by injecting context into predefined templates
2. Template collection containing structured LLM instructions for file-level code analysis, directory synthesis, and project-level documentation generation with standardized variable substitution

## 📚 External Dependencies
- `std::path::Path`
- `super::summary::AnalysisContext`

## 🔌 Public Interfaces
- **PromptTemplates** (`📦 Struct`)
  Central template engine that loads and manages analysis prompt templates, providing methods to generate context-specific prompts for different analysis levels
- **build_file_analysis_prompt** (`🔧 Function`)
  Generates customized file analysis prompts by injecting file path and context into template structure
- **build_directory_synthesis_prompt** (`🔧 Function`)
  Creates directory-level synthesis prompts for aggregating child component analysis into architectural summaries
- **build_project_analysis_prompt** (`🔧 Function`)
  Produces executive-level project analysis prompts for generating comprehensive project documentation
- **Multi-Level Template System** (`⚙️ Configuration`)
  Hierarchical prompt template architecture supporting consistent analysis methodology across file, directory, and project abstraction levels

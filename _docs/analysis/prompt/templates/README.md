# 📁 Directory: `./src/analysis/prompt/templates`

**Depth Level:** 3

## Summary
This directory contains template files that define the structured prompts and guidelines for LLM-based code analysis at different levels of abstraction. It serves as the prompt engineering foundation for the analysis system, providing standardized instructions for generating documentation from file-level code analysis up to executive-level project summaries.

## 🎯 Key Components
- **file_analysis.txt**
- **directory_analysis.txt**
- **project_analysis.txt**

## 📋 Child Summaries
1. Template for generating executive-level project documentation with business value, technical architecture, and operational guidelines
2. Template for synthesizing directory-level architectural summaries from child components with focus on system integration patterns
3. Template for LLM-based file analysis defining extraction of dependencies, interfaces, and code summaries

## 🔌 Public Interfaces
- **Multi-Level Analysis Template System** (`⚙️ Configuration`)
  Hierarchical template structure supporting file, directory, and project-level analysis with consistent variable substitution and formatting guidelines
- **Template Variable Framework** (`⚙️ Configuration`)
  Standardized placeholder system for dynamic content injection including FILE_PATH, PROJECT_TYPE, DIRECTORY_PATH, ANALYSIS_DEPTH, and TARGET_AUDIENCE
- **Architectural Analysis Guidelines** (`🗄️ Data Model`)
  Structured methodology for extracting and documenting software architecture patterns, dependencies, interfaces, and system integration points across multiple abstraction levels

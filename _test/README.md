# 🚀 Project Analysis

## Overview
Auto-doc is an automated documentation generation tool that leverages Large Language Models (LLMs) to analyze source code and produce comprehensive markdown documentation. The tool crawls through directory structures, analyzes code files using AI, and generates human-readable documentation that captures the purpose, structure, and interfaces of software projects.

## 🏗️ Architecture
The system follows a modular architecture with clear separation of concerns. The CLI module serves as the entry point, orchestrating the workflow through distinct phases: file crawling (crawler module), code analysis (analysis module with LLM integration), and documentation generation (generate and output modules). Configuration is managed through a flexible settings system supporting multiple formats and environment variables. The asynchronous design using Tokio enables efficient processing of large codebases.

## 🛠️ Core Technologies
- **Rust programming language**
- **Tokio async runtime for concurrent processing**
- **Clap for command-line interface parsing**
- **Serde for configuration serialization/deserialization**
- **Tracing for structured logging**
- **LLM integration for AI-powered code analysis**
- **Support for multiple configuration formats (JSON, TOML, YAML)**

## 🔌 Main Interfaces
- **run_application** (`🔧 Function`)
  Primary CLI entry point that handles command parsing, logging initialization, and subcommand execution for crawling, generating docs, and managing configuration
- **Settings** (`📦 Struct`)
  Central configuration structure that manages all application settings including crawl options and LLM parameters, with support for file and environment-based configuration
- **llm_interface** (`📁 Module`)
  Module providing abstraction layer for interacting with various Large Language Model providers for code analysis
- **crawler** (`📁 Module`)
  File system traversal module that discovers and filters source code files based on configurable patterns and depth limits

## 🔧 Development Considerations
- Requires Rust toolchain and Cargo for building
- Needs valid LLM API credentials configured via environment or config files
- Async runtime (Tokio) required for execution
- Configuration can be provided through multiple sources with hierarchical override
- Supports git-aware file discovery mode

## 🔗 Extension Points
- LLM provider integration - new AI models can be added to the llm_interface module
- File type support - crawler patterns can be extended for additional programming languages
- Output format generation - new documentation formats beyond markdown can be added
- Configuration formats - additional config file types can be supported
- Analysis strategies - new code analysis approaches can be implemented in the analysis module

## ⚠️ Risk Factors
- **LLM API Dependencies** (`🌐 API`)
  Reliance on external LLM services creates availability and cost risks, requiring API key management and rate limiting considerations
- **Token Limits** (`⚙️ Configuration`)
  LLM token constraints may limit analysis of very large files or projects, requiring chunking strategies
- **File System Performance** (`📁 Module`)
  Large directory traversals could impact performance, especially with deep recursion or broad file patterns

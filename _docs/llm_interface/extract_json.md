# 📄 File Analysis: `./src/llm_interface/extract_json.rs`

**Type:** `rs`

## Summary
This module provides utilities for extracting JSON content from LLM response text using regex patterns and brace balancing. It handles common LLM output formats including JSON wrapped in code blocks and standalone JSON objects embedded in natural language responses.

## 📚 External Dependencies
- `once_cell::sync::Lazy`
- `regex::Regex`

## 🔌 Public Interfaces
- **extract_json_from_response** (`🔧 Function`)
  Attempts to extract valid JSON from LLM response text by first looking for JSON in code blocks, then for standalone JSON objects
- **extract_json_aggressively** (`🔧 Function`)
  Extracts all possible JSON candidates from text using multiple strategies including code blocks, regex matching, and balanced brace counting

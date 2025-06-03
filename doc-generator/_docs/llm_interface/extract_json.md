# 📄 File Analysis: `./src/llm_interface/extract_json.rs`

**Type:** `rs`

## Summary
This module provides utilities for extracting JSON content from LLM responses that may contain mixed text and JSON. It uses regex patterns to identify JSON in code blocks or raw text, with both conservative and aggressive extraction strategies.

## 📚 External Dependencies
- `once_cell::sync::Lazy`
- `regex::Regex`

## 🔌 Public Interfaces
- **extract_json_from_response** (`🔧 Function`)
  Attempts to extract the first valid JSON object from LLM response text, trying code blocks first then raw JSON objects
- **extract_json_aggressively** (`🔧 Function`)
  Extracts all possible JSON objects from text using multiple strategies including code blocks, regex matching, and balanced brace counting

# 📁 Directory: `./src/crawler`

**Depth Level:** 2

## Summary
File system crawler module that provides recursive directory traversal capabilities with configurable filtering and tree-based representation. This module serves as the core crawling engine for the system, enabling structured exploration of file systems with support for pattern matching, depth control, and metadata collection.

## 🎯 Key Components
- **file.rs - Core crawler implementation with FileNode tree structure**
- **mod.rs - Module entry point exposing file crawler functionality**

## 📋 Child Summaries
1. Module declaration exposing file submodule as public interface
2. Complete file system crawler implementation with tree-based representation, configurable filtering, and iterator support for traversing directory structures

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs`
- `std::path`
- `serde`
- `thiserror`
- `glob`
- `tempfile`

## 🔌 Public Interfaces
- **crawl_directory** (`🔧 Function`)
  Primary entry point for crawling directory structures with configurable options, returning a hierarchical FileNode tree representation
- **FileNode** (`📦 Struct`)
  Tree-based data structure representing files and directories with metadata, supporting serialization and iteration
- **CrawlOptions** (`⚙️ Configuration`)
  Configuration interface for controlling crawl behavior including depth limits, pattern filtering, symlink handling, and hidden file inclusion
- **FileNodeIterator** (`📦 Struct`)
  Iterator interface for depth-first traversal of the file tree structure with optional depth tracking

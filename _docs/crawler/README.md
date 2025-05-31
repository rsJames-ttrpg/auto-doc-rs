# 📁 Directory: `./src/crawler`

**Depth Level:** 2

## Summary
The crawler directory implements a comprehensive file system traversal and analysis system that builds tree representations of directory structures with advanced filtering and iteration capabilities. It serves as the core component for scanning and organizing file system data within the larger application.

## 🎯 Key Components
- **file.rs - Core file system crawler implementation**
- **mod.rs - Module entry point and public interface exposure**

## 📋 Child Summaries
1. file.rs: Implements file system crawler with tree building, glob pattern filtering, and iterator support for traversing directory structures
2. mod.rs: Module declaration exposing file submodule as public interface for crawler functionality

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `std::env`
- `serde::{Deserialize, Serialize}`
- `thiserror::Error`
- `glob`
- `tempfile`

## 🔌 Public Interfaces
- **FileNode** (`🗄️ Data Model`)
  Primary data structure representing file system nodes in a tree hierarchy with metadata and traversal capabilities
- **crawl_directory** (`🔧 Function`)
  Main entry point for initiating file system crawls with configurable filtering and depth control
- **CrawlOptions** (`⚙️ Configuration`)
  Configuration interface for customizing crawler behavior including pattern matching, depth limits, and file filtering
- **FileNodeIterator** (`📦 Struct`)
  Iterator interface providing depth-first traversal over crawled file system trees
- **CrawlResult** (`🗄️ Data Model`)
  Result type encapsulating successful crawl outcomes or error conditions

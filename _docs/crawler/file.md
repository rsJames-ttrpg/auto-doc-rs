# 📄 File Analysis: `./src/crawler/file.rs`

**Type:** `rs`

## Summary
This file implements a file system crawler that recursively traverses directory structures and builds a tree representation of files and directories. It provides configurable crawling options including depth limits, glob pattern filtering, symlink handling, and hidden file inclusion.

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `serde::{Deserialize, Serialize}`
- `thiserror::Error`
- `glob`
- `tempfile`

## 🔌 Public Interfaces
- **FileNode** (`📦 Struct`)
  Enum representing either a file or directory node in the file system tree with metadata like size, path, and children
- **CrawlOptions** (`📦 Struct`)
  Configuration struct for customizing crawl behavior including max depth, symlink following, hidden files, and glob patterns
- **CrawlResult** (`🗄️ Data Model`)
  Type alias for Result<FileNode, CrawlError> representing the outcome of a crawl operation
- **CrawlError** (`📦 Struct`)
  Error enum for various crawl failures including IO errors, glob pattern errors, and path not found
- **crawl_directory** (`🔧 Function`)
  Main entry point function that crawls a directory structure with given options and returns a FileNode tree
- **FileNodeIterator** (`📦 Struct`)
  Iterator for traversing all nodes in the file tree using depth-first traversal
- **FileNodeDepthIterator** (`📦 Struct`)
  Iterator for traversing all nodes with their depth level information

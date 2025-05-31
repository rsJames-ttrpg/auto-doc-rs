# 📄 File Analysis: `./src/crawler/file.rs`

**Type:** `rs`

## Summary
This file implements a file system crawler that builds a tree representation of directories and files with filtering capabilities. It provides the core FileNode enum for representing file system structures, crawling functionality with glob pattern support, and iterator implementations for traversing the resulting tree structures.

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `serde::{Deserialize, Serialize}`
- `thiserror::Error`
- `glob`
- `std::env`
- `tempfile (test dependency)`

## 🔌 Public Interfaces
- **FileNode** (`🗄️ Data Model`)
  Enum representing either a file or directory node in a file system tree, with metadata like size, path, and children for directories
- **CrawlOptions** (`⚙️ Configuration`)
  Configuration struct for controlling file system crawling behavior including depth limits, symlink following, hidden files, and glob patterns
- **CrawlResult** (`🗄️ Data Model`)
  Type alias for Result<FileNode, CrawlError> representing the outcome of a crawl operation
- **CrawlError** (`🗄️ Data Model`)
  Error enum for various crawling failures including IO errors, glob pattern errors, and path not found errors
- **crawl_directory** (`🔧 Function`)
  Main public function that crawls a directory structure with optional filtering and returns a FileNode tree representation
- **FileNodeIterator** (`📦 Struct`)
  Iterator that provides depth-first traversal over all nodes in a FileNode tree
- **FileNodeDepthIterator** (`📦 Struct`)
  Iterator that provides depth-first traversal over FileNode tree with depth level information for each node

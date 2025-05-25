use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileNode {
    File {
        name: String,
        path: PathBuf,
        size: u64,
        extension: Option<String>,
    },
    Directory {
        name: String,
        path: PathBuf,
        children: HashMap<String, FileNode>,
        total_size: u64,
    },
}

impl FileNode {
    pub fn name(&self) -> &str {
        match self {
            FileNode::File { name, .. } => name,
            FileNode::Directory { name, .. } => name,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            FileNode::File { path, .. } => path,
            FileNode::Directory { path, .. } => path,
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FileNode::File { .. })
    }

    #[allow(dead_code)]
    pub fn is_directory(&self) -> bool {
        matches!(self, FileNode::Directory { .. })
    }
}

#[derive(Debug, Clone, Default)]
pub struct CrawlOptions {
    pub max_depth: Option<usize>,
    pub follow_symlinks: bool,
    pub include_hidden: bool,
    pub glob_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

pub type CrawlResult = Result<FileNode, CrawlError>;

#[derive(Debug, thiserror::Error)]
pub enum CrawlError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Glob pattern error: {0}")]
    GlobPattern(#[from] glob::PatternError),
    #[error("Glob error: {0}")]
    Glob(#[from] glob::GlobError),
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),
    #[error("Maximum depth exceeded")]
    MaxDepthExceeded,
}

/// Crawl a directory structure with optional glob pattern filtering
pub fn crawl_directory<P: AsRef<Path>>(root_path: P, options: CrawlOptions) -> CrawlResult {
    let root_path = root_path.as_ref();

    if !root_path.exists() {
        return Err(CrawlError::PathNotFound(root_path.to_path_buf()));
    }

    crawl_recursive(root_path, &options, 0)
}

fn crawl_recursive(path: &Path, options: &CrawlOptions, current_depth: usize) -> CrawlResult {
    if let Some(max_depth) = options.max_depth {
        if current_depth > max_depth {
            return Err(CrawlError::MaxDepthExceeded);
        }
    }

    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        let path = create_file_node(path)?;
        return Ok(path);
    }

    if metadata.is_dir() {
        let mut children = HashMap::new();
        let mut total_size = 0u64;

        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            // Skip hidden files if not included
            if !options.include_hidden && entry_name.starts_with('.') {
                continue;
            }

            // Skip symlinks if not following them
            if !options.follow_symlinks && entry_path.is_symlink() {
                continue;
            }

            // Skip if path matches any exclude patterns (applies to both files and directories)
            if !options.exclude_patterns.is_empty()
                && matches_any_pattern(&entry_path, &options.exclude_patterns)
            {
                continue;
            }

            // For files, check if they match the include glob patterns
            // For directories, we always recurse (unless excluded above)
            if entry_path.is_file() {
                // If we have include patterns, file must match at least one
                if !options.glob_patterns.is_empty()
                    && !matches_any_pattern(&entry_path, &options.glob_patterns)
                {
                    continue;
                }
            }

            match crawl_recursive(&entry_path, options, current_depth + 1) {
                Ok(child_node) => {
                    match &child_node {
                        FileNode::File { size, .. } => total_size += size,
                        FileNode::Directory {
                            total_size: dir_size,
                            ..
                        } => {
                            total_size += dir_size;
                        }
                    }
                    children.insert(entry_name, child_node);
                }
                Err(CrawlError::MaxDepthExceeded) => continue,
                Err(e) => return Err(e),
            }
        }

        return Ok(FileNode::Directory {
            name: path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
            path: path.to_path_buf(),
            children,
            total_size,
        });
    }

    // Handle other file types (symlinks, etc.)
    create_file_node(path)
}

fn create_file_node(path: &Path) -> Result<FileNode, CrawlError> {
    let metadata = fs::metadata(path)?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    if metadata.is_file() {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string());

        Ok(FileNode::File {
            name,
            path: path.to_path_buf(),
            size: metadata.len(),
            extension,
        })
    } else {
        Ok(FileNode::Directory {
            name,
            path: path.to_path_buf(),
            children: HashMap::new(),
            total_size: 0,
        })
    }
}

fn matches_any_pattern(path: &Path, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false; // Empty patterns should match nothing, not everything
    }

    let path_str = path.to_string_lossy();

    for pattern in patterns {
        if let Ok(glob_matcher) = glob::Pattern::new(pattern) {
            // Check full path
            if glob_matcher.matches(&path_str) {
                return true;
            }
            // Also check just the filename
            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_string_lossy();
                if glob_matcher.matches(&filename_str) {
                    return true;
                }
            }
            // For relative paths, try without leading components
            if let Ok(relative_path) =
                path.strip_prefix(std::env::current_dir().unwrap_or_default())
            {
                if glob_matcher.matches(&relative_path.to_string_lossy()) {
                    return true;
                }
            }

            // Special handling for directory patterns like "target/**"
            // If pattern ends with "/**", also match the directory name itself
            if pattern.ends_with("/**") {
                let dir_pattern = pattern.trim_end_matches("/**");
                if let Some(filename) = path.file_name() {
                    if filename.to_string_lossy() == dir_pattern {
                        return true;
                    }
                }
            }
        }
    }

    false
}

// Utility functions for working with the file tree
impl FileNode {
    pub fn find_files_by_extension(&self, ext: &str) -> Vec<&FileNode> {
        let mut results = Vec::new();
        self.collect_files_by_extension(ext, &mut results);
        results
    }

    fn collect_files_by_extension<'a>(&'a self, ext: &str, results: &mut Vec<&'a FileNode>) {
        match self {
            FileNode::File {
                extension: Some(file_ext),
                ..
            } if file_ext == ext => {
                results.push(self);
            }
            FileNode::Directory { children, .. } => {
                for child in children.values() {
                    child.collect_files_by_extension(ext, results);
                }
            }
            _ => {}
        }
    }

    pub fn total_files(&self) -> usize {
        match self {
            FileNode::File { .. } => 1,
            FileNode::Directory { children, .. } => {
                children.values().map(|child| child.total_files()).sum()
            }
        }
    }

    pub fn print_tree(&self, indent: usize) {
        let prefix = "  ".repeat(indent);
        match self {
            FileNode::File {
                name,
                size,
                extension,
                ..
            } => {
                let ext_str = extension.as_deref().unwrap_or("");
                println!("{}📄 {} ({} bytes) {}", prefix, name, size, ext_str);
            }
            FileNode::Directory {
                name,
                children,
                total_size,
                ..
            } => {
                println!("{}📁 {} ({} bytes total)", prefix, name, total_size);
                for child in children.values() {
                    child.print_tree(indent + 1);
                }
            }
        }
    }

    /// Returns an iterator over all nodes in the tree (depth-first)
    pub fn iter(&self) -> FileNodeIterator {
        FileNodeIterator::new(self)
    }

    /// Returns an iterator over all nodes with their depth level
    pub fn iter_with_depth(&self) -> FileNodeDepthIterator {
        FileNodeDepthIterator::new(self)
    }

    /// Collects all nodes into a Vec (convenience method)
    #[allow(dead_code)]
    pub fn collect_all_nodes(&self) -> Vec<&FileNode> {
        self.iter().collect()
    }

    /// Collects all file nodes only
    pub fn collect_files(&self) -> Vec<&FileNode> {
        self.iter().filter(|node| node.is_file()).collect()
    }

    /// Collects all directory nodes only
    #[allow(dead_code)]
    pub fn collect_directories(&self) -> Vec<&FileNode> {
        self.iter().filter(|node| node.is_directory()).collect()
    }
}

/// Iterator that traverses all nodes in the file tree (depth-first)
pub struct FileNodeIterator<'a> {
    stack: Vec<&'a FileNode>,
}

impl<'a> FileNodeIterator<'a> {
    fn new(root: &'a FileNode) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for FileNodeIterator<'a> {
    type Item = &'a FileNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.stack.pop() {
            // If it's a directory, add its children to the stack
            if let FileNode::Directory { children, .. } = current {
                // Collect into Vec first so we can reverse for correct traversal order
                let mut child_vec: Vec<_> = children.values().collect();
                child_vec.reverse();
                for child in child_vec {
                    self.stack.push(child);
                }
            }
            Some(current)
        } else {
            None
        }
    }
}

/// Iterator that traverses all nodes with their depth level
pub struct FileNodeDepthIterator<'a> {
    stack: Vec<(&'a FileNode, usize)>,
}

impl<'a> FileNodeDepthIterator<'a> {
    fn new(root: &'a FileNode) -> Self {
        Self {
            stack: vec![(root, 0)],
        }
    }
}

impl<'a> Iterator for FileNodeDepthIterator<'a> {
    type Item = (&'a FileNode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((current, depth)) = self.stack.pop() {
            // If it's a directory, add its children to the stack with incremented depth
            if let FileNode::Directory { children, .. } = current {
                // Collect into Vec first so we can reverse for correct traversal order
                let mut child_vec: Vec<_> = children.values().collect();
                child_vec.reverse();
                for child in child_vec {
                    self.stack.push((child, depth + 1));
                }
            }
            Some((current, depth))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_basic_crawl() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test structure
        fs::create_dir(temp_path.join("subdir")).unwrap();
        fs::write(temp_path.join("file1.txt"), "content1").unwrap();
        fs::write(temp_path.join("subdir/file2.rs"), "content2").unwrap();

        let options = CrawlOptions::default();
        let result = crawl_directory(temp_path, options).unwrap();

        match result {
            FileNode::Directory { children, .. } => {
                assert!(children.contains_key("file1.txt"));
                assert!(children.contains_key("subdir"));
            }
            _ => panic!("Expected directory node"),
        }
    }

    #[test]
    fn test_glob_patterns() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        fs::write(temp_path.join("test.rs"), "rust code").unwrap();
        fs::write(temp_path.join("test.txt"), "text file").unwrap();

        let options = CrawlOptions {
            glob_patterns: vec!["*.rs".to_string()],
            ..Default::default()
        };

        let result = crawl_directory(temp_path, options).unwrap();

        match result {
            FileNode::Directory { children, .. } => {
                assert!(children.contains_key("test.rs"));
                assert!(!children.contains_key("test.txt"));
            }
            _ => panic!("Expected directory node"),
        }
    }

    #[test]
    fn test_exclude_patterns() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test structure
        fs::create_dir(temp_path.join("target")).unwrap();
        fs::create_dir(temp_path.join("src")).unwrap();
        fs::write(temp_path.join("src/main.rs"), "rust code").unwrap();
        fs::write(temp_path.join("target/debug.txt"), "build artifact").unwrap();

        let options = CrawlOptions {
            exclude_patterns: vec!["target".to_string(), "*.log".to_string()],
            ..Default::default()
        };

        let result = crawl_directory(temp_path, options).unwrap();

        match result {
            FileNode::Directory { children, .. } => {
                assert!(!children.contains_key("target"));
                assert!(children.contains_key("src"));
            }
            _ => panic!("Expected directory node"),
        }
    }

    #[test]
    fn test_iterator() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test structure
        fs::create_dir(temp_path.join("subdir")).unwrap();
        fs::write(temp_path.join("file1.txt"), "content1").unwrap();
        fs::write(temp_path.join("subdir/file2.rs"), "content2").unwrap();

        let options = CrawlOptions::default();
        let result = crawl_directory(temp_path, options).unwrap();

        // Test iterator
        let all_nodes: Vec<_> = result.iter().collect();
        assert!(all_nodes.len() >= 3); // root dir + file1.txt + subdir + file2.rs

        // Test file collection
        let files = result.collect_files();
        assert_eq!(files.len(), 2); // file1.txt and file2.rs

        // Test directory collection
        let dirs = result.collect_directories();
        assert!(dirs.len() >= 2); // root and subdir

        // Test depth iterator
        let with_depth: Vec<_> = result.iter_with_depth().collect();
        assert!(with_depth.iter().any(|(_, depth)| *depth == 0)); // root at depth 0
        assert!(with_depth.iter().any(|(_, depth)| *depth == 1)); // files/subdirs at depth 1
        assert!(with_depth.iter().any(|(_, depth)| *depth == 2)); // file2.rs at depth 2
    }
}

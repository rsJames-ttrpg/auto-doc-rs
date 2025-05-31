use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error, warn};

use crate::analysis::summary::{
    AnalysisAudience, AnalysisContext, AnalysisDepth, AnalysisError, ChildAnalysis, FileAnalysis,
    LlmAnalyser, ProjectAnalysis, ProjectType,
};
use crate::crawler::file::{CrawlError, CrawlOptions, FileNode, crawl_directory};

#[derive(Debug, Clone)]
pub struct AnalysisCrawlOptions {
    /// File system crawling options
    pub crawl_options: CrawlOptions,
    /// Analysis context for LLM processing
    pub analysis_context: AnalysisContext,
    /// File extensions to analyze (empty means analyze all text files)
    pub analyzable_extensions: Vec<String>,
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: u64,
}

impl Default for AnalysisCrawlOptions {
    fn default() -> Self {
        Self {
            crawl_options: CrawlOptions::default(),
            analysis_context: AnalysisContext {
                project_type: ProjectType::Unknown,
                target_audience: AnalysisAudience::LlmConsumption,
                analysis_depth: AnalysisDepth::Standard,
            },
            analyzable_extensions: vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "go".to_string(),
                "java".to_string(),
                "cpp".to_string(),
                "c".to_string(),
                "h".to_string(),
                "hpp".to_string(),
                "cs".to_string(),
                "php".to_string(),
                "rb".to_string(),
                "swift".to_string(),
                "kt".to_string(),
                "scala".to_string(),
                "clj".to_string(),
                "ex".to_string(),
                "elm".to_string(),
                "hs".to_string(),
                "ml".to_string(),
                "toml".to_string(),
                "yaml".to_string(),
                "yml".to_string(),
                "json".to_string(),
                "xml".to_string(),
                "md".to_string(),
                "txt".to_string(),
                "config".to_string(),
                "conf".to_string(),
            ],
            max_file_size: 1024 * 1024, // 1MB
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AnalysisCrawlError {
    #[error("Crawl error: {0}")]
    Crawl(#[from] CrawlError),
    #[error("Analysis error: {0}")]
    Analysis(#[from] AnalysisError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}

pub struct AnalysisCrawler<A: LlmAnalyser + Clone + 'static> {
    analyser: A,
}

impl<A: LlmAnalyser> AnalysisCrawler<A> {
    pub fn new(analyser: A) -> Self {
        Self { analyser }
    }

    /// Crawl and analyze a project directory
    pub async fn analyze_project<P: AsRef<Path>>(
        &self,
        root_path: P,
        options: AnalysisCrawlOptions,
    ) -> Result<(Option<ProjectAnalysis>, Vec<ChildAnalysis>), AnalysisCrawlError> {
        let root_path = root_path.as_ref();
        debug!("Starting analysis of: {}", root_path.display());

        // First, crawl the directory structure
        let file_tree = crawl_directory(root_path, options.crawl_options.clone())?;

        // Print what we found during crawling
        debug!("File tree structure: {}", file_tree.tree_string());

        // Then analyze the structure
        debug!("Starting analyze_file_tree...");
        let child_analyses = self.analyze_file_tree(&file_tree, &options).await?;

        // Debug what analyze_file_tree returned
        debug!("analyze_file_tree returned {} items:", child_analyses.len());
        for (i, child) in child_analyses.iter().enumerate() {
            match child {
                ChildAnalysis::File(f) => {
                    debug!("  {}: FILE: {}", i, f.file_path.display());
                }
                ChildAnalysis::Directory(d) => {
                    debug!(
                        "  {}: DIRECTORY: {} (depth: {})",
                        i,
                        d.directory_path.display(),
                        d.depth_level
                    );
                }
            }
        }

        // Finally, synthesize into project analysis
        let project_analysis = match self
            .analyser
            .analyze_project(root_path, &child_analyses, &options.analysis_context)
            .await
        {
            Ok(proj) => Some(proj),
            Err(e) => {
                error!("Error with Project analysis {}", e.to_string());
                None
            }
        };

        Ok((project_analysis, child_analyses))
    }

    /// Analyze a file tree node and all its children
    pub fn analyze_file_tree<'a>(
        &'a self,
        node: &'a FileNode,
        options: &'a AnalysisCrawlOptions,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Vec<ChildAnalysis>, AnalysisCrawlError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            match node {
                FileNode::File { .. } => {
                    // Single file analysis
                    if let Some(analysis) = self.analyze_single_file(node, options).await? {
                        Ok(vec![ChildAnalysis::File(analysis)])
                    } else {
                        Ok(vec![])
                    }
                }
                FileNode::Directory { children, .. } => {
                    let mut child_analyses = Vec::new();

                    // Process each immediate child
                    for child in children.values() {
                        match child {
                            FileNode::File { .. } => {
                                if self.should_analyze_file(child, options) {
                                    match self.analyze_single_file(child, options).await {
                                        Ok(Some(file_analysis)) => {
                                            child_analyses.push(ChildAnalysis::File(file_analysis));
                                        }
                                        Ok(None) => {
                                            warn!("Empty analysis for {}", child.name())
                                        }
                                        Err(e) => {
                                            error!(
                                                "Analysis Failed for {} with error: {}",
                                                child.name(),
                                                e.to_string()
                                            )
                                        }
                                    }
                                }
                            }
                            FileNode::Directory { .. } => {
                                // Recursively analyze subdirectory (boxed to avoid infinite size)
                                let sub_analyses = self.analyze_file_tree(child, options).await?;

                                if !sub_analyses.is_empty() {
                                    child_analyses.extend(sub_analyses.clone());
                                    // Create directory analysis for this subdirectory
                                    match self
                                        .analyser
                                        .analyze_directory(
                                            child.path(),
                                            &sub_analyses,
                                            &options.analysis_context,
                                        )
                                        .await
                                    {
                                        Ok(dir_analysis) => {
                                            child_analyses
                                                .push(ChildAnalysis::Directory(dir_analysis));
                                        }
                                        Err(e) => error!(
                                            "Error with directory Analysis: {}, error: {}",
                                            child.name(),
                                            e.to_string()
                                        ),
                                    }
                                }
                            }
                        }
                    }

                    Ok(child_analyses)
                }
            }
        })
    }

    async fn analyze_single_file_static(
        analyser: &A,
        file_node: &FileNode,
        options: &AnalysisCrawlOptions,
    ) -> Result<Option<FileAnalysis>, AnalysisCrawlError> {
        if let FileNode::File { path, size, .. } = file_node {
            // Check file size limit
            if *size > options.max_file_size {
                return Ok(None);
            }

            // Read file content
            let content = match fs::read_to_string(path) {
                Ok(content) => content,
                Err(_) => return Ok(None), // Skip binary or unreadable files
            };

            // Analyze with LLM
            let analysis = analyser
                .analyze_file(path, &content, &options.analysis_context)
                .await?;

            Ok(Some(analysis))
        } else {
            Ok(None)
        }
    }

    async fn analyze_single_file(
        &self,
        file_node: &FileNode,
        options: &AnalysisCrawlOptions,
    ) -> Result<Option<FileAnalysis>, AnalysisCrawlError> {
        Self::analyze_single_file_static(&self.analyser, file_node, options).await
    }

    fn should_analyze_file(&self, file_node: &FileNode, options: &AnalysisCrawlOptions) -> bool {
        Self::should_analyze_file_static(file_node, options)
    }

    fn should_analyze_file_static(file_node: &FileNode, options: &AnalysisCrawlOptions) -> bool {
        match file_node {
            FileNode::File {
                extension, size, ..
            } => {
                // Check size limit
                if *size > options.max_file_size {
                    return false;
                }
                if *size == 0 {
                    return false;
                }
                // If no extensions specified, analyze all files
                if options.analyzable_extensions.is_empty() {
                    return true;
                }

                // Check if extension matches
                if let Some(ext) = extension {
                    options.analyzable_extensions.contains(ext)
                } else {
                    false // Skip files without extensions unless explicitly included
                }
            }
            FileNode::Directory { .. } => true, // Always process directories
        }
    }

    /// Get a summary of what would be analyzed without actually analyzing
    pub fn preview_analysis<P: AsRef<Path>>(
        &self,
        root_path: P,
        options: &AnalysisCrawlOptions,
    ) -> Result<AnalysisPreview, AnalysisCrawlError> {
        let file_tree = crawl_directory(root_path, options.crawl_options.clone())?;
        let preview = self.build_preview(&file_tree, options);
        Ok(preview)
    }

    fn build_preview(&self, node: &FileNode, options: &AnalysisCrawlOptions) -> AnalysisPreview {
        let mut preview = AnalysisPreview::default();
        self.collect_preview_stats(node, options, &mut preview);
        preview
    }

    fn collect_preview_stats(
        &self,
        node: &FileNode,
        options: &AnalysisCrawlOptions,
        preview: &mut AnalysisPreview,
    ) {
        match node {
            FileNode::File {
                path,
                size,
                extension,
                ..
            } => {
                preview.total_files += 1;
                preview.total_size += size;

                if self.should_analyze_file(node, options) {
                    preview.analyzable_files += 1;
                    preview.analyzable_size += size;

                    if let Some(ext) = extension {
                        *preview.file_types.entry(ext.clone()).or_insert(0) += 1;
                    }
                } else {
                    preview.skipped_files += 1;
                    if *size > options.max_file_size {
                        preview.oversized_files.push(path.clone());
                    }
                }
            }
            FileNode::Directory { children, .. } => {
                preview.total_directories += 1;
                for child in children.values() {
                    self.collect_preview_stats(child, options, preview);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct AnalysisPreview {
    pub total_files: usize,
    pub total_directories: usize,
    pub total_size: u64,
    pub analyzable_files: usize,
    pub analyzable_size: u64,
    pub skipped_files: usize,
    pub oversized_files: Vec<PathBuf>,
    pub file_types: HashMap<String, usize>,
}

impl AnalysisPreview {
    pub fn print_summary(&self) {
        println!("Analysis Preview:");
        println!("  Total files: {}", self.total_files);
        println!("  Total directories: {}", self.total_directories);
        println!("  Total size: {} bytes", self.total_size);
        println!("  Analyzable files: {}", self.analyzable_files);
        println!("  Analyzable size: {} bytes", self.analyzable_size);
        println!("  Skipped files: {}", self.skipped_files);

        if !self.oversized_files.is_empty() {
            println!("  Oversized files ({}):", self.oversized_files.len());
            for file in &self.oversized_files {
                println!("    {}", file.display());
            }
        }

        if !self.file_types.is_empty() {
            println!("  File types to analyze:");
            let mut types: Vec<_> = self.file_types.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1));
            for (ext, count) in types {
                println!("    .{}: {} files", ext, count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::summary::DirectoryAnalysis;
    use mockall::mock;
    use std::fs;
    use tempfile::TempDir;

    mock! {
        TestAnalyser {}


        impl Clone for TestAnalyser {
            fn clone(&self) -> Self;
        }

        #[async_trait::async_trait]
        impl LlmAnalyser for TestAnalyser {
            async fn analyze_file(
                &self,
                file_path: &Path,
                file_content: &str,
                context: &AnalysisContext,
            ) -> Result<FileAnalysis, AnalysisError>;

            async fn analyze_directory(
                &self,
                directory_path: &Path,
                child_analyses: &[ChildAnalysis],
                context: &AnalysisContext,
            ) -> Result<DirectoryAnalysis, AnalysisError>;

            async fn analyze_project(
                &self,
                project_root: &Path,
                child_analyses: &[ChildAnalysis],
                context: &AnalysisContext,
            ) -> Result<ProjectAnalysis, AnalysisError>;
        }
    }

    #[tokio::test]
    async fn test_analysis_preview() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test structure
        fs::create_dir(temp_path.join("src")).unwrap();
        fs::write(temp_path.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(temp_path.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
        fs::write(temp_path.join("README.md"), "# Test Project").unwrap();

        let analyser = MockTestAnalyser::new();
        let crawler = AnalysisCrawler::new(analyser);

        let preview = crawler
            .preview_analysis(temp_path, &AnalysisCrawlOptions::default())
            .unwrap();

        assert!(preview.total_files > 0);
        assert!(preview.analyzable_files > 0);
        assert!(preview.file_types.contains_key("rs"));
        assert!(preview.file_types.contains_key("toml"));
    }
}

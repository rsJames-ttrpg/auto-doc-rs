use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::analysis::summary::{ChildAnalysis, DirectoryAnalysis, FileAnalysis, ProjectAnalysis};

/// Configuration for markdown generation
#[derive(Debug, Clone)]
pub struct MarkdownConfig {
    /// Output directory where markdown files will be written
    pub output_dir: PathBuf,
    /// Whether to create directory index files (README.md or index.md)
    pub create_directory_indices: bool,
    /// Name for directory index files
    pub directory_index_name: String,
    /// Base path to strip from file paths when creating relative structure
    pub project_root: Option<PathBuf>,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./_docs"),
            create_directory_indices: true,
            directory_index_name: "README.md".to_string(),
            project_root: None,
        }
    }
}
/// Builder for MarkdownConfig with fluent API
#[derive(Debug, Clone)]
pub struct MarkdownConfigBuilder {
    config: MarkdownConfig,
}

impl MarkdownConfigBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            config: MarkdownConfig::default(),
        }
    }

    /// Set the output directory where markdown files will be written
    pub fn output_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.config.output_dir = dir.into();
        self
    }

    #[allow(dead_code)]
    /// Set whether to create directory index files
    pub fn create_directory_indices(mut self, create: bool) -> Self {
        self.config.create_directory_indices = create;
        self
    }

    #[allow(dead_code)]
    /// Disable directory index file creation (convenience method)
    pub fn no_directory_indices(mut self) -> Self {
        self.config.create_directory_indices = false;
        self
    }

    #[allow(dead_code)]
    /// Set the name for directory index files
    pub fn directory_index_name<S: Into<String>>(mut self, name: S) -> Self {
        self.config.directory_index_name = name.into();
        self
    }

    #[allow(dead_code)]
    /// Use "index.md" instead of "README.md" for directory indices
    pub fn use_index_md(mut self) -> Self {
        self.config.directory_index_name = "index.md".to_string();
        self
    }

    #[allow(dead_code)]
    /// Set the project root path to strip from file paths
    pub fn project_root<P: Into<PathBuf>>(mut self, root: P) -> Self {
        self.config.project_root = Some(root.into());
        self
    }

    #[allow(dead_code)]
    /// Clear the project root (don't strip any paths)
    pub fn no_project_root(mut self) -> Self {
        self.config.project_root = None;
        self
    }

    /// Build the final MarkdownConfig
    pub fn build(self) -> MarkdownConfig {
        self.config
    }
}

impl Default for MarkdownConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownConfig {
    /// Create a new builder
    pub fn builder() -> MarkdownConfigBuilder {
        MarkdownConfigBuilder::new()
    }
}
/// Generates markdown documentation from project analysis
pub struct MarkdownGenerator {
    config: MarkdownConfig,
}

impl MarkdownGenerator {
    pub fn new(config: MarkdownConfig) -> Self {
        Self { config }
    }

    /// Generate markdown files for the entire project analysis
    pub async fn generate_documentation(
        &self,
        project_analysis: &Option<ProjectAnalysis>,
        child_analyses: &[ChildAnalysis],
    ) -> Result<()> {
        // Create output directory
        fs::create_dir_all(&self.config.output_dir).await?;

        // Generate project overview using Display implementation
        if let Some(project_analysis) = project_analysis {
            self.generate_project_overview(project_analysis).await?;
        }

        for analysis in child_analyses {
            match &analysis {
                ChildAnalysis::Directory(dir) => {
                    self.generate_directory_markdown(dir).await?;
                }
                ChildAnalysis::File(file) => {
                    self.generate_file_markdown(file).await?;
                }
            }
        }

        println!(
            "Documentation generated in: {}",
            self.config.output_dir.display()
        );
        Ok(())
    }

    /// Generate project-level overview markdown using Display impl
    async fn generate_project_overview(&self, analysis: &ProjectAnalysis) -> Result<()> {
        let content = analysis.to_string();
        let output_path = self.config.output_dir.join("README.md");
        fs::write(output_path, content).await?;
        Ok(())
    }

    /// Generate markdown for a directory analysis using Display impl
    async fn generate_directory_markdown(&self, analysis: &DirectoryAnalysis) -> Result<()> {
        let content = analysis.to_string();

        // Create the directory structure in output
        let relative_path = self.make_relative_path(&analysis.directory_path);
        let output_dir = self.config.output_dir.join(&relative_path);
        fs::create_dir_all(&output_dir).await?;

        // Write directory index file
        if self.config.create_directory_indices {
            let index_path = output_dir.join(&self.config.directory_index_name);
            fs::write(index_path, content).await?;
        }

        Ok(())
    }

    /// Generate markdown for a file analysis using Display impl
    async fn generate_file_markdown(&self, analysis: &FileAnalysis) -> Result<()> {
        let content = analysis.to_string();

        // Create output path with .md extension
        let relative_path = self.make_relative_path(&analysis.file_path);
        let mut output_path = self.config.output_dir.join(&relative_path);

        // Replace original extension with .md
        if let Some(file_name) = output_path.file_stem() {
            output_path.set_file_name(format!("{}.md", file_name.to_string_lossy()));
        }

        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(output_path, content).await?;
        Ok(())
    }

    /// Convert paths to relative paths for output structure
    fn make_relative_path(&self, path: &Path) -> PathBuf {
        if let Some(project_root) = &self.config.project_root {
            // Strip project root if configured
            match path.strip_prefix(project_root) {
                Ok(relative_path) => relative_path.to_path_buf(),
                Err(_) => path.to_path_buf(),
            }
        } else {
            // Remove leading slash if present, or return as-is
            let result = path
                .strip_prefix("/")
                .or_else(|_| path.strip_prefix("./"))
                .unwrap_or(path)
                .to_path_buf();
            println!(
                "DEBUG: No project root, processed '{}' -> '{}'",
                path.display(),
                result.display()
            );
            result
        }
    }
}

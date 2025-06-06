use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

// Core data structures
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileAnalysis {
    #[schemars(description = "Path to the file being analyzed")]
    pub file_path: PathBuf,
    #[schemars(description = "File extension or type (e.g., 'rs', 'ts', 'py')")]
    pub file_type: String,
    #[schemars(
        description = "Brief 2-3 sentence description of what this file does and its role in the system"
    )]
    pub summary: String,
    #[schemars(
        description = "External crates, libraries, services, or system dependencies this file uses"
    )]
    pub external_dependencies: Vec<String>,
    #[schemars(
        description = "Public functions, structs, traits, or modules that other components can use"
    )]
    pub public_interfaces: Vec<Interface>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DirectoryAnalysis {
    #[schemars(description = "Path to the directory being analyzed")]
    pub directory_path: PathBuf,
    #[schemars(description = "Directory nesting level from project root (0 = root)")]
    pub depth_level: usize,
    #[schemars(
        description = "High-level description of what this directory accomplishes within the larger system"
    )]
    pub summary: String,
    #[schemars(description = "Condensed summaries from immediate child files and directories")]
    pub child_summaries: Vec<String>,
    #[schemars(description = "Most important components, files, or modules in this directory")]
    pub key_components: Vec<String>,
    #[schemars(description = "Consolidated external dependencies from all child components")]
    pub external_dependencies: Vec<String>,
    #[schemars(
        description = "Main interfaces this directory exposes to other parts of the system"
    )]
    pub public_interfaces: Vec<Interface>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProjectAnalysis {
    #[schemars(
        description = "Executive summary of what this software does and its primary value proposition"
    )]
    pub project_overview: String,
    #[schemars(
        description = "High-level description of how the system is structured and organized"
    )]
    pub architecture_summary: String,
    #[schemars(description = "Primary technologies, frameworks, and significant dependencies")]
    pub core_technologies: Vec<String>,
    #[schemars(description = "Interfaces into the project")]
    pub main_interfaces: Vec<Interface>,
    #[schemars(description = "Key requirements for running/deploying this software")]
    pub development_considerations: Vec<String>,
    #[schemars(description = "Areas where the system is designed to be extended or customized")]
    pub extension_points: Vec<String>,
    #[schemars(description = "Potential technical risks or dependencies that could cause issues")]
    pub risk_factors: Vec<Interface>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Interface {
    #[schemars(
        description = "Name of the interface (function name, struct name, API endpoint, etc.)"
    )]
    pub name: String,
    #[schemars(description = "Category of interface this represents")]
    pub interface_type: InterfaceType,
    #[schemars(description = "What this interface provides and how other components can use it")]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum InterfaceType {
    #[schemars(description = "A callable function or method")]
    Function,
    #[schemars(description = "A data structure or type definition")]
    Struct,
    #[schemars(description = "A trait defining shared behaviour")]
    Trait,
    #[schemars(description = "A module or namespace")]
    Module,
    #[schemars(description = "HTTP API endpoint or service interface")]
    Api,
    #[schemars(description = "Configuration file, environment variable, or setting")]
    Configuration,
    #[schemars(description = "Database schema, data model, or serialization format")]
    DataModel,
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum AnalysisError {
    #[error("LLM communication failed: {0}")]
    LlmError(String),
    #[error("Parsing error: {0}")]
    ParseError(String),
}

// Main trait for LLM interactions
#[async_trait]
pub trait LlmAnalyser: Send + Sync + Clone {
    /// Analyze a single file and return structured analysis
    async fn analyze_file(
        &self,
        file_path: &Path,
        file_content: &str,
        context: &AnalysisContext,
    ) -> Result<FileAnalysis, AnalysisError>;

    /// Synthesize multiple child analyses into a directory-level summary
    async fn analyze_directory(
        &self,
        directory_path: &Path,
        child_analyses: &[ChildAnalysis],
        context: &AnalysisContext,
    ) -> Result<DirectoryAnalysis, AnalysisError>;

    // Synthesize the Root Analysis into a project level analysis
    async fn analyze_project(
        &self,
        project_root: &Path,
        child_analyses: &[ChildAnalysis],
        context: &AnalysisContext,
    ) -> Result<ProjectAnalysis, AnalysisError>;
}

#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub project_type: ProjectType,
    pub target_audience: AnalysisAudience,
    pub analysis_depth: AnalysisDepth,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ProjectType {
    WebApplication,
    Library,
    CliTool,
    SystemService,
    DeveloperTool,
    Unknown,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AnalysisAudience {
    LlmConsumption,
    HumanDeveloper,
    TechnicalDocumentation,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AnalysisDepth {
    Surface,  // Just interfaces and dependencies
    Standard, // Full analysis
    Deep,     // Include implementation details
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(tag = "type")]
pub enum ChildAnalysis {
    File(FileAnalysis),
    Directory(DirectoryAnalysis),
}

impl Display for ChildAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ChildAnalysis::Directory(dir) => write!(f, "{}", dir),
            ChildAnalysis::File(file) => write!(f, "{}", file),
        }
    }
}

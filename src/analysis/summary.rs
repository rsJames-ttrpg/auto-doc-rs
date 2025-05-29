use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::analysis::prompt::PromptTemplates;
use crate::llm_interface::LlmClient;

pub trait SimplifiedSchema {
    /// Generate a simplified, Google-friendly JSON schema
    fn simplified_schema() -> serde_json::Value;
}

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

impl SimplifiedSchema for FileAnalysis {
    /// Generate a simplified schema by modifying the generated one
    fn simplified_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["file_path", "summary", "external_dependencies", "file_type", "public_interfaces"],
            "properties": {
                "file_path": {"type": "string"},
                "summary": {"type": "string"},
                "file_type": {"type": "string"},
                "external_dependencies": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "public_interfaces": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["name", "interface_type"],
                        "properties": {
                            "name": {"type": "string"},
                            "interface_type": {"type": "string", "enum": ["Function", "Struct", "Trait", "Module", "Api", "Configuration", "DataModel"],},
                            "description": {"type": "string"}
                        }
                    }
                }
            }
        })
    }
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

impl SimplifiedSchema for DirectoryAnalysis {
    /// Generate a simplified schema for directory analysis
    fn simplified_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["directory_path", "summary", "child_summaries",  "depth_level", "key_components", "external_dependencies", "public_interfaces"],
            "properties": {
                "directory_path": {"type": "string"},
                "depth_level" : {"type": "integer"},
                "summary": {"type": "string"},
                "key_components": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "child_summaries": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "external_dependencies": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "public_interfaces": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["name", "interface_type"],
                        "properties": {
                            "name": {"type": "string"},
                            "interface_type": {"type": "string", "enum": ["Function", "Struct", "Trait", "Module", "Api", "Configuration", "DataModel"],},
                            "description": {"type": "string"}
                        }
                    }
                }
            }
        })
    }
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

impl SimplifiedSchema for ProjectAnalysis {
    /// Generate a simplified schema for project analysis
    fn simplified_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["project_overview", "architecture_summary", "core_technologies", "main_interfaces", "development_considerations", "extension_points", "risk_factors"],
            "properties": {
                "project_overview": {"type": "string"},
                "architecture_summary": {"type": "string"},
                "core_technologies": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "main_interfaces": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["name", "interface_type"],
                        "properties": {
                            "name": {"type": "string"},
                            "interface_type": {"type": "string", "enum": ["Function", "Struct", "Trait", "Module", "Api", "Configuration", "DataModel"],},
                            "description": {"type": "string"}
                        }
                    }
                },
                "development_considerations": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "extension_points": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "risk_factors": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["name", "interface_type"],
                        "properties": {
                            "name": {"type": "string"},
                            "interface_type": {"type": "string", "enum": ["Function", "Struct", "Trait", "Module", "Api", "Configuration", "DataModel"],},
                            "description": {"type": "string"}
                        }
                    }
                }
            }
        })
    }
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
    #[schemars(description = "A trait defining shared behavior")]
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
pub trait LlmAnalyzer: Send + Sync + Clone {
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

#[async_trait]
impl LlmAnalyzer for LlmClient {
    async fn analyze_file(
        &self,
        file_path: &Path,
        file_content: &str,
        context: &AnalysisContext,
    ) -> Result<FileAnalysis, AnalysisError> {
        let templates = PromptTemplates::default();
        let prompt = templates.build_file_analysis_prompt(file_path, context);
        let request = self
            .request()
            .system_prompt(prompt)
            .content(file_content)
            .execute_structured_with_retry::<FileAnalysis>()
            .await;
        match request {
            Ok(res) => Ok(res),
            Err(e) => {
                eprint!("path: {:?}", file_path);
                Err(AnalysisError::LlmError(e.to_string()))
            }
        }
    }

    async fn analyze_directory(
        &self,
        directory_path: &Path,
        child_analyses: &[ChildAnalysis],
        context: &AnalysisContext,
    ) -> Result<DirectoryAnalysis, AnalysisError> {
        let templates = PromptTemplates::default();
        let prompt = templates.build_directory_synthesis_prompt(directory_path, context);
        let content: String = serde_json::to_string_pretty(child_analyses)
            .map_err(|e| AnalysisError::ParseError(e.to_string()))?;
        let request = self
            .request()
            .system_prompt(prompt)
            .content(content)
            .execute_structured_with_retry::<DirectoryAnalysis>()
            .await;
        match request {
            Ok(res) => Ok(res),
            Err(e) => {
                eprint!("path: {:?}", directory_path);
                Err(AnalysisError::LlmError(e.to_string()))
            }
        }
    }

    async fn analyze_project(
        &self,
        project_root: &Path,
        child_analyses: &[ChildAnalysis],
        context: &AnalysisContext,
    ) -> Result<ProjectAnalysis, AnalysisError> {
        let templates = PromptTemplates::default();
        let prompt = templates.build_project_analysis_prompt(project_root, context);
        let content: String = serde_json::to_string_pretty(child_analyses)
            .map_err(|e| AnalysisError::ParseError(e.to_string()))?;
        let request = self
            .request()
            .system_prompt(prompt)
            .content(content)
            .execute_structured_with_retry::<ProjectAnalysis>()
            .await;
        match request {
            Ok(res) => Ok(res),
            Err(e) => {
                eprint!("path: {:?}", project_root);
                Err(AnalysisError::LlmError(e.to_string()))
            }
        }
    }
}

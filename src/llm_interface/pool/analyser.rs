use std::path::Path;

use async_trait::async_trait;

use crate::analysis::{
    prompt::PromptTemplates,
    summary::{
        AnalysisContext, AnalysisError, ChildAnalysis, DirectoryAnalysis, FileAnalysis,
        LlmAnalyzer, ProjectAnalysis,
    },
};

use super::LlmPool;

#[async_trait]
impl LlmAnalyzer for LlmPool {
    async fn analyze_file(
        &self,
        file_path: &Path,
        file_content: &str,
        context: &AnalysisContext,
    ) -> Result<FileAnalysis, AnalysisError> {
        let templates = PromptTemplates::default();
        let prompt = templates.build_file_analysis_prompt(file_path, context);
        let request = self
            .execute_request(|client| {
                let prompt = prompt.clone();
                async move {
                    client
                        .request()
                        .system_prompt(prompt)
                        .content(file_content)
                        .execute_structured_with_retry::<FileAnalysis>()
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                }
            })
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
            .execute_request(|client| {
                let prompt = prompt.clone();
                let content = content.clone();
                async move {
                    client
                        .request()
                        .system_prompt(prompt)
                        .content(content)
                        .execute_structured_with_retry::<DirectoryAnalysis>()
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                }
            })
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
            .execute_request(|client| {
                let prompt = prompt.clone();
                let content = content.clone();
                async move {
                    client
                        .request()
                        .system_prompt(prompt)
                        .content(content)
                        .execute_structured_with_retry::<ProjectAnalysis>()
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                }
            })
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

use std::path::Path;

use super::summary::AnalysisContext;

#[derive(Debug, Clone)]
pub struct PromptTemplates {
    pub file_analysis_template: String,
    pub directory_synthesis_template: String,
    pub project_analysis_template: String,
}

impl Default for PromptTemplates {
    fn default() -> Self {
        Self {
            file_analysis_template: include_str!("templates/file_analysis.txt").to_string(),
            directory_synthesis_template: include_str!("templates/directory_analysis.txt")
                .to_string(),
            project_analysis_template: include_str!("templates/project_analysis.txt").to_string(),
        }
    }
}

impl PromptTemplates {
    pub fn build_file_analysis_prompt(
        &self,
        file_path: &Path,
        context: &AnalysisContext,
    ) -> String {
        self.file_analysis_template
            .replace("{FILE_PATH}", &file_path.display().to_string())
            .replace("{PROJECT_TYPE}", &format!("{:?}", context.project_type))
            .replace("{ANALYSIS_DEPTH}", &format!("{:?}", context.analysis_depth))
            .replace(
                "{TARGET_AUDIENCE}",
                &format!("{:?}", context.target_audience),
            )
    }

    pub fn build_directory_synthesis_prompt(
        &self,
        directory_path: &Path,
        context: &AnalysisContext,
    ) -> String {
        self.directory_synthesis_template
            .replace("{DIRECTORY_PATH}", &directory_path.display().to_string())
            .replace("{PROJECT_TYPE}", &format!("{:?}", context.project_type))
    }

    pub fn build_project_analysis_prompt(
        &self,
        project_root: &Path,
        context: &AnalysisContext,
    ) -> String {
        self.project_analysis_template
            .replace("{PROJECT_TYPE}", &format!("{:?}", context.project_type))
            .replace("{PROJECT_ROOT}", &format!("{:?}", project_root))
    }
}

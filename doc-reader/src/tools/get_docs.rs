use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::get_global_settings;
use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;

pub struct GetDocumentationTool;

impl GetDocumentationTool {
    pub fn tool() -> Tool {
        let settings = get_global_settings();
        let service_names: Vec<String> = settings.get_component_names();
        Tool {
            name: "get_docs".to_string(),
            description: Some("Retrieves Documentation for given projectss".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "component_name": {
                        "type": "string",
                        "enum": service_names,
                        "description": "The name of the project."
                    },
                    "depth": {
                        "type": "integer",
                        "minimum": 0,
                        "description": "depth of detail the documentation. 0 the least increasing the depth of the file structure."
                    },
                    "summary_only": {
                        "type": "boolean",
                        "description": "If true returns only summaries for the directories false will return all file summaries."
                    }
                },

                "required": ["project_name", "depth", "summary_only"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        move |request: CallToolRequest| {
            Box::pin(async move {
                let settings = get_global_settings();
                // Extract parameters from the request
                let args = match request.arguments.as_ref() {
                    Some(args) => args,
                    None => return tool_text_response!("No arguments provided"),
                };

                let component_name = match args.get("component_name").and_then(|v| v.as_str()) {
                    Some(name) => name,
                    None => return tool_text_response!("component_name parameter is required"),
                };

                let depth = match args.get("depth").and_then(|v| v.as_i64()) {
                    Some(d) => d as usize,
                    None => 0,
                };

                let summary_only = args
                    .get("summary_only")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // Find the component path
                let component_path = match settings.get_component_path(component_name) {
                    Some(path) => path,
                    None => {
                        return tool_text_response!(format!(
                            "Component '{}' not found",
                            component_name
                        ));
                    }
                };

                // Check if path exists
                if !component_path.exists() {
                    return tool_text_response!(format!(
                        "Documentation path does not exist: {} Ensure your configuration is correct.",
                        component_path.display()
                    ));
                }

                let mut files = match get_files_to_depth(component_path, depth) {
                    Ok(value) => value,
                    Err(e) => {
                        return tool_text_response!(format!(
                            "Error retrieving files: {:}",
                            e.to_string()
                        ));
                    }
                };

                if summary_only {
                    files.retain(|file| {
                        file.file_name()
                            .and_then(|name| name.to_str())
                            .map(|name| name.to_lowercase().contains("readme.md"))
                            .unwrap_or(false)
                    });
                }

                let content = format_file_contents(files, None);

                tool_text_response!(content)
            })
        }
    }
}

// Alternative implementation using iterative approach with a queue
fn get_files_to_depth<P: AsRef<Path>>(
    root_dir: P,
    max_depth: usize,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    use std::collections::VecDeque;

    let root_path = root_dir.as_ref();
    let mut files = Vec::new();
    let mut queue = VecDeque::new();

    // Queue entries: (path, depth)
    queue.push_back((root_path.to_path_buf(), 0));

    while let Some((current_path, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }

        let entries = fs::read_dir(&current_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                files.push(path);
            } else if path.is_dir() && depth < max_depth {
                queue.push_back((path, depth + 1));
            }
        }
    }

    Ok(files)
}

fn format_file_contents(file_paths: Vec<PathBuf>, max_size: Option<usize>) -> String {
    let mut result = String::new();

    for path in file_paths {
        result.push_str(&format!("\n## 📄 {}\n\n", path.display()));

        match fs::read_to_string(&path) {
            Ok(content) => {
                let display_content = if let Some(max) = max_size {
                    if content.len() > max {
                        format!(
                            "{}...\n\n*[File truncated at {} bytes]*",
                            &content[..max],
                            max
                        )
                    } else {
                        content
                    }
                } else {
                    content
                };

                result.push_str(&format!("```\n{}\n```\n\n", display_content));
            }
            Err(e) => {
                result.push_str(&format!("*Error reading file: {}*\n\n", e));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_files_to_depth() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory structure for testing
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create files and directories
        fs::write(root.join("file1.txt"), "content")?;
        fs::create_dir(root.join("dir1"))?;
        fs::write(root.join("dir1").join("file2.txt"), "content")?;
        fs::create_dir(root.join("dir1").join("subdir"))?;
        fs::write(
            root.join("dir1").join("subdir").join("file3.txt"),
            "content",
        )?;

        // Test depth 0 (only root level)
        let files = get_files_to_depth(root, 0)?;
        assert_eq!(files.len(), 1);
        assert!(files[0].file_name().unwrap() == "file1.txt");

        // Test depth 1 (root + one level down)
        let files = get_files_to_depth(root, 1)?;
        assert_eq!(files.len(), 2);

        // Test depth 2 (all files)
        let files = get_files_to_depth(root, 2)?;
        assert_eq!(files.len(), 3);

        Ok(())
    }

    #[test]
    fn test_get_file_contents() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create test files
        let file1 = root.join("test1.txt");
        let file2 = root.join("test2.txt");
        fs::write(&file1, "Hello, world!")?;
        fs::write(&file2, "This is a longer content that might be truncated")?;

        let paths = vec![file1, file2];
        let contents = format_file_contents(paths, None);

        assert!(contents.contains("Hello, world!"));

        Ok(())
    }
}

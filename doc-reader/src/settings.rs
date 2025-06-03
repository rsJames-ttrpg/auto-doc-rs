use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use clap::ValueEnum;
use config::{Config, ConfigError, Environment};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Component {
    pub relative_path: PathBuf,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            relative_path: PathBuf::from("doc-reader/_docs"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Project {
    pub location: PathBuf,
    pub components: HashMap<String, Component>, // Changed to HashMap
}

impl Default for Project {
    fn default() -> Self {
        let mut components = HashMap::new();
        components.insert("doc-reader".to_string(), Component::default());

        Self {
            location: PathBuf::from("/home/user/repos/auto-doc"),
            components,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(unused)]
pub struct Settings {
    pub projects: HashMap<String, Project>,
}

impl Default for Settings {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("auto-doc".to_string(), Project::default());
        Self { projects: map }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum FileType {
    Json,
    Toml,
    Yaml,
}

impl Settings {
    fn with_config_builder<F>(configure: F) -> Result<Self, ConfigError>
    where
        F: FnOnce(
            config::ConfigBuilder<config::builder::DefaultState>,
        ) -> config::ConfigBuilder<config::builder::DefaultState>,
    {
        let base_builder = Config::builder().add_source(Config::try_from(&Settings::default())?);

        let config = configure(base_builder)
            .add_source(
                Environment::with_prefix("DOC_READER")
                    .try_parsing(true)
                    .separator(".")
                    .list_separator(",")
                    .with_list_parse_key("projects"),
            )
            .build()?;

        config.try_deserialize()
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        let config_location: String = match std::env::var("DOC_READER.CONFIG") {
            Ok(location) => location,
            Err(_) => {
                // Create a sensible default config path
                let config_dir = dirs::config_dir()
                    .unwrap_or_else(|| PathBuf::from(".")) // Fallback to current directory
                    .join("doc-reader");

                config_dir.join("config").to_string_lossy().to_string()
            }
        };
        debug!("Looking for config in {}", config_location);
        Self::with_config_builder(|builder| {
            builder.add_source(config::File::with_name(&config_location).required(false))
        })
    }

    pub fn from_file(file_name: &str) -> Result<Self, ConfigError> {
        Self::with_config_builder(|builder| {
            builder.add_source(config::File::with_name(file_name).required(false))
        })
    }

    pub fn write_default_config(
        output: Option<PathBuf>,
        format: FileType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match output {
            Some(path) => {
                let file: File = File::create(&path)?;
                Self::write_config_to_writer(file, format)
            }
            None => Self::write_config_to_writer(io::stdout(), format),
        }
    }

    fn write_config_to_writer<W: Write>(
        mut writer: W,
        format: FileType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match format {
            FileType::Json => {
                serde_json::to_writer_pretty(&mut writer, &Self::default())?;
            }
            FileType::Toml => {
                let toml_str = toml::to_string_pretty(&Self::default())?;
                writer.write_all(toml_str.as_bytes())?;
            }
            FileType::Yaml => {
                serde_yaml::to_writer(&mut writer, &Self::default())?;
            }
        }
        Ok(())
    }

    pub fn get_component_names(&self) -> Vec<String> {
        self.projects
            .values()
            .flat_map(|project| project.components.keys().cloned())
            .collect()
    }

    pub fn get_project_names(&self) -> Vec<String> {
        self.projects.keys().cloned().collect()
    }

    pub fn get_project(&self, name: &str) -> Option<&Project> {
        self.projects.get(name)
    }

    pub fn get_component(&self, project_name: &str, component_name: &str) -> Option<&Component> {
        self.projects
            .get(project_name)
            .and_then(|project| project.components.get(component_name))
    }

    pub fn get_component_path(&self, component_name: &str) -> Option<PathBuf> {
        self.projects.values().find_map(|project| {
            project
                .components
                .get(component_name)
                .map(|component| project.location.join(&component.relative_path))
        })
    }

    // Get all components for a specific project
    pub fn get_project_component_names(&self, project_name: &str) -> Vec<String> {
        self.projects
            .get(project_name)
            .map(|project| project.components.keys().cloned().collect())
            .unwrap_or_default()
    }
}

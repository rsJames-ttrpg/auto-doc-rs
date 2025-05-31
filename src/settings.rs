use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use crate::llm_interface::{models::ModelId, pool::Behaviour};
use clap::ValueEnum;
use config::{Config, ConfigError, Environment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct CrawlOptions {
    pub max_depth: Option<usize>,
    pub include_hidden: bool,
    #[serde(default)]
    pub include_patterns: Vec<String>,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    pub git_mode: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LlmSettings {
    pub behaviour: Behaviour,
    pub max_retries: u32,
    pub initial_interval_ms: u32,
    pub max_interval_s: u32,
    pub multiplier: f64,
    pub max_elapsed_time_s: u32,
    pub models: Vec<LlmModel>,
}

impl Default for LlmSettings {
    fn default() -> Self {
        LlmSettings {
            max_retries: 5,
            initial_interval_ms: 5000,
            max_interval_s: 60,
            multiplier: 2.0,
            max_elapsed_time_s: 300,
            behaviour: Behaviour::Failover,
            models: vec![LlmModel::default()],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LlmModel {
    pub model: ModelId,
    pub priority: usize,
    pub api_key: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub prompt_override: Option<String>,
}

impl Default for LlmModel {
    fn default() -> Self {
        LlmModel {
            model: ModelId::Claude4Sonnet,
            priority: 1,
            api_key: None,
            max_tokens: Some(1500),
            temperature: Some(0.5),
            prompt_override: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[allow(unused)]
pub struct Settings {
    pub files: CrawlOptions,
    #[serde(default)]
    pub llm_settings: LlmSettings,
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
                Environment::with_prefix("AUTODOC")
                    .try_parsing(true)
                    .separator(".")
                    .list_separator(",")
                    .with_list_parse_key("llm_settings")
                    .with_list_parse_key("files.include_patterns")
                    .with_list_parse_key("files.exclude_patterns"),
            )
            .build()?;

        config.try_deserialize()
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        Self::with_config_builder(|builder| builder)
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
                let file = File::create(&path)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    #[serial]
    fn test_crawl_options_default() {
        let options = CrawlOptions::default();
        assert_eq!(options.max_depth, None);
        assert!(!options.include_hidden);
        assert!(options.include_patterns.is_empty());
        assert!(options.exclude_patterns.is_empty());
        assert!(!options.git_mode);
    }

    #[test]
    #[serial]
    fn test_crawl_options_equality() {
        let options1 = CrawlOptions {
            max_depth: Some(5),
            include_hidden: true,
            include_patterns: vec!["*.rs".to_string(), "*.toml".to_string()],
            exclude_patterns: vec!["target/".to_string()],
            git_mode: true,
        };

        let options2 = CrawlOptions {
            max_depth: Some(5),
            include_hidden: true,
            include_patterns: vec!["*.rs".to_string(), "*.toml".to_string()],
            exclude_patterns: vec!["target/".to_string()],
            git_mode: true,
        };

        assert_eq!(options1, options2);
    }

    #[test]
    #[serial]
    fn test_crawl_options_serialization() {
        let options = CrawlOptions {
            max_depth: Some(3),
            include_hidden: false,
            include_patterns: vec!["*.md".to_string()],
            exclude_patterns: vec!["*.tmp".to_string()],
            git_mode: true,
        };

        let serialized = serde_json::to_string(&options).unwrap();
        let deserialized: CrawlOptions = serde_json::from_str(&serialized).unwrap();

        assert_eq!(options, deserialized);
    }

    #[test]
    #[serial]
    fn test_llm_settings_serialization() {
        let settings = LlmSettings {
            ..Default::default()
        };

        let serialized = serde_json::to_string(&settings).unwrap();
        let deserialized: LlmSettings = serde_json::from_str(&serialized).unwrap();

        assert_eq!(settings, deserialized);
    }

    #[test]
    #[serial]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.files, CrawlOptions::default());
        assert!(!settings.llm_settings.models.is_empty());
    }

    #[test]
    #[serial]
    fn test_settings_serialization() {
        let settings = Settings {
            files: CrawlOptions {
                max_depth: Some(2),
                include_hidden: true,
                include_patterns: vec!["*.rs".to_string()],
                exclude_patterns: vec!["target/".to_string()],
                git_mode: false,
            },
            llm_settings: LlmSettings {
                behaviour: Behaviour::Failover,
                models: vec![
                    LlmModel {
                        model: ModelId::Llama32,
                        priority: 1,
                        api_key: Some("".to_string()),
                        max_tokens: Some(10),
                        temperature: Some(0.1),
                        prompt_override: None,
                    },
                    LlmModel {
                        model: ModelId::Claude35Haiku,
                        priority: 2,
                        api_key: Some("".to_string()),
                        max_tokens: Some(10),
                        temperature: Some(0.1),
                        prompt_override: None,
                    },
                ],
                ..LlmSettings::default()
            },
        };

        let serialized = serde_json::to_string(&settings).unwrap();
        let deserialized: Settings = serde_json::from_str(&serialized).unwrap();

        assert_eq!(settings, deserialized);
    }

    #[test]
    #[serial]
    fn test_from_env_with_defaults() {
        // Clear any existing AUTODOC environment variables
        clear_autodoc_env_vars();

        let result = Settings::from_env();
        if result.is_err() {
            eprintln!(
                "Error in from_env_with_defaults: {:?}",
                result.as_ref().unwrap_err()
            );
        }
        assert!(result.is_ok());

        let settings = result.unwrap();
        assert_eq!(settings.files, CrawlOptions::default());
        assert!(!settings.llm_settings.models.is_empty());
    }

    #[test]
    #[serial]
    fn test_from_env_with_environment_variables() {
        // Clear any existing AUTODOC environment variables
        clear_autodoc_env_vars();

        unsafe {
            env::set_var("AUTODOC.FILES.MAX_DEPTH", "5");
            env::set_var("AUTODOC.FILES.INCLUDE_HIDDEN", "true");
            env::set_var("AUTODOC.FILES.INCLUDE_PATTERNS", "*.rs,*.toml");
            env::set_var("AUTODOC.FILES.EXCLUDE_PATTERNS", "target/,*.tmp");
            env::set_var("AUTODOC.FILES.GIT_MODE", "true");
        }
        // Set environment variables

        let result = Settings::from_env();

        // Clean up environment variables
        clear_autodoc_env_vars();

        if result.is_err() {
            eprintln!("{:?}", result.as_ref().err())
        }
        assert!(result.is_ok());

        let settings = result.unwrap();

        assert_eq!(settings.files.max_depth, Some(5));
        assert!(settings.files.include_hidden);
        assert_eq!(settings.files.include_patterns, vec!["*.rs", "*.toml"]);
        assert_eq!(settings.files.exclude_patterns, vec!["target/", "*.tmp"]);
        assert!(settings.files.git_mode);
    }

    #[test]
    #[serial]
    fn test_from_file_with_valid_toml() {
        let toml_content = r#"
[files]
max_depth = 3
include_hidden = true
include_patterns = ["*.rs", "*.md"]
exclude_patterns = ["target/"]
git_mode = false

[llm_settings]
behaviour = "failover"

[[llm_settings.models]]
priority = 1
model = "claude-sonnet-4-20250514"
api_key = "test"

[[llm_settings.models]]
priority = 2
model = "gpt-4.1"
api_key = "test"
"#;

        let temp_file = NamedTempFile::with_suffix(".toml").unwrap();
        fs::write(temp_file.path(), toml_content).unwrap();

        let file_path = temp_file.path().to_str().unwrap();
        // Remove the .toml extension since config::File::with_name adds it
        let file_name = file_path.strip_suffix(".toml").unwrap();

        let result = Settings::from_file(file_name);
        if result.is_err() {
            eprint!("{:?}", result.as_ref().err())
        }
        assert!(result.is_ok());

        let settings = result.unwrap();
        assert_eq!(settings.files.max_depth, Some(3));
        assert!(settings.files.include_hidden);
        assert_eq!(settings.files.include_patterns, vec!["*.rs", "*.md"]);
        assert_eq!(settings.files.exclude_patterns, vec!["target/"]);
        assert!(!settings.files.git_mode);
        assert_eq!(settings.llm_settings.models.len(), 2);
        assert!(settings.llm_settings.models[0].model == ModelId::Claude4Sonnet);
        assert!(settings.llm_settings.models[1].model == ModelId::Gpt41);
    }

    #[test]
    #[serial]
    fn test_from_file_with_json() {
        let json_content = r#"{
 "files": {
    "max_depth": 2,
    "include_hidden": false,
    "include_patterns": ["*.json"],
    "exclude_patterns": ["node_modules/"],
    "git_mode": true
  },
  "llm_settings": {
    "behaviour": "failover",
    "max_retries": 5,
    "initial_interval_ms": 5000,
    "max_interval_s": 60,
    "multiplier": 2.0,
    "max_elapsed_time_s": 300,
    "models": [
      {
        "model": "claude-sonnet-4-20250514",
        "priority": 1,
        "api_key": null,
        "max_tokens": 1500,
        "temperature": 0.5,
        "prompt_override": null
      }
    ]
  }
}"#;

        let temp_file = NamedTempFile::with_suffix(".json").unwrap();
        fs::write(temp_file.path(), json_content).unwrap();

        let file_path = temp_file.path().to_str().unwrap();
        // Remove the .json extension since config::File::with_name adds it
        let file_name = file_path.strip_suffix(".json").unwrap();

        let result = Settings::from_file(file_name);

        if result.is_err() {
            eprint!("{:?}", result.as_ref().err())
        }
        assert!(result.is_ok());

        let settings = result.unwrap();
        assert_eq!(settings.files.max_depth, Some(2));
        assert!(!settings.files.include_hidden);
        assert_eq!(settings.files.include_patterns, vec!["*.json"]);
        assert_eq!(settings.files.exclude_patterns, vec!["node_modules/"]);
        assert!(settings.files.git_mode);
        assert_eq!(settings.llm_settings.models.len(), 1);
        assert!(settings.llm_settings.models[0].model == ModelId::Claude4Sonnet);
    }

    #[test]
    #[serial]
    fn test_from_file_with_env_override() {
        clear_autodoc_env_vars();

        let toml_content = r#"
[files]
max_depth = 3
include_hidden = false
git_mode = false
"#;

        let temp_file = NamedTempFile::with_suffix(".toml").unwrap();
        fs::write(temp_file.path(), toml_content).unwrap();

        // Set environment variable that should override file setting
        unsafe { env::set_var("AUTODOC.FILES.INCLUDE_HIDDEN", "true") };
        unsafe { env::set_var("AUTODOC.FILES.GIT_MODE", "true") };

        let file_path = temp_file.path().to_str().unwrap();
        let file_name = file_path.strip_suffix(".toml").unwrap();

        let result = Settings::from_file(file_name);

        // Clean up
        clear_autodoc_env_vars();

        assert!(result.is_ok());
        let settings = result.unwrap();

        // Environment variables should override file values
        assert_eq!(settings.files.max_depth, Some(3)); // from file
        assert!(settings.files.include_hidden); // overridden by env
        assert!(settings.files.git_mode); // overridden by env
    }

    #[test]
    #[serial]
    fn test_from_file_with_partial_config() {
        let toml_content = r#"
[files]
max_depth = 1
"#;

        let temp_file = NamedTempFile::with_suffix(".toml").unwrap();
        fs::write(temp_file.path(), toml_content).unwrap();

        let file_path = temp_file.path().to_str().unwrap();
        let file_name = file_path.strip_suffix(".toml").unwrap();

        let result = Settings::from_file(file_name);
        if result.is_err() {
            eprintln!(
                "Error in from_file_with_partial_config: {:?}",
                result.as_ref().unwrap_err()
            );
            eprintln!("File path: {}", file_path);
            eprintln!("File name: {}", file_name);
        }
        assert!(result.is_ok());

        let settings = result.unwrap();
        assert_eq!(settings.files.max_depth, Some(1));
        // Other fields should have default values
        assert!(!settings.files.include_hidden);
        assert!(settings.files.include_patterns.is_empty());
        assert!(settings.files.exclude_patterns.is_empty());
        assert!(!settings.files.git_mode);
        assert!(!settings.llm_settings.models.is_empty());
    }

    #[test]
    #[serial]
    fn test_invalid_env_var_types() {
        clear_autodoc_env_vars();

        // Set invalid values
        unsafe { env::set_var("AUTODOC.FILES.MAX_DEPTH", "not_a_number") };
        unsafe { env::set_var("AUTODOC.FILES.INCLUDE_HIDDEN", "not_a_bool") };

        let result = Settings::from_env();

        clear_autodoc_env_vars();

        // Should fail due to invalid type conversion
        assert!(result.is_err());
    }

    // #[test]
    // #[serial]
    // fn test_llm_settings_from_env_json_string() {
    //     clear_autodoc_env_vars();

    //     unsafe {
    //         // This approach works - JSON string
    //         env::set_var("AUTODOC.LLM_SETTINGS", r#"[{"test":true},{"test":false},{"test":true}]"#);
    //     }

    //     let result = Settings::from_env();
    //     clear_autodoc_env_vars();

    //     if result.is_err() {
    //         eprintln!("{:?}", result.as_ref().err())
    //     }

    //     assert!(result.is_ok());
    //     let settings = result.unwrap();
    //     assert_eq!(settings.llm_settings.len(), 3);
    //     assert!(settings.llm_settings[0].test);
    //     assert!(!settings.llm_settings[1].test);
    //     assert!(settings.llm_settings[2].test);
    // }

    #[test]
    #[serial]
    fn test_llm_settings_from_env_comma_separated_bools() {
        clear_autodoc_env_vars();

        unsafe {
            // Alternative: comma-separated boolean values
            env::set_var("AUTODOC.LLM_SETTINGS", "true,false,true");
        }

        let result = Settings::from_env();
        clear_autodoc_env_vars();

        // This might work if you implement custom deserialization
        // For now, this will likely fail since it expects JSON
        println!("Comma-separated result: {:?}", result);
    }
    // Helper function to clear all AUTODOC environment variables
    fn clear_autodoc_env_vars() {
        let vars_to_clear = [
            "AUTODOC.FILES.MAX_DEPTH",
            "AUTODOC.FILES.INCLUDE_HIDDEN",
            "AUTODOC.FILES.INCLUDE_PATTERNS",
            "AUTODOC.FILES.EXCLUDE_PATTERNS",
            "AUTODOC.FILES.GIT_MODE",
            "AUTODOC.LLM_SETTINGS",
        ];

        for var in &vars_to_clear {
            unsafe { env::remove_var(var) };
        }

        for (key, _) in env::vars() {
            if key.starts_with("AUTODOC.LLM_SETTINGS.") || key.starts_with("AUTODOC.") {
                unsafe {
                    env::remove_var(key);
                }
            }
        }
    }
}

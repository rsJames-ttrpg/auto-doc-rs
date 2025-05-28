use crate::analysis::summary::{
    DirectoryAnalysis, FileAnalysis, Interface, InterfaceType, ProjectAnalysis,
};
use std::fmt::{Display, Formatter, Result};

impl Display for InterfaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let type_str = match self {
            InterfaceType::Function => "🔧 Function",
            InterfaceType::Struct => "📦 Struct",
            InterfaceType::Trait => "🎯 Trait",
            InterfaceType::Module => "📁 Module",
            InterfaceType::Api => "🌐 API",
            InterfaceType::Configuration => "⚙️ Configuration",
            InterfaceType::DataModel => "🗄️ Data Model",
        };
        write!(f, "{}", type_str)
    }
}

impl Display for Interface {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "- **{}** (`{}`)\n  {}",
            self.name, self.interface_type, self.description
        )
    }
}

impl Display for FileAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "# 📄 File Analysis: `{}`", self.file_path.display())?;
        writeln!(f)?;
        writeln!(f, "**Type:** `{}`", self.file_type)?;
        writeln!(f)?;
        writeln!(f, "## Summary")?;
        writeln!(f, "{}", self.summary)?;
        writeln!(f)?;

        if !self.external_dependencies.is_empty() {
            writeln!(f, "## 📚 External Dependencies")?;
            for dep in &self.external_dependencies {
                writeln!(f, "- `{}`", dep)?;
            }
            writeln!(f)?;
        }

        if !self.public_interfaces.is_empty() {
            writeln!(f, "## 🔌 Public Interfaces")?;
            for interface in &self.public_interfaces {
                writeln!(f, "{}", interface)?;
            }
        }

        Ok(())
    }
}

impl Display for DirectoryAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "# 📁 Directory: `{}`", self.directory_path.display())?;
        writeln!(f)?;
        writeln!(f, "**Depth Level:** {}", self.depth_level)?;
        writeln!(f)?;
        writeln!(f, "## Summary")?;
        writeln!(f, "{}", self.summary)?;
        writeln!(f)?;

        if !self.key_components.is_empty() {
            writeln!(f, "## 🎯 Key Components")?;
            for component in &self.key_components {
                writeln!(f, "- **{}**", component)?;
            }
            writeln!(f)?;
        }

        if !self.child_summaries.is_empty() {
            writeln!(f, "## 📋 Child Summaries")?;
            for (i, summary) in self.child_summaries.iter().enumerate() {
                writeln!(f, "{}. {}", i + 1, summary)?;
            }
            writeln!(f)?;
        }

        if !self.external_dependencies.is_empty() {
            writeln!(f, "## 📚 External Dependencies")?;
            for dep in &self.external_dependencies {
                writeln!(f, "- `{}`", dep)?;
            }
            writeln!(f)?;
        }

        if !self.public_interfaces.is_empty() {
            writeln!(f, "## 🔌 Public Interfaces")?;
            for interface in &self.public_interfaces {
                let interface_string = interface.to_string();
                let interface_lines: Vec<&str> = interface_string.lines().collect();
                for line in interface_lines {
                    writeln!(f, "{}", line)?;
                }
            }
        }

        Ok(())
    }
}

impl Display for ProjectAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "# 🚀 Project Analysis")?;
        writeln!(f)?;
        writeln!(f, "## Overview")?;
        writeln!(f, "{}", self.project_overview)?;
        writeln!(f)?;

        writeln!(f, "## 🏗️ Architecture")?;
        writeln!(f, "{}", self.architecture_summary)?;
        writeln!(f)?;

        if !self.core_technologies.is_empty() {
            writeln!(f, "## 🛠️ Core Technologies")?;
            for tech in &self.core_technologies {
                writeln!(f, "- **{}**", tech)?;
            }
            writeln!(f)?;
        }

        if !self.main_interfaces.is_empty() {
            writeln!(f, "## 🔌 Main Interfaces")?;
            for interface in &self.main_interfaces {
                writeln!(f, "{}", interface)?;
            }
            writeln!(f)?;
        }

        if !self.development_considerations.is_empty() {
            writeln!(f, "## 🔧 Development Considerations")?;
            for consideration in &self.development_considerations {
                writeln!(f, "- {}", consideration)?;
            }
            writeln!(f)?;
        }

        if !self.extension_points.is_empty() {
            writeln!(f, "## 🔗 Extension Points")?;
            for point in &self.extension_points {
                writeln!(f, "- {}", point)?;
            }
            writeln!(f)?;
        }

        if !self.risk_factors.is_empty() {
            writeln!(f, "## ⚠️ Risk Factors")?;
            for risk in &self.risk_factors {
                writeln!(f, "{}", risk)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_interface_display() {
        let interface = Interface {
            name: "parse_config".to_string(),
            interface_type: InterfaceType::Function,
            description: "Parses configuration from TOML file".to_string(),
        };

        let expected = "- **parse_config** (`🔧 Function`)\n  Parses configuration from TOML file";
        assert_eq!(interface.to_string(), expected);
    }

    #[test]
    fn test_file_analysis_display() {
        let analysis = FileAnalysis {
            file_path: PathBuf::from("src/config.rs"),
            file_type: "rs".to_string(),
            summary: "Configuration management module. Handles parsing and validation of application settings.".to_string(),
            external_dependencies: vec!["serde".to_string(), "toml".to_string()],
            public_interfaces: vec![
                Interface {
                    name: "Config".to_string(),
                    interface_type: InterfaceType::Struct,
                    description: "Main configuration struct".to_string(),
                }
            ],
        };

        let output = analysis.to_string();
        assert!(output.contains("# 📄 File Analysis: `src/config.rs`"));
        assert!(output.contains("**Type:** `rs`"));
        assert!(output.contains("## 📚 External Dependencies"));
        assert!(output.contains("- `serde`"));
    }
}

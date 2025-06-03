# 📄 File Analysis: `./src/settings.rs`

**Type:** `rs`

## Summary
This file defines the configuration system for a documentation reader application, providing structures to manage projects and their components with support for multiple configuration file formats. It handles loading configuration from files and environment variables, with methods to query and access project/component information.

## 📚 External Dependencies
- `std::collections::HashMap`
- `std::fs::File`
- `std::io`
- `std::path::PathBuf`
- `clap::ValueEnum`
- `serde`
- `config`
- `tracing`
- `dirs`
- `serde_json`
- `toml`
- `serde_yaml`

## 🔌 Public Interfaces
- **Component** (`📦 Struct`)
  Represents a project component with a relative path, used to define location of documentation within a project
- **Project** (`📦 Struct`)
  Represents a project with a location and collection of named components, manages project structure and component mapping
- **Settings** (`📦 Struct`)
  Main configuration structure containing a map of named projects, serves as the root configuration object
- **FileType** (`📦 Struct`)
  Enum defining supported configuration file formats (Json, Toml, Yaml) for serialization
- **from_env** (`🔧 Function`)
  Creates Settings from environment variables and optional config file, with fallback to default config directory
- **from_file** (`🔧 Function`)
  Creates Settings from a specified configuration file path with environment variable override support
- **write_default_config** (`🔧 Function`)
  Writes default configuration to file or stdout in specified format (JSON, TOML, or YAML)
- **get_component_names** (`🔧 Function`)
  Returns list of all component names across all projects in the configuration
- **get_project_names** (`🔧 Function`)
  Returns list of all project names in the configuration
- **get_project** (`🔧 Function`)
  Retrieves a specific project by name, returns optional reference to Project
- **get_component** (`🔧 Function`)
  Retrieves a specific component by project and component name, returns optional reference to Component
- **get_component_path** (`🔧 Function`)
  Resolves full filesystem path for a component by combining project location with component relative path
- **get_project_component_names** (`🔧 Function`)
  Returns list of component names for a specific project

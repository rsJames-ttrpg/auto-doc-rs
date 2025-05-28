# 📄 File Analysis: `./src/analysis/display.rs`

**Type:** `rs`

## Summary
Implements Display trait for analysis data structures to provide formatted output. Provides human-readable string representations for InterfaceType, Interface, FileAnalysis, DirectoryAnalysis, and ProjectAnalysis structs with emoji icons and markdown formatting.

## 📚 External Dependencies
- `std::fmt`
- `crate::analysis::summary`

## 🔌 Public Interfaces
- **Display for InterfaceType** (`🎯 Trait`)
  Formats InterfaceType enum variants with emoji icons and descriptive text
- **Display for Interface** (`🎯 Trait`)
  Formats Interface struct with name, type, and description in markdown format
- **Display for FileAnalysis** (`🎯 Trait`)
  Formats FileAnalysis struct as markdown document with sections for summary, dependencies, and interfaces
- **Display for DirectoryAnalysis** (`🎯 Trait`)
  Formats DirectoryAnalysis struct as markdown with directory path, components, and child summaries
- **Display for ProjectAnalysis** (`🎯 Trait`)
  Formats ProjectAnalysis struct as comprehensive markdown report with overview, architecture, and technologies

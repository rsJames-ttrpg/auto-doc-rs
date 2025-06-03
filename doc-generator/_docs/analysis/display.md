# 📄 File Analysis: `./src/analysis/display.rs`

**Type:** `rs`

## Summary
This file implements Display traits for analysis data structures, providing formatted output for file, directory, and project analysis results. It handles the presentation layer for analysis summaries with emoji-enhanced formatting.

## 📚 External Dependencies
- `std::fmt`
- `crate::analysis::summary`

## 🔌 Public Interfaces
- **Display for InterfaceType** (`🎯 Trait`)
  Provides formatted display of interface types with emoji prefixes for visual distinction
- **Display for Interface** (`🎯 Trait`)
  Formats interface information with name, type, and description in markdown-style output
- **Display for FileAnalysis** (`🎯 Trait`)
  Renders complete file analysis reports including summary, dependencies, and public interfaces
- **Display for DirectoryAnalysis** (`🎯 Trait`)
  Formats directory analysis with key components, child summaries, and aggregated information
- **Display for ProjectAnalysis** (`🎯 Trait`)
  Provides comprehensive project-level analysis formatting including architecture, technologies, and risk factors

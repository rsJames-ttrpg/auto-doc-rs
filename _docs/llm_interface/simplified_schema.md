# 📄 File Analysis: `./src/llm_interface/simplified_schema.rs`

**Type:** `rs`

## Summary
This file implements a JSON Schema to SimplifiedSchema converter for LLM interfaces. It transforms complex JSON Schema documents into a simplified format suitable for LLM consumption, handling schema references, definitions, type flattening, and validation constraints while maintaining essential schema information.

## 📚 External Dependencies
- `serde`
- `serde_json`
- `std::collections::HashMap`
- `thiserror`

## 🔌 Public Interfaces
- **SchemaType** (`🗄️ Data Model`)
  Enum defining supported schema types (String, Number, Integer, Boolean, Array, Object) for simplified schema representation
- **SimplifiedSchema** (`📦 Struct`)
  Main data structure representing a simplified JSON schema with all validation constraints and metadata fields
- **ConversionError** (`🗄️ Data Model`)
  Error enum for handling various conversion failures including unsupported types, invalid schemas, missing fields, and reference resolution errors
- **JsonSchemaConverter** (`📦 Struct`)
  Main converter struct that transforms JSON Schema documents into SimplifiedSchema format, handling references, definitions, and schema flattening
- **JsonSchemaConverter::new** (`🔧 Function`)
  Creates a new instance of JsonSchemaConverter with empty definitions cache
- **JsonSchemaConverter::convert** (`🔧 Function`)
  Static method that converts a JSON Schema Value into a SimplifiedSchema, handling all preprocessing and conversion logic

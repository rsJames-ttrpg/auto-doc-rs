use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SchemaType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedSchema {
    #[serde(rename = "type")]
    pub schema_type: SchemaType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,

    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,

    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<String>,

    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub min_items: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, SimplifiedSchema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<String>,

    #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<String>,

    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<String>,

    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,

    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<SimplifiedSchema>>,

    #[serde(rename = "propertyOrdering", skip_serializing_if = "Option::is_none")]
    pub property_ordering: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<SimplifiedSchema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Unsupported schema type: {0}")]
    UnsupportedType(String),
    #[error("Invalid JSON Schema: {0}")]
    InvalidSchema(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Reference resolution error: {0}")]
    RefResolutionError(String),
}

pub struct JsonSchemaConverter {
    definitions: HashMap<String, Value>,
}

impl JsonSchemaConverter {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    pub fn convert(json_schema: &Value) -> Result<SimplifiedSchema, ConversionError> {
        let mut converter = Self::new();
        converter.extract_definitions(json_schema);
        converter.convert_schema(json_schema)
    }

    /// Extract definitions from $defs or definitions
    fn extract_definitions(&mut self, schema: &Value) {
        if let Some(obj) = schema.as_object() {
            // Handle $defs (newer JSON Schema)
            if let Some(defs) = obj.get("$defs").and_then(|v| v.as_object()) {
                for (key, value) in defs {
                    self.definitions.insert(key.clone(), value.clone());
                }
            }

            // Handle definitions (older JSON Schema)
            if let Some(defs) = obj.get("definitions").and_then(|v| v.as_object()) {
                for (key, value) in defs {
                    self.definitions.insert(key.clone(), value.clone());
                }
            }
        }
    }

    /// Resolve $ref references
    fn resolve_ref(&self, ref_path: &str) -> Result<Value, ConversionError> {
        // Handle internal references like "#/$defs/veggie" or "#/definitions/veggie"
        if ref_path.starts_with("#/") {
            let parts: Vec<&str> = ref_path.split('/').collect();

            if parts.len() >= 3 {
                let def_type = parts[1]; // "$defs" or "definitions"
                let def_name = parts[2];

                if def_type == "$defs" || def_type == "definitions" {
                    if let Some(definition) = self.definitions.get(def_name) {
                        return Ok(definition.clone());
                    }
                }
            }
        }

        Err(ConversionError::RefResolutionError(format!(
            "Could not resolve reference: {}",
            ref_path
        )))
    }

    #[allow(clippy::only_used_in_recursion)]
    /// Remove JSON Schema specific fields that start with '$'
    fn clean_schema(&self, schema: &Value) -> Value {
        match schema {
            Value::Object(obj) => {
                let mut cleaned = Map::new();
                for (key, value) in obj {
                    // Skip fields that start with '$'
                    if !key.starts_with('$') {
                        cleaned.insert(key.clone(), self.clean_schema(value));
                    }
                }
                Value::Object(cleaned)
            }
            Value::Array(arr) => Value::Array(arr.iter().map(|v| self.clean_schema(v)).collect()),
            _ => schema.clone(),
        }
    }

    fn convert_schema(&self, schema: &Value) -> Result<SimplifiedSchema, ConversionError> {
        // Handle $ref first, before any other processing
        if let Some(obj) = schema.as_object() {
            if let Some(ref_path) = obj.get("$ref").and_then(|v| v.as_str()) {
                let resolved = self.resolve_ref(ref_path)?;
                return self.convert_schema(&resolved);
            }
        }

        // Handle oneOf, allOf, anyOf by flattening them
        if let Some(obj) = schema.as_object() {
            // Handle oneOf - convert to the first valid option or merge enum values
            if let Some(one_of) = obj.get("oneOf").and_then(|v| v.as_array()) {
                return self.flatten_one_of(one_of, obj);
            }

            // Handle allOf - merge all schemas together
            if let Some(all_of) = obj.get("allOf").and_then(|v| v.as_array()) {
                return self.flatten_all_of(all_of, obj);
            }

            // Handle anyOf - convert to the first option (similar to oneOf)
            if let Some(any_of) = obj.get("anyOf").and_then(|v| v.as_array()) {
                return self.flatten_any_of(any_of, obj);
            }
        }

        // Clean the schema to remove $ fields for type determination
        let cleaned_schema = self.clean_schema(schema);
        let schema_obj = cleaned_schema.as_object().ok_or_else(|| {
            ConversionError::InvalidSchema("Schema must be an object".to_string())
        })?;

        // Keep reference to original schema for accessing items/properties with $ref
        let original_schema_obj = schema.as_object().ok_or_else(|| {
            ConversionError::InvalidSchema("Original schema must be an object".to_string())
        })?;

        // Determine the type from cleaned schema
        let schema_type = Self::determine_type(schema_obj)?;

        let mut gemini_schema = SimplifiedSchema {
            schema_type: schema_type.clone(),
            format: None,
            title: None,
            description: None,
            nullable: None,
            enum_values: None,
            max_items: None,
            min_items: None,
            properties: None,
            required: None,
            min_properties: None,
            max_properties: None,
            min_length: None,
            max_length: None,
            pattern: None,
            example: None,
            any_of: None,
            property_ordering: None,
            default: None,
            items: None,
            minimum: None,
            maximum: None,
        };

        // Set basic fields from cleaned schema
        if let Some(title) = schema_obj.get("title").and_then(|v| v.as_str()) {
            gemini_schema.title = Some(title.to_string());
        }

        if let Some(description) = schema_obj.get("description").and_then(|v| v.as_str()) {
            gemini_schema.description = Some(description.to_string());
        }

        if let Some(example) = schema_obj.get("example") {
            gemini_schema.example = Some(example.clone());
        }

        if let Some(default) = schema_obj.get("default") {
            gemini_schema.default = Some(default.clone());
        }

        // Handle nullable
        if let Some(nullable) = schema_obj.get("nullable").and_then(|v| v.as_bool()) {
            gemini_schema.nullable = Some(nullable);
        }

        // Type-specific conversions
        match schema_type {
            SchemaType::String => {
                Self::convert_string_fields(schema_obj, &mut gemini_schema)?;
            }
            SchemaType::Number | SchemaType::Integer => {
                Self::convert_number_fields(schema_obj, &mut gemini_schema)?;
            }
            SchemaType::Array => {
                // Use original schema for items that might contain $ref
                self.convert_array_fields(original_schema_obj, &mut gemini_schema)?;
            }
            SchemaType::Object => {
                // Use original schema for properties that might contain $ref
                self.convert_object_fields(original_schema_obj, &mut gemini_schema)?;
            }
            SchemaType::Boolean => {
                // Boolean type doesn't have additional fields
            }
        }

        Ok(gemini_schema)
    }

    /// Flatten oneOf into a single schema - merge enum values or use first option
    fn flatten_one_of(
        &self,
        one_of: &[Value],
        parent_obj: &Map<String, Value>,
    ) -> Result<SimplifiedSchema, ConversionError> {
        if one_of.is_empty() {
            return Err(ConversionError::InvalidSchema(
                "Empty oneOf array".to_string(),
            ));
        }

        // Check if all variants are string enums - if so, merge them
        let mut all_enum_values = Vec::new();
        let mut all_are_string_enums = true;

        for variant in one_of {
            if let Some(variant_obj) = variant.as_object() {
                if let Some(type_str) = variant_obj.get("type").and_then(|v| v.as_str()) {
                    if type_str == "string" {
                        if let Some(enum_vals) = variant_obj.get("enum").and_then(|v| v.as_array())
                        {
                            for enum_val in enum_vals {
                                if let Some(val_str) = enum_val.as_str() {
                                    all_enum_values.push(val_str.to_string());
                                } else {
                                    all_are_string_enums = false;
                                    break;
                                }
                            }
                        } else {
                            all_are_string_enums = false;
                        }
                    } else {
                        all_are_string_enums = false;
                    }
                } else {
                    all_are_string_enums = false;
                }
            } else {
                all_are_string_enums = false;
            }

            if !all_are_string_enums {
                break;
            }
        }

        if all_are_string_enums && !all_enum_values.is_empty() {
            // Create a merged enum schema
            let mut merged_schema = SimplifiedSchema {
                schema_type: SchemaType::String,
                format: Some("enum".to_string()),
                enum_values: Some(all_enum_values),
                title: None,
                description: None,
                nullable: None,
                max_items: None,
                min_items: None,
                properties: None,
                required: None,
                min_properties: None,
                max_properties: None,
                min_length: None,
                max_length: None,
                pattern: None,
                example: None,
                any_of: None,
                property_ordering: None,
                default: None,
                items: None,
                minimum: None,
                maximum: None,
            };

            // Copy description from parent if available
            if let Some(description) = parent_obj.get("description").and_then(|v| v.as_str()) {
                merged_schema.description = Some(description.to_string());
            }

            return Ok(merged_schema);
        }

        // Otherwise, use the first variant
        self.convert_schema(&one_of[0])
    }

    /// Flatten allOf by merging all schemas together
    fn flatten_all_of(
        &self,
        all_of: &[Value],
        _parent_obj: &Map<String, Value>,
    ) -> Result<SimplifiedSchema, ConversionError> {
        if all_of.is_empty() {
            return Err(ConversionError::InvalidSchema(
                "Empty allOf array".to_string(),
            ));
        }

        // For allOf with a single $ref, just resolve the reference
        if all_of.len() == 1 {
            return self.convert_schema(&all_of[0]);
        }

        // For multiple schemas, try to merge them intelligently
        // This is complex, so for now, use the first non-reference schema
        for schema in all_of {
            if let Some(obj) = schema.as_object() {
                if !obj.contains_key("$ref") {
                    return self.convert_schema(schema);
                }
            }
        }

        // If all are references, use the first one
        self.convert_schema(&all_of[0])
    }

    /// Flatten anyOf similar to oneOf
    fn flatten_any_of(
        &self,
        any_of: &[Value],
        parent_obj: &Map<String, Value>,
    ) -> Result<SimplifiedSchema, ConversionError> {
        self.flatten_one_of(any_of, parent_obj)
    }

    fn determine_type(schema: &Map<String, Value>) -> Result<SchemaType, ConversionError> {
        if let Some(type_value) = schema.get("type") {
            match type_value {
                Value::String(type_str) => match type_str.as_str() {
                    "string" => Ok(SchemaType::String),
                    "number" => Ok(SchemaType::Number),
                    "integer" => Ok(SchemaType::Integer),
                    "boolean" => Ok(SchemaType::Boolean),
                    "array" => Ok(SchemaType::Array),
                    "object" => Ok(SchemaType::Object),
                    "null" => Err(ConversionError::UnsupportedType(
                        "null type not directly supported".to_string(),
                    )),
                    other => Err(ConversionError::UnsupportedType(other.to_string())),
                },
                Value::Array(types) => {
                    // Handle union types like ["string", "null"]
                    let non_null_types: Vec<_> = types
                        .iter()
                        .filter_map(|v| v.as_str())
                        .filter(|&s| s != "null")
                        .collect();

                    if non_null_types.len() == 1 {
                        match non_null_types[0] {
                            "string" => Ok(SchemaType::String),
                            "number" => Ok(SchemaType::Number),
                            "integer" => Ok(SchemaType::Integer),
                            "boolean" => Ok(SchemaType::Boolean),
                            "array" => Ok(SchemaType::Array),
                            "object" => Ok(SchemaType::Object),
                            other => Err(ConversionError::UnsupportedType(other.to_string())),
                        }
                    } else {
                        Err(ConversionError::UnsupportedType(
                            "Multiple non-null types not supported".to_string(),
                        ))
                    }
                }
                _ => Err(ConversionError::InvalidSchema(
                    "Type must be string or array".to_string(),
                )),
            }
        } else if schema.contains_key("properties") || schema.contains_key("additionalProperties") {
            Ok(SchemaType::Object)
        } else if schema.contains_key("items") {
            Ok(SchemaType::Array)
        } else if schema.contains_key("enum") {
            Ok(SchemaType::String)
        } else if schema.is_empty() {
            Err(ConversionError::InvalidSchema("Empty schema after cleaning - this might be a $ref-only schema that wasn't resolved".to_string()))
        } else {
            Err(ConversionError::MissingField("type".to_string()))
        }
    }

    fn convert_string_fields(
        schema: &Map<String, Value>,
        gemini_schema: &mut SimplifiedSchema,
    ) -> Result<(), ConversionError> {
        // Handle format
        if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
            match format {
                "date-time" | "enum" => {
                    gemini_schema.format = Some(format.to_string());
                }
                _ => {
                    // Ignore unsupported formats
                }
            }
        }

        // Handle enum
        if let Some(enum_values) = schema.get("enum").and_then(|v| v.as_array()) {
            let string_enums: Result<Vec<String>, _> = enum_values
                .iter()
                .map(|v| {
                    v.as_str()
                        .ok_or_else(|| {
                            ConversionError::InvalidSchema(
                                "Enum values must be strings".to_string(),
                            )
                        })
                        .map(|s| s.to_string())
                })
                .collect();
            gemini_schema.enum_values = Some(string_enums?);
            gemini_schema.format = Some("enum".to_string());
        }

        // Handle string constraints
        if let Some(min_length) = schema.get("minLength").and_then(|v| v.as_u64()) {
            gemini_schema.min_length = Some(min_length.to_string());
        }

        if let Some(max_length) = schema.get("maxLength").and_then(|v| v.as_u64()) {
            gemini_schema.max_length = Some(max_length.to_string());
        }

        if let Some(pattern) = schema.get("pattern").and_then(|v| v.as_str()) {
            gemini_schema.pattern = Some(pattern.to_string());
        }

        Ok(())
    }

    fn convert_number_fields(
        schema: &Map<String, Value>,
        gemini_schema: &mut SimplifiedSchema,
    ) -> Result<(), ConversionError> {
        // Handle format for numbers
        if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
            match format {
                "float" | "double" if gemini_schema.schema_type == SchemaType::Number => {
                    gemini_schema.format = Some(format.to_string());
                }
                "int32" | "int64" if gemini_schema.schema_type == SchemaType::Integer => {
                    gemini_schema.format = Some(format.to_string());
                }
                _ => {
                    // Ignore unsupported formats
                }
            }
        }

        // Handle numeric constraints
        if let Some(minimum) = schema.get("minimum").and_then(|v| v.as_f64()) {
            gemini_schema.minimum = Some(minimum);
        }

        if let Some(maximum) = schema.get("maximum").and_then(|v| v.as_f64()) {
            gemini_schema.maximum = Some(maximum);
        }

        Ok(())
    }

    fn convert_array_fields(
        &self,
        schema: &Map<String, Value>,
        gemini_schema: &mut SimplifiedSchema,
    ) -> Result<(), ConversionError> {
        // Handle items - process from original schema that may contain $ref
        if let Some(items) = schema.get("items") {
            let converted_items = self.convert_schema(items)?;
            gemini_schema.items = Some(Box::new(converted_items));
        }

        // Handle array constraints
        if let Some(min_items) = schema.get("minItems").and_then(|v| v.as_u64()) {
            gemini_schema.min_items = Some(min_items.to_string());
        }

        if let Some(max_items) = schema.get("maxItems").and_then(|v| v.as_u64()) {
            gemini_schema.max_items = Some(max_items.to_string());
        }

        Ok(())
    }

    fn convert_object_fields(
        &self,
        schema: &Map<String, Value>,
        gemini_schema: &mut SimplifiedSchema,
    ) -> Result<(), ConversionError> {
        // Handle properties - process from original schema that may contain $ref
        if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
            let mut converted_properties = HashMap::new();
            for (key, value) in properties {
                converted_properties.insert(key.clone(), self.convert_schema(value)?);
            }
            gemini_schema.properties = Some(converted_properties);
        }

        // Handle required fields
        if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
            let required_strings: Result<Vec<String>, _> = required
                .iter()
                .map(|v| {
                    v.as_str()
                        .ok_or_else(|| {
                            ConversionError::InvalidSchema(
                                "Required field names must be strings".to_string(),
                            )
                        })
                        .map(|s| s.to_string())
                })
                .collect();
            gemini_schema.required = Some(required_strings?);
        }

        // Handle object constraints
        if let Some(min_properties) = schema.get("minProperties").and_then(|v| v.as_u64()) {
            gemini_schema.min_properties = Some(min_properties.to_string());
        }

        if let Some(max_properties) = schema.get("maxProperties").and_then(|v| v.as_u64()) {
            gemini_schema.max_properties = Some(max_properties.to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_string_schema() {
        let json_schema = json!({
            "type": "string",
            "title": "Name",
            "description": "A person's name"
        });

        let result: SimplifiedSchema = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::String);
        assert_eq!(result.title, Some("Name".to_string()));
        assert_eq!(result.description, Some("A person's name".to_string()));
    }

    #[test]
    fn test_enum_schema() {
        let json_schema = json!({
            "type": "string",
            "enum": ["red", "green", "blue"]
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::String);
        assert_eq!(result.format, Some("enum".to_string()));
        assert_eq!(
            result.enum_values,
            Some(vec![
                "red".to_string(),
                "green".to_string(),
                "blue".to_string()
            ])
        );
    }

    #[test]
    fn test_object_schema() {
        let json_schema = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                },
                "age": {
                    "type": "integer",
                    "minimum": 0
                }
            },
            "required": ["name"]
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::Object);
        assert!(result.properties.is_some());
        assert_eq!(result.required, Some(vec!["name".to_string()]));

        let properties = result.properties.unwrap();
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("age"));

        let age_schema = &properties["age"];
        assert_eq!(age_schema.schema_type, SchemaType::Integer);
        assert_eq!(age_schema.minimum, Some(0.0));
    }

    #[test]
    fn test_array_schema() {
        let json_schema = json!({
            "type": "array",
            "items": {
                "type": "string"
            },
            "minItems": 1,
            "maxItems": 10
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::Array);
        assert_eq!(result.min_items, Some("1".to_string()));
        assert_eq!(result.max_items, Some("10".to_string()));
        assert!(result.items.is_some());

        let items = result.items.unwrap();
        assert_eq!(items.schema_type, SchemaType::String);
    }

    #[test]
    fn test_nullable_type() {
        let json_schema = json!({
            "type": ["string", "null"],
            "title": "Optional String"
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::String);
        assert_eq!(result.title, Some("Optional String".to_string()));
    }

    #[test]
    fn test_simple_ref_resolution() {
        let json_schema = json!({
            "type": "object",
            "properties": {
                "user": { "$ref": "#/$defs/person" }
            },
            "$defs": {
                "person": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" }
                    }
                }
            }
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::Object);
        let properties = result.properties.unwrap();
        let user = &properties["user"];
        assert_eq!(user.schema_type, SchemaType::Object);

        let user_props = user.properties.as_ref().unwrap();
        assert!(user_props.contains_key("name"));
        assert_eq!(user_props["name"].schema_type, SchemaType::String);
    }

    #[test]
    fn test_ref_resolution() {
        let json_schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "vegetables": {
                    "type": "array",
                    "items": { "$ref": "#/$defs/veggie" }
                }
            },
            "$defs": {
                "veggie": {
                    "type": "object",
                    "required": ["veggieName", "veggieLike"],
                    "properties": {
                        "veggieName": {
                            "type": "string",
                            "description": "The name of the vegetable."
                        },
                        "veggieLike": {
                            "type": "boolean",
                            "description": "Do I like this vegetable?"
                        }
                    }
                }
            }
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::Object);

        let properties = result.properties.unwrap();
        let vegetables = &properties["vegetables"];
        assert_eq!(vegetables.schema_type, SchemaType::Array);

        let items = vegetables.items.as_ref().unwrap();
        assert_eq!(items.schema_type, SchemaType::Object);

        let veggie_props = items.properties.as_ref().unwrap();
        assert!(veggie_props.contains_key("veggieName"));
        assert!(veggie_props.contains_key("veggieLike"));

        assert_eq!(
            items.required,
            Some(vec!["veggieName".to_string(), "veggieLike".to_string()])
        );
    }

    #[test]
    fn test_clean_schema_removes_dollar_fields() {
        let json_schema = json!({
            "$id": "https://example.com/test.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "title": "Test Schema"
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::String);
        assert_eq!(result.title, Some("Test Schema".to_string()));
    }

    #[test]
    fn test_full_example_schema() {
        let json_schema = json!({
            "$id": "https://example.com/arrays.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "description": "Arrays of strings and objects",
            "title": "Arrays",
            "type": "object",
            "properties": {
                "fruits": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "vegetables": {
                    "type": "array",
                    "items": { "$ref": "#/$defs/veggie" }
                }
            },
            "$defs": {
                "veggie": {
                    "type": "object",
                    "required": ["veggieName", "veggieLike"],
                    "properties": {
                        "veggieName": {
                            "type": "string",
                            "description": "The name of the vegetable."
                        },
                        "veggieLike": {
                            "type": "boolean",
                            "description": "Do I like this vegetable?"
                        }
                    }
                }
            }
        });

        let result = JsonSchemaConverter::convert(&json_schema).unwrap();

        assert_eq!(result.schema_type, SchemaType::Object);
        assert_eq!(result.title, Some("Arrays".to_string()));
        assert_eq!(
            result.description,
            Some("Arrays of strings and objects".to_string())
        );

        let properties = result.properties.unwrap();

        // Check fruits array
        let fruits = &properties["fruits"];
        assert_eq!(fruits.schema_type, SchemaType::Array);
        let fruits_items = fruits.items.as_ref().unwrap();
        assert_eq!(fruits_items.schema_type, SchemaType::String);

        // Check vegetables array with resolved reference
        let vegetables = &properties["vegetables"];
        assert_eq!(vegetables.schema_type, SchemaType::Array);
        let veggie_items = vegetables.items.as_ref().unwrap();
        assert_eq!(veggie_items.schema_type, SchemaType::Object);

        let veggie_props = veggie_items.properties.as_ref().unwrap();
        assert_eq!(veggie_props.len(), 2);

        let veggie_name = &veggie_props["veggieName"];
        assert_eq!(veggie_name.schema_type, SchemaType::String);
        assert_eq!(
            veggie_name.description,
            Some("The name of the vegetable.".to_string())
        );

        let veggie_like = &veggie_props["veggieLike"];
        assert_eq!(veggie_like.schema_type, SchemaType::Boolean);
        assert_eq!(
            veggie_like.description,
            Some("Do I like this vegetable?".to_string())
        );

        assert_eq!(
            veggie_items.required,
            Some(vec!["veggieName".to_string(), "veggieLike".to_string()])
        );
    }
}

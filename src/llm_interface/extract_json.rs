use once_cell::sync::Lazy;
use regex::Regex;

// Regex patterns for extracting JSON from various formats
static JSON_CODE_BLOCK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"```(?:json)?\s*(\{.*?\})\s*```").unwrap());

static JSON_OBJECT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{.*\}").unwrap());

/// Attempts to extract valid JSON from LLM response text
pub fn extract_json_from_response(text: &str) -> Option<String> {
    // First, try to extract from code blocks
    if let Some(captures) = JSON_CODE_BLOCK.captures(text) {
        if let Some(json_match) = captures.get(1) {
            return Some(json_match.as_str().to_string());
        }
    }

    // If no code block, look for the first complete JSON object
    if let Some(json_match) = JSON_OBJECT.find(text) {
        return Some(json_match.as_str().to_string());
    }

    // If still nothing, try the original text (maybe it's already clean JSON)
    None
}

pub fn extract_json_aggressively(text: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    // Strategy 1: Code blocks
    for captures in JSON_CODE_BLOCK.captures_iter(text) {
        if let Some(json_match) = captures.get(1) {
            candidates.push(json_match.as_str().to_string());
        }
    }

    // Strategy 2: All JSON objects in text
    for json_match in JSON_OBJECT.find_iter(text) {
        candidates.push(json_match.as_str().to_string());
    }

    // Strategy 3: Try to find JSON by looking for balanced braces
    if let Some(start) = text.find('{') {
        let mut brace_count = 0;
        let mut end_pos = start;

        for (i, ch) in text[start..].char_indices() {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        end_pos = start + i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }

        if brace_count == 0 && end_pos > start {
            candidates.push(text[start..end_pos].to_string());
        }
    }

    // Remove duplicates and return
    candidates.sort();
    candidates.dedup();
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_extraction() {
        let response1 = r#"
Here's the analysis:
```json
{"name": "test", "value": 42}
```
Hope this helps!
        "#;

        let response2 = r#"
The result is: {"name": "test", "value": 42}
        "#;

        let response3 = r#"{"name": "test", "value": 42}"#;

        assert_eq!(
            extract_json_from_response(response1),
            Some(r#"{"name": "test", "value": 42}"#.to_string())
        );

        assert_eq!(
            extract_json_from_response(response2),
            Some(r#"{"name": "test", "value": 42}"#.to_string())
        );

        assert_eq!(
            extract_json_from_response(response3),
            Some(r#"{"name": "test", "value": 42}"#.to_string())
        );
    }
}

pub mod extract_json;
pub mod models;
use extract_json::{extract_json_aggressively, extract_json_from_response};
use llm::{
    builder::LLMBuilder,
    chat::{ChatMessage, StructuredOutputFormat},
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tracing::{debug, error};

#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Schema serialization error: {0}")]
    SchemaSerialization(#[from] serde_json::Error),
    #[error("LLM build error: {0}")]
    Build(String),
    #[error("Chat error: {0}")]
    Chat(String),
    #[error("Response parsing error: {0}")]
    ResponseParsing(String),
}

#[derive(Clone)]
pub struct LlmClient {
    api_key: String,
    model: models::ModelId,
    max_tokens: u32,
    temperature: f32,
}

fn try_parse<T>(text: &str) -> Result<T, LlmError>
where
    T: DeserializeOwned,
{
    let json_candidates: Vec<Option<String>> = vec![
        Some(text.trim().to_string()),
        extract_json_from_response(text),
        extract_json_aggressively(text).first().cloned(),
    ];

    let mut all_errors = Vec::new();

    for (i, candidate) in json_candidates.iter().enumerate() {
        if let Some(json_text) = candidate {
            debug!("Trying parse strategy {}: {:.200}...", i + 1, json_text);

            match serde_json::from_str::<T>(json_text) {
                Ok(parsed) => {
                    debug!("✅ Successfully parsed with strategy {}", i + 1);
                    return Ok(parsed);
                }
                Err(e) => {
                    debug!("❌ Strategy {} failed: {}", i + 1, e);
                    all_errors.push(format!("Strategy {}: {}", i + 1, e));
                    continue;
                }
            }
        } else {
            debug!("❌ Strategy {} returned no candidate", i + 1);
            all_errors.push(format!("Strategy {}: No JSON candidate found", i + 1));
        }
    }

    // If all strategies failed, return comprehensive error
    let error_summary = format!(
        "Failed to parse JSON with all {} strategies:\n{}",
        json_candidates.len(),
        all_errors.join("\n")
    );
    error!("{:}", error_summary);
    Err(LlmError::ResponseParsing(error_summary))
}

impl LlmClient {
    pub fn new(
        model: models::ModelId,
        api_key: Option<String>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Self {
        let key = match api_key {
            Some(key) => key,
            None => {
                let env_var = match model.provider() {
                    llm::builder::LLMBackend::Anthropic => "ANTHROPIC_KEY",
                    llm::builder::LLMBackend::AzureOpenAI => "AZURE_OPENAI_KEY",
                    llm::builder::LLMBackend::DeepSeek => "DEEPSEEK_KEY",
                    llm::builder::LLMBackend::ElevenLabs => "ELEVEN_LABS_KEY",
                    llm::builder::LLMBackend::Google => "GOOGLE_KEY",
                    llm::builder::LLMBackend::Groq => "GROQ_KEY",
                    llm::builder::LLMBackend::Ollama => "OLLAMA_KEY",
                    llm::builder::LLMBackend::OpenAI => "OPENAI_KEY",
                    llm::builder::LLMBackend::Phind => "PHIND_KEY",
                    llm::builder::LLMBackend::XAI => "XAI_KEY",
                };

                std::env::var(env_var).unwrap_or_else(|_| {
                    panic!(
                        "Missing env variable or config value {:?} for the model {:?}",
                        env_var, model
                    )
                })
            }
        };

        Self {
            api_key: key,
            model,
            max_tokens: max_tokens.unwrap_or(1500),
            temperature: temperature.unwrap_or(0.5),
        }
    }

    pub async fn get_structured_response<T>(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<T, LlmError>
    where
        T: JsonSchema + Serialize + for<'de> Deserialize<'de>,
    {
        let schema = schema_for!(T);

        let value_schema = serde_json::to_value(&schema)?;

        let output_schema = StructuredOutputFormat {
            name: T::schema_name(),
            schema: Some(value_schema.clone()),
            description: None,
            strict: Some(true),
        };

        let builder = LLMBuilder::new()
            .backend(self.model.provider())
            .api_key(&self.api_key)
            .model(self.model.to_string())
            .max_tokens(self.max_tokens)
            .temperature(self.temperature)
            .stream(false)
            .system(format!(
                r#"{}
CRITICAL INSTRUCTIONS:
- You MUST respond with ONLY a valid JSON object
- NO explanatory text before or after the JSON
- NO markdown code blocks or formatting
- NO comments or additional content
- The JSON must exactly match this schema:
```json
{:?}
``
Any response that is not pure JSON will be rejected."#,
                system_prompt, value_schema
            ))
            .schema(output_schema);

        let llm = builder
            .build()
            .map_err(|e| LlmError::Build(e.to_string()))?;

        let messages = vec![ChatMessage::user().content(user_prompt).build()];

        let response = llm
            .chat(&messages)
            .await
            .map_err(|e| LlmError::Chat(e.to_string()))?;

        let response_text = &response.text().unwrap();
        if response_text.is_empty() {
            error!("Empty response");
            return Err(LlmError::ResponseParsing("Empty Response".to_string()));
        }

        // Parse the structured response
        try_parse::<T>(response_text)
    }

    pub async fn get_simple_response(
        &self,
        system_prompt: &str,
        content: &str,
    ) -> Result<String, LlmError> {
        let llm = LLMBuilder::new()
            .backend(self.model.provider())
            .api_key(&self.api_key)
            .model(self.model.to_string())
            .max_tokens(self.max_tokens)
            .temperature(self.temperature)
            .stream(false)
            .system(system_prompt)
            .build()
            .map_err(|e| LlmError::Build(e.to_string()))?;

        let messages = vec![ChatMessage::user().content(content).build()];

        let response = llm
            .chat(&messages)
            .await
            .map_err(|e| LlmError::Chat(e.to_string()))?;

        // Match the pattern used in get_structured_response for consistency
        let response_text = response
            .text()
            .ok_or_else(|| LlmError::Chat("No text in response".to_string()))?;

        if response_text.is_empty() {
            return Err(LlmError::ResponseParsing("Empty response".to_string()));
        }

        Ok(response_text.to_string())
    }
}

// Builder pattern for more ergonomic configuration
pub struct LlmRequestBuilder<'a> {
    client: &'a LlmClient,
    system_prompt: String,
    content: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

impl<'a> LlmRequestBuilder<'a> {
    pub fn new(client: &'a LlmClient) -> Self {
        Self {
            client,
            system_prompt: String::new(),
            content: String::new(),
            max_tokens: Some(1000),
            temperature: Some(0.0),
        }
    }

    pub fn system_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    pub fn content<S: Into<String>>(mut self, prompt: S) -> Self {
        self.content = prompt.into();
        self
    }

    #[allow(dead_code)]
    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    #[allow(dead_code)]
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub async fn execute_structured<T>(self) -> Result<T, LlmError>
    where
        T: JsonSchema + Serialize + for<'de> Deserialize<'de>,
    {
        self.client
            .get_structured_response(&self.system_prompt, &self.content)
            .await
    }

    #[allow(dead_code)]
    pub async fn execute_simple(self) -> Result<String, LlmError> {
        self.client
            .get_simple_response(&self.system_prompt, &self.content)
            .await
    }
}

impl LlmClient {
    pub fn request(&self) -> LlmRequestBuilder {
        LlmRequestBuilder::new(self)
    }
}

// Example usage with proper error handling
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct TaskResponse {
        #[schemars(description = "If You can successfully do the task")]
        success: bool,
        #[schemars(
            description = "A short description of how you would do the task. 2 sentences max"
        )]
        message: String,
        #[schemars(description = "Confidence in your answer.")]
        confidence: f32,
    }

    #[tokio::test]
    async fn test_structured_response() -> Result<(), LlmError> {
        dotenv().ok();
        let client = LlmClient::new(models::ModelId::Claude35Haiku, None, Some(1000), Some(0.2));

        let response: TaskResponse = client
            .request()
            .system_prompt("Analyze this task and provide a structured response.")
            .content("refactoring a codebase to use an llm it currently has a number of heuristic based rules approximately 10 5 of which are regexes.")
            .execute_structured::<TaskResponse>()
            .await?;

        println!("Structured response: {:?}", response);
        Ok(())
    }

    #[tokio::test]
    async fn test_simple_response() -> Result<(), LlmError> {
        dotenv().ok();
        let client = LlmClient::new(models::ModelId::Claude35Haiku, None, None, None);

        let response = client
            .request()
            .system_prompt("You are a helpful assistant.")
            .content("What is the capital of France?")
            .execute_simple()
            .await?;

        println!("Simple response: {}", response);
        Ok(())
    }
}

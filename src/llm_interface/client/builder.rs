#![allow(dead_code)]
use super::LlmClient;
use super::RetryConfig;
use super::models;
use std::time::Duration;

/// Builder for constructing LlmClient instances with fluent configuration
pub struct LlmClientBuilder {
    model: Option<models::ModelId>,
    api_key: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    retry_config: Option<RetryConfig>,
}

impl LlmClientBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            model: None,
            api_key: None,
            max_tokens: None,
            temperature: None,
            retry_config: None,
        }
    }

    /// Set the model (required)
    pub fn model(mut self, model: models::ModelId) -> Self {
        self.model = Some(model);
        self
    }

    /// Set the API key explicitly
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set max tokens (default: 1500)
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set temperature (default: 0.5)
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set retry configuration
    pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = Some(retry_config);
        self
    }

    /// Enable retries with default configuration
    pub fn with_retries(mut self) -> Self {
        self.retry_config = Some(RetryConfig::default());
        self
    }

    /// Configure retry settings fluently
    pub fn retry_settings(self) -> RetryConfigBuilder {
        RetryConfigBuilder::new(self)
    }

    /// Build the LlmClient
    ///
    /// # Errors
    /// Returns an error if no model is specified
    pub fn build(self) -> Result<LlmClient, LlmClientBuilderError> {
        let model = self.model.ok_or(LlmClientBuilderError::MissingModel)?;

        let mut client = LlmClient::new(model, self.api_key, self.max_tokens, self.temperature);

        if let Some(retry_config) = self.retry_config {
            client = client.with_retry_config(retry_config);
        }

        Ok(client)
    }
}

impl Default for LlmClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for retry configuration
pub struct RetryConfigBuilder {
    client_builder: LlmClientBuilder,
    max_retries: Option<u32>,
    initial_interval: Option<Duration>,
    max_interval: Option<Duration>,
    multiplier: Option<f64>,
    max_elapsed_time: Option<Duration>,
}

impl RetryConfigBuilder {
    fn new(client_builder: LlmClientBuilder) -> Self {
        Self {
            client_builder,
            max_retries: None,
            initial_interval: None,
            max_interval: None,
            multiplier: None,
            max_elapsed_time: None,
        }
    }

    /// Set maximum number of retries (default: 5)
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Set initial retry interval (default: 1 second)
    pub fn initial_interval(mut self, duration: Duration) -> Self {
        self.initial_interval = Some(duration);
        self
    }

    /// Set maximum retry interval (default: 60 seconds)
    pub fn max_interval(mut self, duration: Duration) -> Self {
        self.max_interval = Some(duration);
        self
    }

    /// Set backoff multiplier (default: 2.0)
    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = Some(multiplier);
        self
    }

    /// Set maximum elapsed time for all retries (default: 5 minutes)
    pub fn max_elapsed_time(mut self, duration: Duration) -> Self {
        self.max_elapsed_time = Some(duration);
        self
    }

    /// Finish retry configuration and return to client builder
    pub fn finish(mut self) -> LlmClientBuilder {
        let retry_config = RetryConfig {
            max_retries: self.max_retries.unwrap_or(5),
            initial_interval: self.initial_interval.unwrap_or(Duration::from_millis(1000)),
            max_interval: self.max_interval.unwrap_or(Duration::from_secs(60)),
            multiplier: self.multiplier.unwrap_or(2.0),
            max_elapsed_time: self.max_elapsed_time.unwrap_or(Duration::from_secs(300)),
        };

        self.client_builder.retry_config = Some(retry_config);
        self.client_builder
    }

    /// Build the client directly with retry configuration
    pub fn build(self) -> Result<LlmClient, LlmClientBuilderError> {
        self.finish().build()
    }
}

/// Errors that can occur when building an LlmClient
#[derive(Debug, Clone, PartialEq)]
pub enum LlmClientBuilderError {
    /// No model was specified
    MissingModel,
}

impl std::fmt::Display for LlmClientBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmClientBuilderError::MissingModel => {
                write!(f, "Model must be specified when building LlmClient")
            }
        }
    }
}

impl std::error::Error for LlmClientBuilderError {}

// Convenience methods for LlmClient
impl LlmClient {
    /// Create a new builder for LlmClient
    pub fn builder() -> LlmClientBuilder {
        LlmClientBuilder::new()
    }

    /// Quick builder with just a model
    pub fn with_model(model: models::ModelId) -> LlmClientBuilder {
        LlmClientBuilder::new().model(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_errors() {
        // Test missing model
        let result = LlmClient::builder().build();
        assert!(matches!(result, Err(LlmClientBuilderError::MissingModel)));
    }
}

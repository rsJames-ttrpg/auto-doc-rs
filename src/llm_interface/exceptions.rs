use super::simplified_schema;

#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Schema serialization error: {0}")]
    SchemaSerialization(#[from] serde_json::Error),
    #[error("Schema serialization error: {0}")]
    SchemaSimplification(#[from] simplified_schema::ConversionError),
    #[error("LLM build error: {0}")]
    Build(String),
    #[error("Chat error: {0}")]
    Chat(String),
    #[error("Response parsing error: {0}")]
    ResponseParsing(String),
    #[error("Rate limit exceeded (429): {0}")]
    RateLimit(String),
    #[error("Server error (5xx): {0}")]
    ServerError(String),
}

impl LlmError {
    /// Determines if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self, LlmError::RateLimit(_) | LlmError::ServerError(_))
    }

    /// Creates an LlmError from a generic error string, detecting specific error types
    pub fn from_error_string(error: String) -> Self {
        let error_lower = error.to_lowercase();

        // Check for rate limiting indicators (429 errors)
        if error_lower.contains("429")
            || error_lower.contains("rate limit")
            || error_lower.contains("too many requests")
            || error_lower.contains("quota exceeded")
            || error_lower.contains("requests per minute")
            || error_lower.contains("requests per hour")
        {
            return LlmError::RateLimit(error);
        }

        // Check for server errors (5xx)
        if error_lower.contains("500")
            || error_lower.contains("502")
            || error_lower.contains("503")
            || error_lower.contains("504")
            || error_lower.contains("internal server error")
            || error_lower.contains("bad gateway")
            || error_lower.contains("service unavailable")
            || error_lower.contains("gateway timeout")
            || error_lower.contains("http status server error")
        {
            return LlmError::ServerError(error);
        }

        // Default to Chat error for other cases
        LlmError::Chat(error)
    }
}

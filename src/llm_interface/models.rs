use clap::ValueEnum;
use llm::builder::LLMBackend;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum, EnumIter)]
#[serde(rename_all = "kebab-case")]
pub enum ModelId {
    // OpenAI Models
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4.1")]
    Gpt41,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt41Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt41Nano,
    #[serde(rename = "o3")]
    O3,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o4-mini")]
    O4Mini,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,

    // Anthropic Claude Models
    #[serde(rename = "claude-3-5-sonnet-latest")]
    Claude35Sonnet,
    #[serde(rename = "claude-3-5-haiku-latest")]
    Claude35Haiku,
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-7-sonnet-latest")]
    Claude37Sonnet,
    #[serde(rename = "claude-sonnet-4-20250514")]
    Claude4Sonnet,
    #[serde(rename = "claude-opus-4-20250514")]
    Claude4Opus,

    // Google Gemini Models
    #[serde(rename = "gemini-1.5-pro")]
    Gemini15Pro,
    #[serde(rename = "gemini-1.5-flash")]
    Gemini15Flash,
    #[serde(rename = "gemini-1.5-flash-8b")]
    Gemini15Flash8b,
    #[serde(rename = "gemini-2.0-flash")]
    Gemini20FlashExp,
    #[serde(rename = "gemini-2.5-pro-preview-05-06")]
    Gemini25Pro,
    #[serde(rename = "gemini-2.5-flash-preview-05-20")]
    Gemini25Flash,

    // DeepSeek Models
    #[serde(rename = "deepseek-reasoner")]
    DeepseekReason,
    #[serde(rename = "deepseek-chat")]
    DeepseekChat,
    #[serde(rename = "deepseek-coder")]
    DeepseekCoder,

    // xAI Grok Models
    #[serde(rename = "grok-3")]
    Grok3,
    #[serde(rename = "grok-3-mini")]
    Grok3Mini,
    #[serde(rename = "grok-3-reasoning")]
    Grok3Reasoning,
    #[serde(rename = "grok-3-mini-reasoning")]
    Grok3MiniReasoning,
    #[serde(rename = "grok-2")]
    Grok2,
    #[serde(rename = "grok-1")]
    Grok1,

    // Popular Ollama Models (for local use)
    #[serde(rename = "llama3.3")]
    Llama33,
    #[serde(rename = "llama3.2")]
    Llama32,
    #[serde(rename = "codellama")]
    Codellama,
    #[serde(rename = "mistral")]
    Mistral,
    #[serde(rename = "mistral:7b")]
    Mistral7b,
    #[serde(rename = "gemma2")]
    Gemma2,
    #[serde(rename = "qwen2.5")]
    Qwen25,
    #[serde(rename = "phi3")]
    Phi3,

    // ElevenLabs Voice Models
    #[serde(rename = "eleven_multilingual_v2")]
    ElevenMultilingualV2,
    #[serde(rename = "eleven_flash_v2_5")]
    ElevenFlashV25,
    #[serde(rename = "eleven_turbo_v2_5")]
    ElevenTurboV25,
}

impl ModelId {
    /// Returns the provider for this model
    pub fn provider(&self) -> LLMBackend {
        match self {
            Self::Gpt4o
            | Self::Gpt4oMini
            | Self::Gpt41
            | Self::Gpt41Mini
            | Self::Gpt41Nano
            | Self::O3
            | Self::O3Mini
            | Self::O4Mini
            | Self::Gpt4Turbo
            | Self::Gpt35Turbo => LLMBackend::OpenAI,

            Self::Claude35Sonnet
            | Self::Claude35Haiku
            | Self::Claude3Opus
            | Self::Claude37Sonnet
            | Self::Claude4Sonnet
            | Self::Claude4Opus => LLMBackend::Anthropic,

            Self::Gemini15Pro
            | Self::Gemini15Flash
            | Self::Gemini15Flash8b
            | Self::Gemini20FlashExp
            | Self::Gemini25Pro
            | Self::Gemini25Flash => LLMBackend::Google,

            &Self::DeepseekChat | Self::DeepseekReason | Self::DeepseekCoder => {
                LLMBackend::DeepSeek
            }

            Self::Grok3
            | Self::Grok3Mini
            | Self::Grok3Reasoning
            | Self::Grok3MiniReasoning
            | Self::Grok2
            | Self::Grok1 => LLMBackend::XAI,

            Self::Llama33
            | Self::Llama32
            | Self::Codellama
            | Self::Mistral
            | Self::Mistral7b
            | Self::Gemma2
            | Self::Qwen25
            | Self::Phi3 => LLMBackend::Ollama,

            Self::ElevenMultilingualV2 | Self::ElevenFlashV25 | Self::ElevenTurboV25 => {
                LLMBackend::ElevenLabs
            }
        }
    }

    #[allow(dead_code)]
    /// Returns true if this model supports structured output
    pub fn supports_structured_output(&self) -> bool {
        matches!(
            self,
            Self::Gpt4o
                | Self::Gpt4oMini
                | Self::Gpt41
                | Self::Gpt41Mini
                | Self::O3
                | Self::O3Mini
                | Self::O4Mini
                | Self::Claude35Sonnet
                | Self::Claude35Haiku
                | Self::Claude3Opus
                | Self::Claude37Sonnet
                | Self::Claude4Sonnet
                | Self::Claude4Opus
                | Self::Gemini15Pro
                | Self::Gemini15Flash
                | Self::Gemini20FlashExp
                | Self::Gemini25Pro
                | Self::Gemini25Flash
                | Self::DeepseekChat
                | Self::DeepseekReason
                | Self::DeepseekCoder
                | Self::Grok3
                | Self::Grok3Mini
                | Self::Grok2
        )
    }

    #[allow(dead_code)]
    /// Returns true if this model has reasoning capabilities
    pub fn has_reasoning(&self) -> bool {
        matches!(
            self,
            Self::O3
                | Self::O3Mini
                | Self::O4Mini
                | Self::Claude37Sonnet
                | Self::Claude4Sonnet
                | Self::Claude4Opus
                | Self::Gemini25Pro
                | Self::Gemini25Flash
                | Self::DeepseekReason
                | Self::Grok3Reasoning
                | Self::Grok3MiniReasoning
        )
    }

    #[allow(dead_code)]
    /// Returns true if this model supports multimodal input (images/audio)
    pub fn is_multimodal(&self) -> bool {
        matches!(
            self,
            Self::Gpt4o
                | Self::Gpt4oMini
                | Self::Gpt41
                | Self::O3
                | Self::O4Mini
                | Self::Claude35Sonnet
                | Self::Claude35Haiku
                | Self::Claude3Opus
                | Self::Claude37Sonnet
                | Self::Claude4Sonnet
                | Self::Claude4Opus
                | Self::Gemini15Pro
                | Self::Gemini15Flash
                | Self::Gemini15Flash8b
                | Self::Gemini20FlashExp
                | Self::Gemini25Pro
                | Self::Gemini25Flash
                | Self::Grok3
                | Self::Grok3Mini
                | Self::Grok2
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Provider {
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "anthropic")]
    Anthropic,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "azure_openai")]
    AzureOpenAI,
    #[serde(rename = "deepseek")]
    DeepSeek,
    #[serde(rename = "xai")]
    Xai,
    #[serde(rename = "groq")]
    Groq,
    #[serde(rename = "ollama")]
    Ollama,
    #[serde(rename = "elevenlabs")]
    ElevenLabs,
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", serialized.trim_matches('"'))
    }
}

impl std::str::FromStr for ModelId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(&format!("\"{}\"", s))
    }
}

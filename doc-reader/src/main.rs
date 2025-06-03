pub mod settings;
pub mod tools;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::settings::{FileType, Settings};
use crate::tools::get_docs::GetDocumentationTool;
use clap::CommandFactory;
use clap::{Command, Parser, Subcommand, ValueEnum};
use clap_complete::{Generator, Shell, generate};
use mcp_core::server::Server;
use mcp_core::tool_text_content;
use mcp_core::transport::{ServerSseTransport, ServerStdioTransport};
use mcp_core::types::{ServerCapabilities, ToolCapabilities, ToolResponseContent};
use mcp_core_macros::{tool, tool_param};
use tracing::{Level, debug, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// Global settings that will be initialized once during CLI parsing
static SETTINGS: OnceLock<Settings> = OnceLock::new();

fn init_settings(config_path: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let settings = match config_path {
        Some(config_path) => Settings::from_file(&config_path.to_string_lossy())?,
        None => Settings::from_env()?,
    };
    debug!("Found settings with: {:#?}", settings);
    SETTINGS
        .set(settings)
        .map_err(|_| "Settings already initialized")?;
    Ok(())
}

pub fn get_global_settings() -> &'static Settings {
    SETTINGS
        .get()
        .expect("Settings not initialized - call init_settings() first")
}

#[derive(Parser, Debug)]
#[command(name = "doc-reader")]
#[command(bin_name = "doc-reader")]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, global = true, default_value = "warn")]
    log_level: LogLevel,
    /// Enable JSON logging format
    #[arg(long, global = true, default_value_t = false)]
    json_logs: bool,
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

/// Initialize tracing with the specified log level
fn init_tracing(log_level: LogLevel, json_format: bool) -> Result<(), Box<dyn std::error::Error>> {
    let level: Level = log_level.into();

    // Create a filter that respects RUST_LOG env var or uses the CLI arg
    let filter =
        EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(level.to_string()))?;

    if json_format {
        // JSON format for structured logging
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(filter)
            .json()
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;
    } else {
        // Pretty format for human-readable logs
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(filter)
            .with_target(false) // Don't show module paths
            .with_thread_ids(false) // Don't show thread IDs
            .with_file(false) // Don't show file names
            .with_line_number(false) // Don't show line numbers
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;
    }

    Ok(())
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum TransportType {
    Stdio,
    Sse,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Serve the mcp server
    Serve {
        #[arg(value_enum, default_value_t = TransportType::Stdio)]
        transport: TransportType,
    },
    /// Command for creating the config
    Config {
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, value_enum, default_value_t = FileType::Yaml)]
        format: FileType,
    },
    /// Generate shell completions
    Completions { shell: Shell },
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

#[tool(
    name = "echo",
    description = "Echo back the message you send",
    annotations(title = "Echo Tool", read_only_hint = true, destructive_hint = false)
)]
async fn echo_tool(
    message: tool_param!(String, description = "The message to echo back"),
) -> Result<ToolResponseContent, anyhow::Error> {
    Ok(tool_text_content!(message))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();
    init_tracing(cli.log_level.clone(), cli.json_logs)?;
    init_settings(cli.config)?;
    match cli.command {
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            print_completions(shell, &mut cmd);
            Ok(())
        }
        Some(Commands::Serve { transport }) => {
            let server_protocol = Server::builder(
                "Doc Reader".to_string(),
                "1.0".to_string(),
                mcp_core::types::ProtocolVersion::V2024_11_05,
            )
            .set_capabilities(ServerCapabilities {
                tools: Some(ToolCapabilities::default()),
                ..Default::default()
            })
            .register_tool(GetDocumentationTool::tool(), GetDocumentationTool::call())
            .build();
            match transport {
                TransportType::Stdio => {
                    let transport = ServerStdioTransport::new(server_protocol);
                    Server::start(transport).await?
                }
                TransportType::Sse => {
                    let transport =
                        ServerSseTransport::new("127.0.0.1".to_string(), 3000, server_protocol);
                    Server::start(transport).await?
                }
            }
            Ok(())
        }
        Some(Commands::Config { output, format }) => {
            if let Err(e) = Settings::write_default_config(output, format) {
                error!("Error generating config: {}", e);
                std::process::exit(1);
            }
            Ok(())
        }
        None => Ok(()),
    }
}

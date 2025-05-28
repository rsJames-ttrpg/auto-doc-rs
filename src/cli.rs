use crate::crawler::file::{CrawlOptions, crawl_directory};
use crate::generate::{AnalysisCrawlOptions, AnalysisCrawler};
use crate::llm_interface::LlmClient;
use crate::llm_interface::models::ModelId;
use crate::output::file_system::{MarkdownConfig, MarkdownGenerator};
use crate::settings::{FileType, Settings};
use clap::CommandFactory;
use clap::{Command, Parser, Subcommand, ValueEnum};
use clap_complete::{Generator, Shell, generate};
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use strum::IntoEnumIterator;
use tracing::{Level, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Parser, Debug)]
#[command(name = "auto-doc")]
#[command(bin_name = "auto-doc")]
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
    #[arg(long = "completion", value_enum)]
    completions: Option<Shell>,
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

#[derive(Subcommand, Debug)]
enum Commands {
    /// Uses the files options in the config to show what files are targeted (useful for testing globs/excludes)
    Crawl,
    /// Generates the docs
    Generate {
        #[arg(short, long, default_value_t = false)]
        preview: bool,
        dir: PathBuf,
        #[arg(short, long)]
        directory_output: Option<PathBuf>,
    },
    /// Generate an example config
    Config {
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, value_enum, default_value_t = FileType::Toml)]
        format: FileType,
    },
    /// Print supported models to std out
    Models,
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn crawl() -> Result<(), Box<dyn std::error::Error>> {
    let options: CrawlOptions = CrawlOptions {
        max_depth: Some(3),
        include_hidden: false,
        glob_patterns: vec!["*.rs".to_string(), "*.toml".to_string()], // Only include these
        exclude_patterns: vec![
            "target/**".to_string(),
            "node_modules/**".to_string(),
            ".git/**".to_string(),
            "*.tmp".to_string(),
        ],
        ..Default::default()
    };

    match crawl_directory("./src", options) {
        Ok(tree) => {
            println!("File tree:");
            tree.print_tree();

            println!("\nTotal files: {}", tree.total_files());

            let rust_files = tree.find_files_by_extension("rs");
            println!("Found {} Rust files", rust_files.len());

            // Examples using the new iterators
            println!("\nAll nodes:");
            for node in tree.iter() {
                println!("  {:?}", node.path());
            }

            println!("\nFiles only:");
            for file in tree.collect_files() {
                println!("  📄 {:?}", file.path());
            }

            println!("\nWith depth levels:");
            for (node, depth) in tree.iter_with_depth() {
                let indent = "  ".repeat(depth);
                println!("{}[{}] {}", indent, depth, node.name());
            }
        }
        Err(e) => eprintln!("Error crawling directory: {}", e),
    }

    Ok(())
}

pub async fn run_application() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    if let Some(generator) = cli.completions {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
    } else {
        println!("{cli:#?}");
    }
    init_tracing(cli.log_level.clone(), cli.json_logs)?;
    let settings: Settings = match cli.config {
        Some(config_path) => Settings::from_file(&config_path.to_string_lossy())?,
        None => Settings::from_env()?,
    };

    match cli.command {
        Some(Commands::Crawl) => {
            print!("{:#?}", settings);
            crawl()
        }
        Some(Commands::Config { output, format }) => {
            if let Err(e) = Settings::write_default_config(output, format) {
                error!("Error generating config: {}", e);
                std::process::exit(1);
            }
            Ok(())
        }
        Some(Commands::Generate {
            preview,
            dir,
            directory_output,
        }) => {
            dotenv().ok();
            let analyser: LlmClient = LlmClient::new(
                settings.llm_settings.first().unwrap().model.clone(),
                settings.llm_settings.first().unwrap().api_key.clone(),
                settings.llm_settings.first().unwrap().max_tokens,
                settings.llm_settings.first().unwrap().temperature,
            );
            let crawler = AnalysisCrawler::new(analyser);

            let options = AnalysisCrawlOptions {
                crawl_options: CrawlOptions {
                    exclude_patterns: settings.files.exclude_patterns,
                    glob_patterns: settings.files.include_patterns,
                    include_hidden: settings.files.include_hidden,
                    max_depth: settings.files.max_depth,
                    ..Default::default()
                },
                ..Default::default()
            };
            match preview {
                true => {
                    let preview = crawler.preview_analysis(dir.clone(), &options)?;
                    preview.print_summary()
                }
                false => {
                    let crawl_spinner = ProgressBar::new_spinner();
                    crawl_spinner.set_style(
                        ProgressStyle::default_spinner()
                            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                            .template("{spinner:.blue} {msg}")
                            .unwrap(),
                    );
                    crawl_spinner.set_message("Crawling directory structure...");
                    crawl_spinner.enable_steady_tick(Duration::from_millis(100));
                    let (analysis, children) =
                        crawler.analyze_project(dir.clone(), options).await?;
                    crawl_spinner.finish_with_message("✅ Directory crawling complete");

                    let mut config_builder = MarkdownConfig::builder().project_root(dir.clone());

                    if let Some(output_dir) = directory_output {
                        config_builder = config_builder.output_dir(output_dir);
                    }

                    let config = config_builder.build();
                    let md_generator = MarkdownGenerator::new(config);
                    md_generator
                        .generate_documentation(&analysis, &children)
                        .await?;
                }
            }
            Ok(())
        }
        Some(Commands::Models) => {
            for model in ModelId::iter() {
                println!(
                    "model-id: {:?}, provider: {:?}",
                    model.to_string(),
                    model.provider()
                );
            }

            Ok(())
        }
        None => Ok(()),
    }
}

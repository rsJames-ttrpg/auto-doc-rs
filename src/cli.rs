use crate::crawler::file::{CrawlOptions, crawl_directory};
use crate::generate::{AnalysisCrawlOptions, AnalysisCrawler};
use crate::llm_interface::LlmClient;
use crate::llm_interface::models::ModelId;
use crate::settings::{FileType, Settings};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::path::PathBuf;
use strum::IntoEnumIterator;

#[derive(Parser)]
#[command(name = "auto-doc")]
#[command(bin_name = "auto-doc")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Crawl {
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    Generate {
        #[arg(short, long, default_value_t = false)]
        preview: bool,
        #[arg(short, long)]
        config: Option<PathBuf>,
        dir: PathBuf,
    },
    GenerateConfig {
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, value_enum, default_value_t = FileType::Toml)]
        format: FileType,
    },
    Models,
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
            tree.print_tree(0);

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
    match cli.command {
        Some(Commands::Crawl { config }) => {
            let settings = match config {
                Some(config_path) => Settings::from_file(&config_path.to_string_lossy())?,
                None => Settings::from_env()?,
            };
            print!("{:#?}", settings);
            crawl()
        }
        Some(Commands::GenerateConfig { output, format }) => {
            if let Err(e) = Settings::write_default_config(output, format) {
                eprintln!("Error generating config: {}", e);
                std::process::exit(1);
            }
            Ok(())
        }
        Some(Commands::Generate {
            preview,
            config,
            dir,
        }) => {
            dotenv().ok();
            let settings = match config {
                Some(config_path) => Settings::from_file(&config_path.to_string_lossy())?,
                None => Settings::from_env()?,
            };
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
                max_concurrent_analyses: 5,
                ..Default::default()
            };
            match preview {
                true => {
                    let preview = crawler.preview_analysis(dir, &options)?;
                    preview.print_summary()
                }
                false => {
                    let analysis = crawler.analyze_project(dir, options).await?;
                    println!("{:?}", analysis)
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

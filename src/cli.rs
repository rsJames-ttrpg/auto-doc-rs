use crate::crawler::file::{CrawlOptions, crawl_directory};
use crate::settings::{FileType, Settings};

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    GenerateConfig {
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, value_enum, default_value_t = FileType::Toml)]
        format: FileType,
    },
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

pub fn run_application() -> Result<(), Box<dyn std::error::Error>> {
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
        None => Ok(()),
    }
}

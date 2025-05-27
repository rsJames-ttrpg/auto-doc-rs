mod analysis;
mod cli;
mod crawler;
mod generate;
mod llm_interface;
mod settings;
use crate::cli::run_application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_application().await
}

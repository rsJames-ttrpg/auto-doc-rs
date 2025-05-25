mod cli;
mod crawler;
mod settings;

use crate::cli::run_application;

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_application()
}

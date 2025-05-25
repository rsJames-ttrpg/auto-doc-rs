mod crawler;

use crate::crawler::file::{CrawlOptions, crawl_directory};

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example usage with specific patterns
    let options = CrawlOptions {
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

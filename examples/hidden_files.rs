//! Hidden files handling example
//!
//! This example demonstrates the difference between analyzing
//! directories with and without hidden files.
//!
//! Run with: `cargo run --example hidden_files`

use std::path::Path;
use lesort::collect_files;

fn main() {
    println!("=== LeSort Hidden Files Example ===");
    println!();

    let dir = Path::new(".");
    println!("Analyzing directory: {}", dir.display());
    println!();

    // Analyze without hidden files
    match collect_files(dir, false) {
        Ok(analysis) => {
            println!("📊 Analysis WITHOUT hidden files:");
            println!("  Total files: {}", analysis.file_count);
            println!("  File types: {}", analysis.extension_counts.len());
            println!();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }

    // Analyze with hidden files
    match collect_files(dir, true) {
        Ok(analysis) => {
            println!("📊 Analysis WITH hidden files included:");
            println!("  Total files: {}", analysis.file_count);
            println!("  File types: {}", analysis.extension_counts.len());
            println!();

            // Show hidden files if any
            let hidden_files: Vec<_> = analysis
                .files
                .iter()
                .filter(|(name, _)| name.starts_with('.'))
                .collect();

            if !hidden_files.is_empty() {
                println!("🔍 Hidden files found:");
                for (name, ext) in hidden_files {
                    println!("  {} [{}]", name, ext);
                }
            } else {
                println!("🔍 No hidden files found in this directory");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

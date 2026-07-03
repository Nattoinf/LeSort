//! Hidden files handling example
//!
//! This example demonstrates the difference between analyzing
//! directories with and without hidden files.
//!
//! Run with: `cargo run --example hidden_files`

use lesort::collect_files;
use std::path::Path;

fn main() {
    println!("=== LeSort Hidden Files Example ===");
    println!();

    let dir = Path::new(".");
    println!("Analyzing directory: {}", dir.display());
    println!();

    analyze_without_hidden(dir);
    println!();
    analyze_with_hidden(dir);
}

/// Analyze directory without hidden files
fn analyze_without_hidden(dir: &Path) {
    match collect_files(dir, false) {
        Ok(analysis) => {
            println!("📊 Analysis WITHOUT hidden files:");
            println!("  Total files: {}", analysis.file_count);
            println!("  File types: {}", analysis.extension_counts.len());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

/// Analyze directory with hidden files and display them
fn analyze_with_hidden(dir: &Path) {
    match collect_files(dir, true) {
        Ok(analysis) => {
            println!("📊 Analysis WITH hidden files included:");
            println!("  Total files: {}", analysis.file_count);
            println!("  File types: {}", analysis.extension_counts.len());
            println!();

            display_hidden_files(&analysis);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

/// Display hidden files found in the directory
fn display_hidden_files(analysis: &lesort::AnalysisResult) {
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

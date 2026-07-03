//! Basic usage example - Analyze current directory
//!
//! This example shows how to use the lesort library to analyze
//! a directory and print the results.
//!
//! Run with: `cargo run --example basic_analysis`

use lesort::collect_files;
use std::path::Path;

fn main() {
    println!("=== LeSort Basic Analysis Example ===");
    println!();

    let dir = Path::new(".");
    println!("Analyzing directory: {:?}", dir.display());
    println!();

    match collect_files(dir, false) {
        Ok(analysis) => {
            display_statistics(&analysis);
            display_extension_breakdown(&analysis);
            display_organization_score(&analysis);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

/// Display basic file statistics
fn display_statistics(analysis: &lesort::AnalysisResult) {
    println!("📊 File Statistics:");
    println!("  Total files: {}", analysis.file_count);
    println!("  File types: {}", analysis.extension_counts.len());
    println!();
}

/// Display top file extensions
fn display_extension_breakdown(analysis: &lesort::AnalysisResult) {
    println!("📁 Extension Breakdown:");
    let mut extensions: Vec<_> = analysis.extension_counts.iter().collect();
    extensions.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

    for &(ext, count) in extensions.iter().take(10) {
        let percentage = (*count as f64 / analysis.file_count as f64) * 100.0;
        println!("  {}: {} files ({:.1}%)", ext, count, percentage);
    }
    println!();
}

/// Calculate and display organization score
fn display_organization_score(analysis: &lesort::AnalysisResult) {
    let score =
        lesort::calculate_organization_score(analysis.file_count, analysis.extension_counts.len());
    println!("📈 Organization Score: {:.2}%", score);
    println!("   {}", lesort::interpret_score(score));
}

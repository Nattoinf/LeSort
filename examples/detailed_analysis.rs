//! Detailed analysis example - Show all files with extensions
//!
//! This example demonstrates how to display a detailed list of all files
//! in a directory with their extensions.
//!
//! Run with: `cargo run --example detailed_analysis -- <directory>`
//! Or: `cargo run --example detailed_analysis` (uses current directory)

use std::env;
use std::path::Path;
use lesort::collect_files;

fn main() {
    println!("=== LeSort Detailed Analysis Example ===");
    println!();

    let dir_path = parse_arguments();
    let dir = Path::new(&dir_path);

    if !validate_directory(dir) {
        return;
    }

    println!("Analyzing directory: {}", dir.display());
    println!();

    match collect_files(dir, false) {
        Ok(analysis) => {
            display_summary(&analysis);
            display_file_list(&analysis);
            display_extension_summary(&analysis);
            display_organization_score(&analysis);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

/// Parse command line arguments or return default directory
fn parse_arguments() -> String {
    env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string())
}

/// Validate that the path is a valid directory
fn validate_directory(dir: &Path) -> bool {
    if !dir.exists() {
        eprintln!("Error: Directory not found: {}", dir.display());
        return false;
    }

    if !dir.is_dir() {
        eprintln!("Error: Not a directory: {}", dir.display());
        return false;
    }

    true
}

/// Display summary statistics
fn display_summary(analysis: &lesort::AnalysisResult) {
    println!("📊 Summary:");
    println!("  Total files: {}", analysis.file_count);
    println!("  File types: {}", analysis.extension_counts.len());
    println!();
}

/// Display all files sorted alphabetically
fn display_file_list(analysis: &lesort::AnalysisResult) {
    println!("📝 All Files:");
    let mut files = analysis.files.clone();
    files.sort();

    for (name, ext) in files.iter() {
        println!("  {} [{}]", name, ext);
    }
    println!();
}

/// Display extension summary
fn display_extension_summary(analysis: &lesort::AnalysisResult) {
    println!("📁 Extension Summary:");
    let mut extensions: Vec<_> = analysis.extension_counts.iter().collect();
    extensions.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

    for (ext, count) in extensions {
        let percentage = (*count as f64 / analysis.file_count as f64) * 100.0;
        println!("  {}: {} files ({:.1}%)", ext, count, percentage);
    }
    println!();
}

/// Calculate and display organization score
fn display_organization_score(analysis: &lesort::AnalysisResult) {
    let score = lesort::calculate_organization_score(
        analysis.file_count,
        analysis.extension_counts.len(),
    );
    println!("📈 Organization Score: {:.2}%", score);
    println!("   {}", lesort::interpret_score(score));
}

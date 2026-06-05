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
    // Get directory from command line or use current directory
    let dir_path = env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string());

    println!("=== LeSort Detailed Analysis Example ===");
    println!();

    let dir = Path::new(&dir_path);

    // Validate directory exists
    if !dir.exists() {
        eprintln!("Error: Directory not found: {}", dir_path);
        return;
    }

    if !dir.is_dir() {
        eprintln!("Error: Not a directory: {}", dir_path);
        return;
    }

    println!("Analyzing directory: {}", dir.display());
    println!();

    match collect_files(dir, false) {
        Ok(analysis) => {
            // Summary
            println!("📊 Summary:");
            println!("  Total files: {}", analysis.file_count);
            println!("  File types: {}", analysis.extension_counts.len());
            println!();

            // Detailed file list
            println!("📝 All Files:");
            let mut files = analysis.files.clone();
            files.sort();

            for (name, ext) in files.iter() {
                println!("  {} [{}]", name, ext);
            }
            println!();

            // Extension summary
            println!("📁 Extension Summary:");
            let mut extensions: Vec<_> = analysis.extension_counts.iter().collect();
            extensions.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

            for (ext, count) in extensions {
                let percentage = (*count as f64 / analysis.file_count as f64) * 100.0;
                println!("  {}: {} files ({:.1}%)", ext, count, percentage);
            }
            println!();

            // Organization score
            let score = lesort::calculate_organization_score(
                analysis.file_count,
                analysis.extension_counts.len(),
            );
            println!("📈 Organization Score: {:.2}%", score);
            println!("   {}", lesort::interpret_score(score));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

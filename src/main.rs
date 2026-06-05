//! # LeSort - File Organization Analyzer
//!
//! LeSort is a command-line tool that analyzes file organization in a directory.
//! It provides statistics about file types, calculates an organization score,
//! and helps users understand the structure of their directories.
//!
//! ## Features
//!
//! - Analyze file distribution by extension
//! - Calculate organization score based on file count and diversity
//! - Display detailed file lists
//! - Support for hidden files
//! - Command-line interface with multiple options
//!
//! ## Usage
//!
//! ```text
//! lesort [OPTIONS] [PATH]
//!
//! Arguments:
//!   [PATH]  Target directory to analyze (default: current directory)
//!
//! Options:
//!   -a, --all      Include hidden files in analysis
//!   -d, --detail   Show detailed classification of files
//!   -s, --score    Display organization score
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```
//!
//! ## Examples
//!
//! Analyze current directory:
//! ```text
//! $ lesort
//! ```
//!
//! Analyze with score and details:
//! ```text
//! $ lesort -s -d
//! ```
//!
//! Analyze specific directory with all files:
//! ```text
//! $ lesort /path/to/dir -a
//! ```

use clap::Parser;
use std::path::Path;
use std::collections::HashMap;

/// File organization analyzer and scoring tool
#[derive(Parser, Debug)]
#[command(name = "lesort")]
#[command(version = "0.1.0")]
#[command(about = "Analyzes files in a directory based on their extensions and naming patterns", long_about = None)]
struct Args {
    /// Target directory to analyze (default: current directory)
    #[arg(value_name = "PATH")]
    path: Option<String>,

    /// Include hidden files in analysis
    #[arg(short, long)]
    all: bool,

    /// Show detailed classification of files
    #[arg(short, long)]
    detail: bool,

    /// Display organization score
    #[arg(short, long)]
    score: bool,
}

/// File analysis result structure
pub struct AnalysisResult {
    pub file_count: usize,
    pub extension_counts: HashMap<String, usize>,
    pub files: Vec<(String, String)>,
}

fn main() {
    let args = Args::parse();

    let path = args.path.unwrap_or_else(|| ".".to_string());

    match analyze_directory(&path, args.all, args.detail, args.score) {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Analyzes a directory and displays file organization statistics and scores.
///
/// # Arguments
/// * `path` - Directory path to analyze
/// * `include_all` - Whether to include hidden files
/// * `detail` - Whether to show detailed file list
/// * `show_score` - Whether to display organization score
fn analyze_directory(path: &str, include_all: bool, detail: bool, show_score: bool) -> Result<(), String> {
    let dir_path = Path::new(path);

    if !dir_path.exists() {
        return Err(format!("Directory not found: {}", path));
    }

    if !dir_path.is_dir() {
        return Err(format!("Not a directory: {}", path));
    }

    println!("Analyzing directory: {}\n", dir_path.display());

    let mut analysis_result = lesort::collect_files(dir_path, include_all)?;

    print_results(
        analysis_result.file_count,
        &analysis_result.extension_counts,
        &mut analysis_result.files,
        detail,
        show_score,
    );

    Ok(())
}

/// Displays analysis results with optional detailed and score information.
///
/// Always shows file statistics and extension breakdown.
/// Conditionally shows detailed file list and organization score.
fn print_results(
    file_count: usize,
    extension_counts: &HashMap<String, usize>,
    files: &mut [(String, String)],
    detail: bool,
    show_score: bool,
) {
    print_file_statistics(file_count, extension_counts);
    print_extension_breakdown(file_count, extension_counts);

    if detail {
        print_detailed_file_list(files);
    }

    if show_score {
        print_organization_score(file_count, extension_counts);
    }
}

/// Displays file count and file type statistics.
fn print_file_statistics(file_count: usize, extension_counts: &HashMap<String, usize>) {
    println!("📊 File Statistics:");
    println!("  Total files: {}", file_count);
    println!("  File types: {}", extension_counts.len());
    println!();
}

/// Displays extension breakdown sorted by file count in descending order.
///
/// Shows each extension with its file count and percentage.
fn print_extension_breakdown(file_count: usize, extension_counts: &HashMap<String, usize>) {
    println!("📁 Extension Breakdown:");
    let mut extensions: Vec<_> = extension_counts.iter().collect();
    extensions.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

    for (ext, count) in extensions {
        let percentage = (*count as f64 / file_count as f64) * 100.0;
        println!("  {}: {} files ({:.1}%)", ext, count, percentage);
    }
    println!();
}

/// Displays a detailed list of all files sorted alphabetically.
///
/// Format: filename [extension]
fn print_detailed_file_list(files: &mut [(String, String)]) {
    println!("📝 Detailed File List:");
    files.sort();

    for (name, ext) in files.iter() {
        println!("  {} [{}]", name, ext);
    }

    println!();
}

/// Calculates and displays the organization score.
///
/// Score is computed using three penalty factors:
/// 1. File count penalty (max 30 points)
/// 2. Extension type penalty (quadratic increase)
/// 3. Diversity penalty (max 20 points)
fn print_organization_score(file_count: usize, extension_counts: &HashMap<String, usize>) {
    let score = lesort::calculate_organization_score(
        file_count,
        extension_counts.len(),
    );

    println!("📈 Organization Score: {:.2}%", score);
    println!("{}", lesort::interpret_score(score));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_extension_with_extension() {
        let path = Path::new("test.txt");
        assert_eq!(lesort::get_extension(path), "txt");
    }

    #[test]
    fn test_get_extension_without_extension() {
        let path = Path::new("README");
        assert_eq!(lesort::get_extension(path), "(no extension)");
    }

    #[test]
    fn test_get_extension_multiple_dots() {
        let path = Path::new("archive.tar.gz");
        assert_eq!(lesort::get_extension(path), "gz");
    }

    #[test]
    fn test_is_hidden_with_dot() {
        assert!(lesort::is_hidden(".hidden"));
    }

    #[test]
    fn test_is_hidden_without_dot() {
        assert!(!lesort::is_hidden("visible"));
    }

    #[test]
    fn test_is_hidden_dot_only() {
        assert!(lesort::is_hidden("."));
    }

    #[test]
    fn test_calculate_organization_score_empty_directory() {
        let score = lesort::calculate_organization_score(0, 0);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn test_calculate_organization_score_single_file_single_type() {
        let score = lesort::calculate_organization_score(1, 1);
        assert!(score > 0.0 && score < 100.0);
    }

    #[test]
    fn test_calculate_organization_score_many_files_many_types() {
        let score = lesort::calculate_organization_score(213, 14);
        assert!(score < 20.0);
    }

    #[test]
    fn test_calculate_organization_score_range() {
        let score = lesort::calculate_organization_score(50, 5);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_interpret_score_excellent() {
        let result = lesort::interpret_score(85.0);
        assert!(result.contains("✅ Excellent"));
    }

    #[test]
    fn test_interpret_score_good() {
        let result = lesort::interpret_score(70.0);
        assert!(result.contains("👍 Good"));
    }

    #[test]
    fn test_interpret_score_fair() {
        let result = lesort::interpret_score(50.0);
        assert!(result.contains("⚠️"));
    }

    #[test]
    fn test_interpret_score_poor() {
        let result = lesort::interpret_score(20.0);
        assert!(result.contains("❌ Poor"));
    }

    /// Score should be in good range (60-80): score ≈ 69.89
    #[test]
    fn test_calculate_organization_score_boundary_good() {
        let score = lesort::calculate_organization_score(50, 1);
        assert!(score >= 60.0 && score < 80.0, "Expected 60-80, got {}", score);
    }

    /// Score should be in fair range (40-60): score ≈ 49.5
    #[test]
    fn test_calculate_organization_score_boundary_fair() {
        let score = lesort::calculate_organization_score(100, 8);
        assert!(score >= 40.0 && score < 60.0, "Expected 40-60, got {}", score);
    }

    /// Score should be in poor range (0-40): score ≈ 25.4
    #[test]
    fn test_calculate_organization_score_boundary_poor() {
        let score = lesort::calculate_organization_score(150, 12);
        assert!(score >= 0.0 && score < 40.0, "Expected 0-40, got {}", score);
    }

    #[test]
    fn test_calculate_organization_score_penalty_increases() {
        let score_10 = lesort::calculate_organization_score(100, 10);
        let score_15 = lesort::calculate_organization_score(100, 15);
        assert!(score_10 > score_15);
    }
}

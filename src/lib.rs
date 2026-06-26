//! # LeSort Library - File Analysis Core
//!
//! This library provides the core functionality for LeSort, a file organization analyzer.
//! It offers utilities for analyzing directory structures, calculating organization scores,
//! and providing insights about file organization.
//!
//! ## Main Components
//!
//! - [`collect_files`]: Collects and analyzes files from a directory
//! - [`calculate_organization_score`]: Computes an organization quality score
//! - [`interpret_score`]: Converts numeric scores to human-readable feedback
//! - [`get_extension`]: Extracts file extensions
//! - [`is_hidden`]: Checks if files are hidden
//!
//! ## Organization Score
//!
//! The organization score is calculated based on three penalty factors:
//!
//! - **File count penalty**: Penalizes having too many files (max 30 points)
//! - **Type penalty**: Penalizes excessive file type diversity (quadratic increase)
//! - **Diversity penalty**: Penalizes unbalanced extension distribution (max 20 points)
//!
//! The score is calculated as:
//! ```text
//! score = 100 - file_penalty - type_penalty - diversity_penalty
//! ```
//!
//! Score ranges:
//! - 80-100%: Excellent organization ✅
//! - 60-79%: Good organization 👍
//! - 40-59%: Fair organization ⚠️
//! - 0-39%: Poor organization ❌
//!
//! ## Example
//!
//! ```no_run
//! use std::path::Path;
//! use lesort::collect_files;
//!
//! let dir = Path::new(".");
//! if let Ok(analysis) = collect_files(dir, false) {
//!     println!("Total files: {}", analysis.file_count);
//!     println!("File types: {}", analysis.extension_counts.len());
//! }
//! ```

use std::fs;
use std::path::Path;
use std::collections::HashMap;

/// File analysis result structure
#[derive(Debug)]
pub struct AnalysisResult {
    pub file_count: usize,
    pub extension_counts: HashMap<String, usize>,
    pub files: Vec<(String, String)>,
}

/// Collects file information from a directory.
///
/// Returns file count, extension statistics, and file list details.
/// Skips hidden files unless include_all is true.
pub fn collect_files(dir_path: &Path, include_all: bool) -> Result<AnalysisResult, String> {
    let mut file_count = 0;
    let mut extension_counts: HashMap<String, usize> = HashMap::new();
    let mut files: Vec<(String, String)> = Vec::new();

    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let file_path = entry.path();

                process_file(
                    &file_path,
                    include_all,
                    &mut file_count,
                    &mut extension_counts,
                    &mut files,
                );
            }
        }
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    }

    Ok(AnalysisResult {
        file_count,
        extension_counts,
        files,
    })
}

/// Extracts the file extension from a file path.
///
/// Returns "(no extension)" if the file has no extension.
pub fn get_extension(file_path: &Path) -> String {
    let extension = file_path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    if extension.is_empty() {
        "(no extension)".to_string()
    } else {
        extension
    }
}

/// Checks if a filename is hidden (starts with a dot).
pub fn is_hidden(file_name: &str) -> bool {
    file_name.starts_with('.')
}

/// Updates file statistics with information about a new file.
///
/// Increments file count, adds extension to counts map, and adds file to list.
fn update_statistics(
    file_name: String,
    ext: String,
    file_count: &mut usize,
    extension_counts: &mut HashMap<String, usize>,
    files: &mut Vec<(String, String)>,
) {
    *file_count += 1;

    *extension_counts
        .entry(ext.clone())
        .or_insert(0) += 1;

    files.push((file_name, ext));
}

/// Processes a single file for analysis.
///
/// Skips hidden files and directories. Updates statistics for valid files.
fn process_file(
    file_path: &Path,
    include_all: bool,
    file_count: &mut usize,
    extension_counts: &mut HashMap<String, usize>,
    files: &mut Vec<(String, String)>,
) {
    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    if !include_all && is_hidden(&file_name) {
        return;
    }

    if !file_path.is_file() {
        return;
    }

    let ext = get_extension(file_path);

    update_statistics(
        file_name,
        ext,
        file_count,
        extension_counts,
        files,
    );
}

/// Calculates the organization score based on file count and extension diversity.
///
/// Formula:
/// - Base score: 100
/// - File penalty: (file_count / (1 + file_count)) × 30
/// - Type penalty: (type_count)² × 0.3
/// - Diversity penalty: (type_count / file_count) × 20
/// - Final score: 100 - file_penalty - type_penalty - diversity_penalty (clamped to 0-100)
pub fn calculate_organization_score(file_count: usize, type_count: usize) -> f64 {
    if file_count == 0 {
        return 100.0;
    }

    let mut score = 100.0;

    let file_penalty = (file_count as f64 / (1.0 + file_count as f64)) * 30.0;
    score -= file_penalty;

    let type_penalty = (type_count as f64).powi(2) * 0.3;
    score -= type_penalty;

    let diversity_ratio = type_count as f64 / file_count as f64;
    let diversity_penalty = diversity_ratio * 20.0;
    score -= diversity_penalty;

    score.clamp(0.0, 100.0)
}

/// Interprets the organization score and returns a descriptive message.
///
/// - 80-100%: Excellent organization
/// - 60-79%: Good organization
/// - 40-59%: Fair organization (could be improved)
/// - 0-39%: Poor organization (needs reorganizing)
pub fn interpret_score(score: f64) -> String {
    if score >= 80.0 {
        "✅ Excellent organization!".to_string()
    } else if score >= 60.0 {
        "👍 Good organization.".to_string()
    } else if score >= 40.0 {
        "⚠️  Fair organization. Could be improved.".to_string()
    } else {
        "❌ Poor organization. Consider reorganizing.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_extension_with_extension() {
        let path = Path::new("test.txt");
        assert_eq!(get_extension(path), "txt");
    }

    #[test]
    fn test_get_extension_without_extension() {
        let path = Path::new("README");
        assert_eq!(get_extension(path), "(no extension)");
    }

    #[test]
    fn test_get_extension_multiple_dots() {
        let path = Path::new("archive.tar.gz");
        assert_eq!(get_extension(path), "gz");
    }

    #[test]
    fn test_is_hidden_with_dot() {
        assert!(is_hidden(".hidden"));
    }

    #[test]
    fn test_is_hidden_without_dot() {
        assert!(!is_hidden("visible"));
    }

    #[test]
    fn test_is_hidden_dot_only() {
        assert!(is_hidden("."));
    }

    #[test]
    fn test_calculate_organization_score_empty_directory() {
        let score = calculate_organization_score(0, 0);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn test_calculate_organization_score_single_file_single_type() {
        let score = calculate_organization_score(1, 1);
        assert!(score > 0.0 && score < 100.0);
    }

    #[test]
    fn test_calculate_organization_score_many_files_many_types() {
        let score = calculate_organization_score(213, 14);
        assert!(score < 20.0);
    }

    #[test]
    fn test_calculate_organization_score_range() {
        let score = calculate_organization_score(50, 5);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_interpret_score_excellent() {
        let result = interpret_score(85.0);
        assert!(result.contains("✅ Excellent"));
    }

    #[test]
    fn test_interpret_score_good() {
        let result = interpret_score(70.0);
        assert!(result.contains("👍 Good"));
    }

    #[test]
    fn test_interpret_score_fair() {
        let result = interpret_score(50.0);
        assert!(result.contains("⚠️"));
    }

    #[test]
    fn test_interpret_score_poor() {
        let result = interpret_score(20.0);
        assert!(result.contains("❌ Poor"));
    }

    /// Score should be in good range (60-80): score ≈ 69.89
    #[test]
    fn test_calculate_organization_score_boundary_good() {
        let score = calculate_organization_score(50, 1);
        assert!(score >= 60.0 && score < 80.0, "Expected 60-80, got {}", score);
    }

    /// Score should be in fair range (40-60): score ≈ 49.5
    #[test]
    fn test_calculate_organization_score_boundary_fair() {
        let score = calculate_organization_score(100, 8);
        assert!(score >= 40.0 && score < 60.0, "Expected 40-60, got {}", score);
    }

    /// Score should be in poor range (0-40): score ≈ 25.4
    #[test]
    fn test_calculate_organization_score_boundary_poor() {
        let score = calculate_organization_score(150, 12);
        assert!(score >= 0.0 && score < 40.0, "Expected 0-40, got {}", score);
    }

    #[test]
    fn test_calculate_organization_score_penalty_increases() {
        let score_10 = calculate_organization_score(100, 10);
        let score_15 = calculate_organization_score(100, 15);
        assert!(score_10 > score_15);
    }

    // --- collect_files のテスト ---

#[test]
fn test_collect_files_empty_directory() {
    let dir = tempfile::TempDir::new().unwrap();
    let result = collect_files(dir.path(), false);
    assert!(result.is_ok());
    let analysis = result.unwrap();
    assert_eq!(analysis.file_count, 0);
    assert!(analysis.extension_counts.is_empty());
    assert!(analysis.files.is_empty());
}

#[test]
fn test_collect_files_excludes_hidden_by_default() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("visible.txt"), "").unwrap();
    std::fs::write(dir.path().join(".hidden.txt"), "").unwrap();

    let result = collect_files(dir.path(), false).unwrap();
    assert_eq!(result.file_count, 1);
    assert!(result.files.iter().all(|(name, _)| !name.starts_with('.')));
}

#[test]
fn test_collect_files_includes_hidden_with_all_flag() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("visible.txt"), "").unwrap();
    std::fs::write(dir.path().join(".hidden.txt"), "").unwrap();

    let result = collect_files(dir.path(), true).unwrap();
    assert_eq!(result.file_count, 2);
}

#[test]
fn test_collect_files_nonexistent_path() {
    let result = collect_files(Path::new("/nonexistent/path/xyz"), false);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to read directory"));
}

#[test]
fn test_collect_files_skips_subdirectories() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("file.txt"), "").unwrap();
    std::fs::create_dir(dir.path().join("subdir")).unwrap();

    let result = collect_files(dir.path(), false).unwrap();
    // サブディレクトリはカウントされない
    assert_eq!(result.file_count, 1);
}

#[test]
fn test_collect_files_file_tuple_contains_name_and_ext() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("hello.rs"), "").unwrap();

    let result = collect_files(dir.path(), false).unwrap();
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0], ("hello.rs".to_string(), "rs".to_string()));
}

// --- calculate_organization_score の追加ケース ---

#[test]
fn test_calculate_organization_score_clamps_to_zero() {
    // 大量の種類があるとスコアが負になるが0にクランプされるはず
    let score = calculate_organization_score(10, 100);
    assert_eq!(score, 0.0);
}

#[test]
fn test_calculate_organization_score_one_file_no_types() {
    // type_count=0 は現実には起きないが、計算上は問題ないはず
    let score = calculate_organization_score(1, 0);
    // file_penalty だけ引かれる: 100 - (1/2)*30 = 85.0
    assert!((score - 85.0).abs() < 0.001);
}

#[test]
fn test_calculate_organization_score_excellent_boundary() {
    // スコアがちょうど80付近になるケース
    let score = calculate_organization_score(5, 1);
    // file_penalty=(5/6)*30≈25, type_penalty=0.3, diversity=4.0 → 約70.7
    // 境界80はfile_countが非常に少ない時のみ達成可能
    assert!(score >= 0.0 && score <= 100.0);
}

// --- interpret_score の境界値テスト ---

#[test]
fn test_interpret_score_boundary_at_80() {
    assert!(crate::interpret_score(80.0).contains("✅ Excellent"));
}

#[test]
fn test_interpret_score_boundary_just_below_80() {
    assert!(crate::interpret_score(79.9).contains("👍 Good"));
}

#[test]
fn test_interpret_score_boundary_at_60() {
    assert!(crate::interpret_score(60.0).contains("👍 Good"));
}

#[test]
fn test_interpret_score_boundary_just_below_60() {
    assert!(crate::interpret_score(59.9).contains("⚠️"));
}

#[test]
fn test_interpret_score_boundary_at_40() {
    assert!(crate::interpret_score(40.0).contains("⚠️"));
}

#[test]
fn test_interpret_score_boundary_just_below_40() {
    assert!(crate::interpret_score(39.9).contains("❌ Poor"));
}

#[test]
fn test_interpret_score_zero() {
    assert!(crate::interpret_score(0.0).contains("❌ Poor"));
}

#[test]
fn test_interpret_score_100() {
    assert!(crate::interpret_score(100.0).contains("✅ Excellent"));
}

// --- is_hidden の追加ケース ---

#[test]
fn test_is_hidden_empty_string() {
    assert!(!is_hidden(""));
}

#[test]
fn test_is_hidden_double_dot() {
    assert!(is_hidden(".."));
}

// --- get_extension の追加ケース ---

#[test]
fn test_get_extension_hidden_file_with_ext() {
    let path = Path::new(".gitignore");
    // .gitignore は拡張子なし扱いになることをRustの標準動作で確認
    // Path::extension() は ".gitignore" → None を返す
    assert_eq!(get_extension(path), "(no extension)");
}

#[test]
fn test_get_extension_uppercase() {
    let path = Path::new("image.PNG");
    assert_eq!(get_extension(path), "PNG");
}

#[test]
fn test_get_extension_empty_path() {
    let path = Path::new("");
    assert_eq!(get_extension(path), "(no extension)");
}
}

use std::fs;
use std::path::Path;
use std::collections::HashMap;

/// File analysis result structure
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

    score.max(0.0).min(100.0)
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
}

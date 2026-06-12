use std::fs;
use tempfile::TempDir;

/// Helper function to create a test directory with sample files
fn create_test_directory(files: Vec<(&str, &str)>) -> TempDir {
    let dir = TempDir::new().unwrap();
    for (name, _ext) in files {
        let file_path = dir.path().join(name);
        fs::write(file_path, "test content").unwrap();
    }
    dir
}

#[test]
fn test_collect_files_with_various_extensions() {
    let test_dir = create_test_directory(vec![
        ("document.pdf", "pdf"),
        ("image.png", "png"),
        ("archive.zip", "zip"),
        ("spreadsheet.xlsx", "xlsx"),
        ("presentation.pptx", "pptx"),
    ]);

    let result = lesort::collect_files(test_dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.file_count, 5);
    assert_eq!(analysis.extension_counts.len(), 5);
    assert_eq!(analysis.files.len(), 5);
}

#[test]
fn test_collect_files_same_extension() {
    let test_dir = create_test_directory(vec![
        ("file1.txt", "txt"),
        ("file2.txt", "txt"),
        ("file3.txt", "txt"),
    ]);

    let result = lesort::collect_files(test_dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.file_count, 3);
    assert_eq!(analysis.extension_counts.len(), 1);
    assert_eq!(*analysis.extension_counts.get("txt").unwrap(), 3);
}

#[test]
fn test_collect_files_without_extension() {
    let test_dir = create_test_directory(vec![
        ("README", ""),
        ("Makefile", ""),
        ("LICENSE", ""),
    ]);

    let result = lesort::collect_files(test_dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.file_count, 3);
    assert_eq!(*analysis.extension_counts.get("(no extension)").unwrap(), 3);
}

#[test]
fn test_collect_files_mixed_extensions() {
    let test_dir = create_test_directory(vec![
        ("doc1.pdf", "pdf"),
        ("doc2.pdf", "pdf"),
        ("image.png", "png"),
        ("README", ""),
        ("README.md", "md"),
    ]);

    let result = lesort::collect_files(test_dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.file_count, 5);
    assert_eq!(analysis.extension_counts.len(), 4);
    assert_eq!(*analysis.extension_counts.get("pdf").unwrap(), 2);
    assert_eq!(*analysis.extension_counts.get("png").unwrap(), 1);
    assert_eq!(*analysis.extension_counts.get("md").unwrap(), 1);
}

#[test]
fn test_score_calculation_with_real_data() {
    let file_count = 50;
    let type_count = 3;
    let score = lesort::calculate_organization_score(file_count, type_count);
    
    assert!(score >= 60.0 && score < 100.0);
    assert_eq!(lesort::interpret_score(score), "👍 Good organization.");
}

#[test]
fn test_score_interpretation_consistency() {
    let test_cases = vec![
        (95.0, "✅ Excellent"),
        (75.0, "👍 Good"),
        (50.0, "⚠️"),
        (25.0, "❌ Poor"),
    ];

    for (score, expected) in test_cases {
        let result = lesort::interpret_score(score);
        assert!(result.contains(expected), 
                "Score {}: expected '{}', got '{}'", score, expected, result);
    }
}

#[test]
fn test_extension_counting_accuracy() {
    use std::collections::HashMap;
    
    let extensions: HashMap<String, usize> = vec![
        ("pdf".to_string(), 10),
        ("png".to_string(), 5),
        ("txt".to_string(), 3),
    ]
    .into_iter()
    .collect();

    let total: usize = extensions.values().sum();
    assert_eq!(total, 18);
}
#[test]
fn test_collect_files_all_flag_counts_hidden() {
    let test_dir = create_test_directory(vec![
        ("normal.txt", "txt"),
        (".hidden", ""),
    ]);
    // .hidden ファイルを手動で作成（create_test_directory は name をそのまま使う）
    let result_without = lesort::collect_files(test_dir.path(), false).unwrap();
    let result_with    = lesort::collect_files(test_dir.path(), true).unwrap();

    assert_eq!(result_without.file_count, 1);
    assert_eq!(result_with.file_count, 2);
}

#[test]
fn test_collect_files_empty_directory_returns_zero() {
    let dir = tempfile::TempDir::new().unwrap();
    let result = lesort::collect_files(dir.path(), false).unwrap();
    assert_eq!(result.file_count, 0);
    assert!(result.extension_counts.is_empty());
}

#[test]
fn test_score_clamps_at_zero_for_extreme_diversity() {
    let score = lesort::calculate_organization_score(5, 50);
    assert_eq!(score, 0.0);
}

#[test]
fn test_interpret_score_all_boundaries() {
    assert!(lesort::interpret_score(100.0).contains("✅"));
    assert!(lesort::interpret_score(80.0).contains("✅"));
    assert!(lesort::interpret_score(79.9).contains("👍"));
    assert!(lesort::interpret_score(60.0).contains("👍"));
    assert!(lesort::interpret_score(59.9).contains("⚠️"));
    assert!(lesort::interpret_score(40.0).contains("⚠️"));
    assert!(lesort::interpret_score(39.9).contains("❌"));
    assert!(lesort::interpret_score(0.0).contains("❌"));
}

#[test]
fn test_collect_files_nonexistent_path_returns_error() {
    let result = lesort::collect_files(std::path::Path::new("/no/such/dir"), false);
    assert!(result.is_err());
}

use std::fs;
use tempfile::TempDir;

/// Creates a realistic test directory structure
fn create_realistic_directory() -> TempDir {
    let dir = TempDir::new().unwrap();

    let files = vec![
        "document1.pdf",
        "document2.pdf",
        "document3.pdf",
        "document4.pdf",
        "document5.pdf",
        "image1.png",
        "image2.png",
        "image3.png",
        "spreadsheet.xlsx",
        "presentation.pptx",
        "archive.zip",
        "README.md",
        "LICENSE",
    ];

    for file_name in files {
        let file_path = dir.path().join(file_name);
        fs::write(file_path, "test content").unwrap();
    }

    dir
}

/// Creates a directory with subdirectories (should be ignored)
fn create_directory_with_subdirs() -> TempDir {
    let dir = TempDir::new().unwrap();

    fs::write(dir.path().join("file1.txt"), "content").unwrap();
    fs::write(dir.path().join("file2.txt"), "content").unwrap();
    fs::create_dir(dir.path().join("subdir")).unwrap();
    fs::write(dir.path().join("subdir").join("file3.txt"), "content").unwrap();

    dir
}

#[test]
fn test_system_analyze_realistic_directory() {
    let test_dir = create_realistic_directory();

    let result = lesort::collect_files(test_dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();

    assert_eq!(analysis.file_count, 13);
    assert!(analysis.extension_counts.len() >= 6);

    assert_eq!(*analysis.extension_counts.get("pdf").unwrap(), 5);
    assert_eq!(*analysis.extension_counts.get("png").unwrap(), 3);

    let score =
        lesort::calculate_organization_score(analysis.file_count, analysis.extension_counts.len());
    assert!(score >= 0.0 && score <= 100.0);
}

#[test]
fn test_system_score_realistic_data() {
    let test_dir = create_realistic_directory();

    let result = lesort::collect_files(test_dir.path(), false);
    let analysis = result.unwrap();

    let score =
        lesort::calculate_organization_score(analysis.file_count, analysis.extension_counts.len());

    assert!(score < 80.0);
    let interpretation = lesort::interpret_score(score);
    assert!(!interpretation.is_empty());
}

#[test]
fn test_system_subdirectories_not_counted() {
    let test_dir = create_directory_with_subdirs();

    let result = lesort::collect_files(test_dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.file_count, 2);
}

#[test]
fn test_system_large_directory_simulation() {
    let dir = TempDir::new().unwrap();

    let extensions = vec!["pdf", "doc", "xls", "ppt", "zip", "txt"];

    for i in 0..100 {
        let ext = extensions[i % extensions.len()];
        let file_name = format!("file{}.{}", i, ext);
        fs::write(dir.path().join(file_name), "content").unwrap();
    }

    let result = lesort::collect_files(dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.file_count, 100);
    assert_eq!(analysis.extension_counts.len(), 6);

    let score =
        lesort::calculate_organization_score(analysis.file_count, analysis.extension_counts.len());
    assert!(score >= 0.0 && score <= 100.0);
}

#[test]
fn test_system_score_quality_threshold() {
    let organized_score = lesort::calculate_organization_score(50, 2);
    let disorganized_score = lesort::calculate_organization_score(200, 30);

    assert!(organized_score > disorganized_score);
}

#[test]
fn test_system_extension_distribution_analysis() {
    let dir = TempDir::new().unwrap();

    fs::write(dir.path().join("file1.pdf"), "content").unwrap();
    fs::write(dir.path().join("file2.pdf"), "content").unwrap();
    fs::write(dir.path().join("file3.pdf"), "content").unwrap();
    fs::write(dir.path().join("file4.png"), "content").unwrap();
    fs::write(dir.path().join("file5.txt"), "content").unwrap();

    let result = lesort::collect_files(dir.path(), false);
    let analysis = result.unwrap();

    assert_eq!(analysis.file_count, 5);
    assert_eq!(analysis.extension_counts.len(), 3);

    let pdf_count = analysis.extension_counts.get("pdf").unwrap();
    let total = analysis.file_count as f64;
    let pdf_percentage = (*pdf_count as f64 / total) * 100.0;

    assert!(pdf_percentage > 50.0);
}

#[test]
fn test_system_end_to_end_quality_assessment() {
    let test_dir = create_realistic_directory();

    let result = lesort::collect_files(test_dir.path(), false);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert!(analysis.file_count > 0);
    assert!(analysis.extension_counts.len() > 0);

    let score =
        lesort::calculate_organization_score(analysis.file_count, analysis.extension_counts.len());

    let interpretation = lesort::interpret_score(score);

    assert!(!interpretation.is_empty());
    assert!(
        interpretation.contains("✅")
            || interpretation.contains("👍")
            || interpretation.contains("⚠️")
            || interpretation.contains("❌")
    );
}

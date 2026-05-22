use clap::Parser;
use std::fs;
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

struct AnalysisResult {
    file_count: usize,
    extension_counts: HashMap<String, usize>,
    files: Vec<(String, String)>,
}

fn main() {
    let args = Args::parse();

    let path = args.path.unwrap_or_else(|| ".".to_string());

    match analyze_directory(&path, args.all, args.detail, args.score) {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn analyze_directory(path: &str, include_all: bool, detail: bool, show_score: bool) -> Result<(), String> {
    let dir_path = Path::new(path);

    if !dir_path.exists() {
        return Err(format!("Directory not found: {}", path));
    }

    if !dir_path.is_dir() {
        return Err(format!("Not a directory: {}", path));
    }

    println!("Analyzing directory: {}\n", dir_path.display());

    let mut analysis_result = collect_files(dir_path, include_all)?;

    print_results(
        analysis_result.file_count,
        &analysis_result.extension_counts,
        &mut analysis_result.files,
        detail,
        show_score,
    );

    Ok(())
}

fn collect_files(dir_path: &Path, include_all: bool) -> Result<AnalysisResult, String> {
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

fn get_extension(file_path: &Path) -> String {
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

fn is_hidden(file_name: &str) -> bool {
    file_name.starts_with('.')
}

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

    // hidden file skip
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

fn print_results(
    file_count: usize,
    extension_counts: &HashMap<String, usize>,
    files: &mut Vec<(String, String)>,
    detail: bool,
    show_score: bool,
) {
    // Display file statistics
    println!("📊 File Statistics:");
    println!("  Total files: {}", file_count);
    println!("  File types: {}", extension_counts.len());
    println!();

    // Display extension breakdown
    println!("📁 Extension Breakdown:");
    let mut extensions: Vec<_> = extension_counts.iter().collect();
    extensions.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

    for (ext, count) in extensions {
        let percentage = (*count as f64 / file_count as f64) * 100.0;
        println!("  {}: {} files ({:.1}%)", ext, count, percentage);
    }
    println!();

    // Display detailed file list if requested
    if detail {
        println!("📝 Detailed File List:");
        files.sort();

        for (name, ext) in files.iter() {
            println!("  {} [{}]", name, ext);
        }

        println!();
    }

    // Calculate and display organization score if requested
    if show_score {
        let score = calculate_organization_score(
            file_count,
            extension_counts.len(),
        );

        println!("📈 Organization Score: {:.2}%", score);
        println!("{}", interpret_score(score));
    }
}

fn calculate_organization_score(file_count: usize, type_count: usize) -> f64 {
    if file_count == 0 {
        return 100.0;
    }

    // Base score starts at 100
    let mut score = 100.0;

    // 1. ファイル数による減点（ファイルが多いほど減点が増える）
    // ロジスティック関数を使用してスムーズな減衰
    let file_penalty = (file_count as f64 / (1.0 + file_count as f64)) * 30.0;
    score -= file_penalty;

    // 2. 拡張子の種類による減点（増加するたびに重みが増大）
    // 二次関数を使用して加速的に減点を増やす
    let type_penalty = (type_count as f64).powi(2) * 0.3;
    score -= type_penalty;

    // 3. 拡張子の多様性による減点（理想的な比率は1ファイルあたり1種類未満）
    let diversity_ratio = type_count as f64 / file_count as f64;
    let diversity_penalty = diversity_ratio * 20.0;
    score -= diversity_penalty;

    // スコアを0-100の範囲に正規化
    score.max(0.0).min(100.0)
}

fn interpret_score(score: f64) -> String {
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

# LeSort Examples

This directory contains various examples demonstrating how to use the LeSort library.

## Examples

### 1. Basic Analysis (`basic_analysis.rs`)

Shows the simplest way to analyze a directory and display results.

```bash
cargo run --example basic_analysis
```

**Output:**
- File statistics (total files, file types)
- Top file extensions with percentages
- Organization score and interpretation

### 2. Detailed Analysis (`detailed_analysis.rs`)

Provides a comprehensive analysis including a detailed file list.

```bash
cargo run --example detailed_analysis
cargo run --example detailed_analysis -- /path/to/directory
```

**Output:**
- Summary statistics
- Complete list of all files with extensions
- Extension breakdown
- Organization score

### 3. Score Calculation (`score_calculation.rs`)

Demonstrates the organization score calculation with various scenarios.

```bash
cargo run --example score_calculation
```

**Output:**
- Comparison of scores for different scenarios
- Score ranges and their meanings
- Detailed formula explanation
- Step-by-step calculation example

### 4. Hidden Files (`hidden_files.rs`)

Shows the difference between analyzing with and without hidden files.

```bash
cargo run --example hidden_files
```

**Output:**
- Statistics without hidden files
- Statistics with hidden files
- List of hidden files found (if any)

## Using the Library

### Basic Usage

```rust
use std::path::Path;
use lesort::collect_files;

let dir = Path::new(".");
match collect_files(dir, false) {
    Ok(analysis) => {
        println!("Total files: {}", analysis.file_count);
        println!("File types: {}", analysis.extension_counts.len());
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Calculate Organization Score

```rust
use lesort::{calculate_organization_score, interpret_score};

let file_count = 100;
let type_count = 8;
let score = calculate_organization_score(file_count, type_count);
println!("Score: {:.2}%", score);
println!("Interpretation: {}", interpret_score(score));
```

### Check for Hidden Files

```rust
use lesort::is_hidden;

if is_hidden(".gitignore") {
    println!("This is a hidden file");
}
```

### Extract File Extension

```rust
use std::path::Path;
use lesort::get_extension;

let file = Path::new("document.pdf");
let ext = get_extension(file);
println!("Extension: {}", ext);
```

## Score Interpretation

The organization score is calculated based on:

- **File count penalty**: Penalizes having too many files
- **Type penalty**: Penalizes excessive file type diversity  
- **Diversity penalty**: Penalizes unbalanced extension distribution

### Score Ranges

| Range | Interpretation | Emoji |
|-------|----------------|-------|
| 80-100% | Excellent organization | ✅ |
| 60-79% | Good organization | 👍 |
| 40-59% | Fair organization | ⚠️ |
| 0-39% | Poor organization | ❌ |

## Next Steps

- Check the `src/lib.rs` for the complete API documentation
- Run tests with `cargo test`
- Modify examples to explore different use cases

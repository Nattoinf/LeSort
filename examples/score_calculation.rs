//! Organization score calculation example
//!
//! This example shows how to manually calculate and interpret
//! organization scores for different scenarios.
//!
//! Run with: `cargo run --example score_calculation`

use lesort::{calculate_organization_score, interpret_score};

fn main() {
    println!("=== LeSort Organization Score Calculation Example ===");
    println!();

    display_scenario_analysis();
    println!();
    display_score_ranges();
    println!();
    display_formula_explanation();
    println!();
    display_detailed_example();
}

/// Display analysis of different scenarios
fn display_scenario_analysis() {
    let scenarios = vec![
        ("Empty directory", 0, 0),
        ("Single file", 1, 1),
        ("Well organized", 50, 3),
        ("Moderately organized", 100, 8),
        ("Poorly organized", 150, 12),
        ("Very disorganized", 200, 25),
        ("Extremely disorganized", 500, 50),
    ];

    println!("Scenario Analysis:");
    println!();
    println!(
        "{:<25} {:>10} {:>5} {:>10} {:<30}",
        "Scenario", "Files", "Types", "Score", "Interpretation"
    );
    println!("{}", "-".repeat(82));

    for (name, file_count, type_count) in scenarios {
        let score = calculate_organization_score(file_count, type_count);
        let interpretation = interpret_score(score);
        let clean = remove_emoji(&interpretation);

        println!(
            "{:<25} {:>10} {:>5} {:>9.2}% {}",
            name, file_count, type_count, score, clean
        );
    }
}

/// Remove emoji from interpretation string
fn remove_emoji(text: &str) -> String {
    text.replace("✅", "")
        .replace("👍", "")
        .replace("⚠️", "")
        .replace("❌", "")
        .trim()
        .to_string()
}

/// Display score ranges and their meanings
fn display_score_ranges() {
    println!("Score Ranges:");
    println!("  80-100%: Excellent organization ✅");
    println!("  60-79%:  Good organization 👍");
    println!("  40-59%:  Fair organization ⚠️");
    println!("  0-39%:   Poor organization ❌");
}

/// Display formula explanation
fn display_formula_explanation() {
    println!("Score Calculation Formula:");
    println!();
    println!("  score = 100");
    println!("         - (file_count / (1 + file_count)) × 30      [file penalty]");
    println!("         - (type_count)² × 0.3                       [type penalty]");
    println!("         - (type_count / file_count) × 20             [diversity penalty]");
    println!();
    println!("  Final score is clamped to range [0, 100]");
}

/// Display detailed example with step-by-step calculation
fn display_detailed_example() {
    println!("Detailed Example: 100 files with 8 file types");
    println!();

    let file_count = 100.0_f64;
    let type_count = 8.0_f64;
    let file_penalty = (file_count / (1.0 + file_count)) * 30.0;
    let type_penalty = type_count.powi(2) * 0.3;
    let diversity_penalty = (type_count / file_count) * 20.0;
    let score = 100.0 - file_penalty - type_penalty - diversity_penalty;

    println!("  File penalty:      {:.2}", file_penalty);
    println!("  Type penalty:      {:.2}", type_penalty);
    println!("  Diversity penalty: {:.2}", diversity_penalty);
    println!("  ─────────────────────────────");
    println!(
        "  Total penalty:     {:.2}",
        file_penalty + type_penalty + diversity_penalty
    );
    println!("  Final score:       {:.2}%", score);
    println!("  Interpretation:    {}", interpret_score(score));
}

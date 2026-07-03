# LeSort

![License](https://img.shields.io/badge/license-MIT-blue.svg)

![Version](https://img.shields.io/badge/version-0.1.0-green.svg)

[![Coverage Status](https://coveralls.io/repos/github/Nattoinf/LeSort/badge.svg?branch=main)](https://coveralls.io/github/Nattoinf/LeSort?branch=main)

![Build Status](https://github.com/Nattoinf/LeSort/actions/workflows/build.yaml/badge.svg)


- A CLI tool that reimplements the Unix `ls` command

# Overview
- LeSort is a lightweight CLI tool that analyzes directory organization.
- It evaluates files based on their extensions and naming patterns, then calculates an organization score.
- The goal is to help users keep their directories clean and well organized.

Features
- 📁 Analyze directory contents
- 📊 Count file extensions
- 📂 Include hidden files (--all)
- 📋 Show detailed file information (--detail)
- 📈 Calculate an organization score (--score)
- ⚡ Fast and lightweight CLI application
- 🦀 Written in Rust

# Installation
Build from source

- **Clone the repository**

git clone https://github.com/Nattoinf/LeSort.git
cd LeSort

- **Build the project**

cargo build --release

- **The executable will be generated at**

target/release/lesort

- **You can also install it locally with**

cargo install --path .

# Usage

File organization analyzer and scoring tool

Usage: lesort [OPTIONS] [PATH]

Arguments:
[PATH]
Target directory to analyze (default: current directory)

Options:
-a, --all
Include hidden files in analysis

-d, --detail
Show detailed classification of files

-s, --score
Display organization score

-h, --help
Print this help message

-V, --version
Print version information

Description:
LeSort analyzes files in a directory based on their extensions and naming patterns.
It evaluates whether the directory structure is well-organized and outputs a score.

## Examples

- **Analyze the current directory**

lesort

- **Analyze another directory**

lesort ./src

- **Include hidden files**

lesort -a

- **Display the organization score**

lesort -s

- **Display detailed file information**

lesort -d

- **Combine multiple options**

lesort -a -d -s

# Scoring Algorithm

LeSort evaluates directory organization using a three-stage penalty system.

The score starts at 100 points, and penalties are applied based on directory complexity.

Final Score =
100
- File Count Penalty
- File Type Penalty
- Diversity Penalty
## 1. File Count Penalty

Directories containing more files become increasingly difficult to manage.

file_penalty =
(file_count / (1 + file_count)) × 30

**Characteristics**

- Maximum penalty: 30 points
- Smoothly converges as the number of files increases
## 2. File Type Penalty

Having many different file types generally indicates that unrelated files are mixed together.

type_penalty =
(type_count)² × 0.3

Example penalties:

|File Types| Penalty |
|----------|---------|
|     1    |   0.3  |
|     5    |   7.5  |
|    10    |  30.0  |
|    14    |  58.8  |
## 3. Diversity Penalty

This penalty considers the ratio between the number of file types and the total number of files.

**diversity_penalty =**
**(type_count / file_count) × 20**

Directories containing many different file types relative to the total number of files receive a larger penalty.

## Score Interpretation
| Score | Evaluation |
|------:|------------|
| 80–100 | ✅ Excellent organization |
| 60–79  | 👍 Good organization |
| 40–59  | ⚠️ Fair organization |
| 0–39   | ❌ Poor organization |

## Example Calculations
Example 1
Files: 50
File types: 1
File penalty      = 29.41
Type penalty      = 0.30
Diversity penalty = 0.40

Final score = 69.89%

- Result

👍 Good organization
Example 2
Files: 100
File types: 10
File penalty      = 29.70
Type penalty      = 30.00
Diversity penalty = 2.00

Final score = 38.30%

- Result

❌ Poor organization
Example 3
Files: 213
File types: 14
File penalty      = 29.86
Type penalty      = 58.80
Diversity penalty = 1.31

Final score = 10.03%

Result:

❌ Poor organization


# Testing

- **Run all tests**

cargo test

- **Generate a coverage report**

cargo llvm-cov --html

- **Run Clippy**

cargo clippy -- -D warnings

# License
- MIT License

# Author
- Shun Naito



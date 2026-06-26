---

## title: "LeSort"

# LeSort

LeSort is a command-line tool written in Rust for analyzing directory structures and evaluating file organization quality.

## Overview

LeSort scans a directory and provides:

* File count statistics
* File extension distribution
* Hidden file analysis
* Organization score evaluation
* Detailed file listings

The goal is to help users understand how files are organized and identify opportunities for improving directory management.

## Features

### File Analysis

Analyze all files within a directory and generate statistics about file types and distribution.

### Organization Score

LeSort calculates an organization score based on:

* Number of files
* Number of file types
* Diversity of file extensions

This provides a simple indicator of how well a directory is organized.

### Hidden File Support

Optionally include hidden files when performing analysis.

### Detailed Reports

Display individual file information alongside summary statistics.

## Example

```bash
cargo run -- -d ~/Documents
```

Example output:

```text
📊 File Statistics:
  Total files: 120
  File types: 8

📁 Extension Breakdown:
  pdf: 45 files (37.5%)
  docx: 30 files (25.0%)
  txt: 15 files (12.5%)

📈 Organization Score: 82.4%
```

## Documentation

* Installation
* Usage
* Examples
* Scoring System
* Testing
* Development

## Source Code

GitHub Repository:

https://github.com/Nattoinf/LeSort

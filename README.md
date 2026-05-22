# LeSort

![License](https://img.shields.io/badge/license-MIT-blue.svg)

![Version](https://img.shields.io/badge/version-0.1.0-green.svg)

[![Coverage Status](https://coveralls.io/repos/github/Nattoinf/LeSort/badge.svg?branch=main)](https://coveralls.io/github/Nattoinf/LeSort?branch=main)

![Build Status](https://github.com/Nattoinf/LeSort/actions/workflows/build.yaml/badge.svg)


- Unixのlsコマンドを再実装したCLIツール

# Overview
- フォルダ内のファイル一覧を解析し、拡張子や命名規則に基づいて分類の妥当性を評価するツール
- ファイルの種類や命名パターンから、整理状態が適切かどうかをスコアとして可視化する
- ディレクトリ構成の改善や整理の指針を提供することを目的とする

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

Examples:
lesort
lesort ./src
lesort -a -s

# Scoring Algorithm

LeSortは、ディレクトリの整理状態を**3段階の減点システム**によって評価します。

## スコア計算方式

基本スコア: **100点**から開始

```
最終スコア = 100 - ファイル数減点 - 拡張子種類減点 - 多様性減点
```

### 1. ファイル数による減点（最大30点）

ファイル数が多いほど、ディレクトリが複雑になるため減点が増えます。
ロジスティック関数でスムーズに減衰：

```
file_penalty = (file_count / (1 + file_count)) × 30
```

- ファイル数が少ないほど減点が少ない
- ファイル数が増えても最大30点に収束

### 2. 拡張子の種類による減点（加速的に増加）

拡張子の種類が増えるたびに、減点の重みが**増大**します。
二次関数を使用：

```
type_penalty = (type_count)² × 0.3
```

**例：**
- 1種類：0.3点
- 5種類：7.5点
- 10種類：30点
- 14種類：58.8点

### 3. ファイル多様性による減点（最大20点）

ファイル数に対する拡張子の種類の比率。
理想的には1ファイルあたり1種類未満：

```
diversity_penalty = (type_count / file_count) × 20
```

## スコア評価基準

| スコア | 評価 | 状態 |
|--------|------|------|
| 80% - 100% | ✅ Excellent organization! | 非常に良好 |
| 60% - 79% | 👍 Good organization. | 良好 |
| 40% - 59% | ⚠️ Fair organization. Could be improved. | 改善推奨 |
| 0% - 39% | ❌ Poor organization. Consider reorganizing. | 要再整理 |

## 計算例

### 例1：整理された小規模ディレクトリ
- ファイル数: 50
- 拡張子種類: 1

計算：
```
- ファイル数減点: (50 / 51) × 30 = 29.41点
- 拡張子種類減点: 1² × 0.3 = 0.3点
- 多様性減点: (1 / 50) × 20 = 0.4点

スコア = 100 - 29.41 - 0.3 - 0.4 = 69.89%
→ 👍 Good organization.
```

### 例2：やや雑然としたディレクトリ
- ファイル数: 100
- 拡張子種類: 10

計算：
```
- ファイル数減点: (100 / 101) × 30 = 29.70点
- 拡張子種類減点: 10² × 0.3 = 30点
- 多様性減点: (10 / 100) × 20 = 2点

スコア = 100 - 29.70 - 30 - 2 = 38.30%
→ ❌ Poor organization. Consider reorganizing.
```

### 例3：非常に雑然としたディレクトリ
- ファイル数: 213
- 拡張子種類: 14

計算：
```
- ファイル数減点: (213 / 214) × 30 = 29.86点
- 拡張子種類減点: 14² × 0.3 = 58.8点
- 多様性減点: (14 / 213) × 20 = 1.31点

スコア = 100 - 29.86 - 58.8 - 1.31 = 10.03%
→ ❌ Poor organization. Consider reorganizing.
```

# Installation

# Lisence
- MIT Lisense

# Author
- Naito Shun

# About



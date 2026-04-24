[![Coverage Status](https://coveralls.io/repos/github/Nattoinf/LeSort/badge.svg?branch=main)](https://coveralls.io/github/Nattoinf/LeSort?branch=main)

![Build Status](https://github.com/Nattoinf/LeSort/actions/workflows/build.yaml/badge.svg)


# LeSort
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

# Installation

# Lisence
- MIT Lisense

# Author
- Naito Shun

# About



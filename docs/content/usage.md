
----------
title: "Usage"
weight: 20
----------

# Usage

## 基本的な使用方法

```bash
cargo run -- -d ~/Documents
```

## 隠しファイルも含めて解析

```bash
cargo run -- -d ~/Documents -a
```

## 詳細表示

```bash
cargo run -- -d ~/Documents --detail
```

## スコア表示

```bash
cargo run -- -d ~/Documents --score
```

## 出力例

```text
Analyzing directory: ~/Documents

File Statistics:
  Total files: 120
  File types: 8

Extension Breakdown:
  pdf: 45 files
  docx: 30 files
  txt: 15 files

Organization Score: 76.2%
```

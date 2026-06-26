---

title: "Installation"
weight: 10
----------

# Installation

## 動作環境

* Rust 1.80 以上
* Cargo

## リポジトリの取得

```bash
git clone https://github.com/Nattoinf/LeSort.git
cd LeSort
```

## ビルド

```bash
cargo build --release
```

## テスト実行

```bash
cargo test
```

## 静的解析

```bash
cargo clippy -- -D warnings
```



---

title: "Development"
weight: 60
----------

# Development

## プロジェクト構成

```text
LeSort/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
├── examples/
├── docs/
├── Cargo.toml
└── README.md
```

## 開発方針

* 関数単位で責務を分離
* Clippy Warning を 0 に維持
* テストによる品質保証
* GitHub Actions による CI

## 品質管理

ビルド:

```bash
cargo build
```

静的解析:

```bash
cargo clippy -- -D warnings
```

テスト:

```bash
cargo test
```

カバレッジ測定:

```bash
cargo llvm-cov --html
```

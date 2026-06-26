
---

title: "Testing"
weight: 50
----------

# Testing

LeSort では複数レベルのテストを実施しています。

## Unit Tests

ライブラリ関数単位のテスト

```bash
cargo test
```

## Integration Tests

複数モジュールを組み合わせたテスト

```text
tests/integration_tests.rs
```

## System Tests

実際の利用を想定したテスト

```text
tests/system_tests.rs
```

## Coverage

```bash
cargo llvm-cov --html
```

現在のカバレッジは約 76 % です。

---

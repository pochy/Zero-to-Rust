# perf_log_rust

Rust 側の大量ログ処理比較実装です。

std streaming 版:

```bash
cargo run -p perf_log_rust -- data/perf_logs/sample_10k.jsonl
```

rayon 発展版:

```bash
cargo run -p perf_log_rust --features rayon --bin analyze_rayon -- data/perf_logs/sample_10k.jsonl
```

release build で比較してください。

```bash
cargo build --release -p perf_log_rust
time target/release/perf_log_rust data/perf_logs/sample_10k.jsonl
```

## 次に読む

- 前へ: [projects/perf_log_python/README.md](../perf_log_python/README.md)
- 次へ: [docs/labs/PERFORMANCE_RESULTS.md](../../docs/labs/PERFORMANCE_RESULTS.md)
- 関連: [docs/labs/PERFORMANCE_LAB.md](../../docs/labs/PERFORMANCE_LAB.md), [projects/PROJECT_WALKTHROUGH.md](../PROJECT_WALKTHROUGH.md), [docs/guide/FINAL_PROJECT_SPEC.md](../../docs/guide/FINAL_PROJECT_SPEC.md)

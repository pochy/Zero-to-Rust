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

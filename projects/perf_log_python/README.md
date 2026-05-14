# perf_log_python

Python 側の大量ログ処理比較実装です。

```bash
python3 projects/perf_log_python/analyze_naive.py data/perf_logs/sample_10k.jsonl
python3 projects/perf_log_python/analyze_streaming.py data/perf_logs/sample_10k.jsonl
```

`analyze_naive.py` は全行を読み込んでから集計します。`analyze_streaming.py` は 1 行ずつ処理します。

Rust との差を見る前に、Python 内でも「素朴に全部持つ」設計と「streaming で必要な状態だけ持つ」設計の違いを確認してください。

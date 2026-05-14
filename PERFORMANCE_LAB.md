# Python vs Rust Performance Lab

この lab は、Rust で作るメリットを自分の端末で数値として体感するための教材です。

同じ大量ログ処理を Python と Rust で実装し、次を比較します。

```text
実行時間
最大 RSS / メモリ使用量
起動時間
処理行数 / 秒
binary size
エラー行の扱い
```

Rust の「速いらしい」「メモリ効率がよいらしい」を、説明ではなく実測で確認します。

## なぜ大量ログ処理か

Rust を学ぶ意味を体感しやすい題材はいくつかあります。

| 題材 | 体感できること | 初回教材としての評価 |
| --- | --- | --- |
| 大量ログ処理 | 速度、メモリ、streaming、HashMap、error handling | 最優先 |
| grep / 検索ツール | I/O、文字列処理、buffering、single binary 配布 | 良い |
| JSONL / CSV ETL | serde、Result、pipeline、data infra | 良い |
| 並列画像処理 | CPU bound、rayon、GIL との差 | 発展向け |
| 小さい HTTP API 負荷比較 | latency、throughput、concurrency | framework 差が入りやすい |

v1 では大量ログ処理を扱います。Web API benchmark より環境差が少なく、Python と Rust の実装方針の違いが見えやすいためです。

## データセット方針

元データは外部 dataset ではなく、自前で生成します。

理由:

```text
外部 dataset のライセンス確認が不要
個人情報や機密ログを含まない
サイズを自由に変えられる
seed 固定で再現性を保てる
Python/Rust の比較条件を揃えられる
CI では小さく、ローカルでは大きく実行できる
```

形式は JSONL です。

```json
{"ts":"2026-05-14T12:00:00Z","ip":"203.0.113.10","method":"GET","path":"/api/items","status":200,"bytes":1234,"latency_ms":18}
```

field:

```text
ts:
ISO-like UTC timestamp string.

ip:
documentation-safe IP range。

method:
GET、POST、PUT、DELETE。

path:
/api/items、/api/items/{id}、/api/login、/api/search、/static/app.js、/health、/metrics。

status:
200、201、204、400、401、404、429、500、503。

bytes:
response body bytes。

latency_ms:
処理 latency。
```

壊れた JSON 行も少し混ぜます。Rust の `Result` と Python の例外処理を比較するためです。

## データ生成

小さいサンプル:

```bash
python3 scripts/generate_perf_logs.py --rows 10000 --seed 42 --out data/perf_logs/sample_10k.jsonl
```

大きい benchmark:

```bash
python3 scripts/generate_perf_logs.py --rows 100000 --seed 42 --out /tmp/perf_logs_100k.jsonl
python3 scripts/generate_perf_logs.py --rows 1000000 --seed 42 --out /tmp/perf_logs_1m.jsonl
python3 scripts/generate_perf_logs.py --rows 10000000 --seed 42 --out /tmp/perf_logs_10m.jsonl
```

サイズの目安:

```text
10k rows:
動作確認用。Git に入れる。

100k rows:
軽い benchmark。生成して使う。

1m rows:
通常の性能差を見る。

10m rows:
Rust の差がかなり見える。Git には入れない。
```

## 実装

Python:

```bash
python3 projects/perf_log_python/analyze_naive.py data/perf_logs/sample_10k.jsonl
python3 projects/perf_log_python/analyze_streaming.py data/perf_logs/sample_10k.jsonl
```

Rust:

```bash
cargo run -p perf_log_rust -- data/perf_logs/sample_10k.jsonl
```

Rust rayon 版:

```bash
cargo run -p perf_log_rust --features rayon --bin analyze_rayon -- data/perf_logs/sample_10k.jsonl
```

## 集計内容

すべての実装は、同じ summary を JSON で出します。

```text
total_lines
ok_lines
broken_lines
total_bytes
avg_latency_ms
p95_latency_ms
status_counts
path_counts
top_10_ips
elapsed_ms
rows_per_second
```

`elapsed_ms` と `rows_per_second` は実行ごとに変わります。それ以外の集計値が Python/Rust で揃うことを確認してください。

## 計測

速度を測る:

```bash
time python3 projects/perf_log_python/analyze_streaming.py /tmp/perf_logs_1m.jsonl > /tmp/python_streaming_summary.json
time cargo run --release -p perf_log_rust -- /tmp/perf_logs_1m.jsonl > /tmp/rust_summary.json
```

`hyperfine` がある場合:

```bash
hyperfine \
  'python3 projects/perf_log_python/analyze_streaming.py /tmp/perf_logs_1m.jsonl' \
  'target/release/perf_log_rust /tmp/perf_logs_1m.jsonl'
```

Linux で最大 RSS を見る:

```bash
/usr/bin/time -v python3 projects/perf_log_python/analyze_streaming.py /tmp/perf_logs_1m.jsonl > /tmp/python_streaming_summary.json
/usr/bin/time -v target/release/perf_log_rust /tmp/perf_logs_1m.jsonl > /tmp/rust_summary.json
```

macOS で見る:

```bash
/usr/bin/time -l python3 projects/perf_log_python/analyze_streaming.py /tmp/perf_logs_1m.jsonl > /tmp/python_streaming_summary.json
/usr/bin/time -l target/release/perf_log_rust /tmp/perf_logs_1m.jsonl > /tmp/rust_summary.json
```

release binary を先に作る:

```bash
cargo build --release -p perf_log_rust
```

binary size:

```bash
ls -lh target/release/perf_log_rust
```

## 結果記録表

自分の環境で測った結果を書いてください。

| implementation | rows | elapsed | max RSS | rows/sec | notes |
| --- | --- | --- | --- | --- | --- |
| Python naive | 100k |  |  |  | 全行を保持 |
| Python streaming | 100k |  |  |  | 1 行ずつ処理 |
| Rust release | 100k |  |  |  | `BufRead` streaming |
| Rust rayon | 100k |  |  |  | 発展版 |

絶対値より、同じ端末での相対比較を重視してください。

## なぜ差が出るか

観察する観点:

```text
Python naive:
全行を list に読み込むため、メモリを使いやすい。

Python streaming:
1 行ずつ処理することで改善する。ただし object allocation と dynamic typing の cost は残る。

Rust streaming:
BufRead で 1 行ずつ処理し、必要な集計状態だけを所有する。

Rust Result:
壊れた行を分類し、処理全体を止めずに続行できる。

Rust HashMap:
集計に必要な key/value だけを持つ。

rayon:
CPU bound な parse / aggregation を並列化できる。ただし I/O や merge cost が支配的なら効果は限定される。
```

Rust のメリットは「常に何倍速い」ではありません。所有、借用、allocation、copy、error handling、並行処理を設計として扱えることです。その結果として、速度やメモリ使用量を制御しやすくなります。

## 次の発展

この lab の後は、次へ進めます。

```text
grep / search benchmark:
Level 4 と接続する。

JSONL / CSV ETL:
AI / data pipeline と接続する。

FastAPI vs axum:
Cloud Native Rust と接続する。

image processing:
rayon と CPU bound 並列処理を体感する。
```

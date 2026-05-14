# Performance Lab Results

このファイルは、[PERFORMANCE_LAB.md](PERFORMANCE_LAB.md) の測定例です。

数値は実行環境、CPU、storage、Python version、Rust build mode、同時に動いている process によって変わります。ここで見るべきものは絶対値ではなく、同じ端末・同じデータで比較した傾向です。

## 測定環境

```text
OS:
macOS

date:
2026-05-14

data:
/tmp/perf_logs_100k.jsonl

rows:
100,000

seed:
42

broken lines:
94
```

データ生成:

```bash
python3 scripts/generate_perf_logs.py --rows 100000 --seed 42 --out /tmp/perf_logs_100k.jsonl
```

Rust release build:

```bash
cargo build --release -p perf_log_rust --features rayon
```

## 測定コマンド

Python naive:

```bash
/usr/bin/time -l python3 projects/perf_log_python/analyze_naive.py /tmp/perf_logs_100k.jsonl > /tmp/perf_python_naive_100k.json
```

Python streaming:

```bash
/usr/bin/time -l python3 projects/perf_log_python/analyze_streaming.py /tmp/perf_logs_100k.jsonl > /tmp/perf_python_streaming_100k.json
```

Rust release:

```bash
/usr/bin/time -l target/release/perf_log_rust /tmp/perf_logs_100k.jsonl > /tmp/perf_rust_release_100k.json
```

Rust rayon:

```bash
/usr/bin/time -l target/release/analyze_rayon /tmp/perf_logs_100k.jsonl > /tmp/perf_rust_rayon_100k.json
```

## 結果

| implementation | program elapsed | real time | max RSS | rows/sec | notes |
| --- | ---: | ---: | ---: | ---: | --- |
| Python naive | 383.661 ms | 0.46 s | 156,876,800 bytes | 260,646 | 全行を list に保持 |
| Python streaming | 350.050 ms | 0.40 s | 17,940,480 bytes | 285,673 | 1 行ずつ処理 |
| Rust release | 77.835 ms | 0.49 s | 2,998,272 bytes | 1,284,771 | `BufRead` streaming |
| Rust rayon | 24.119 ms | 0.48 s | 20,758,528 bytes | 4,146,080 | `par_lines` + merge |

`program elapsed` は analyzer 自身が計測した処理時間です。`real time` は `/usr/bin/time -l` が見た process 全体の時間です。小さい 100k dataset では起動時間や shell / redirect の影響が見えやすいため、1m rows 以上でも測ってください。

## 同じ集計になっているか

4 実装の主要集計は一致しました。

```text
total_lines:
100000

ok_lines:
99906

broken_lines:
94

total_bytes:
2275324985

avg_latency_ms:
35.369

p95_latency_ms:
117
```

これは重要です。性能比較の前に、同じ処理をしていることを確認します。

## 観察

Python naive は全行を読み込んでから `json.loads` した event を `list` に保持します。そのため、この測定では最大 RSS が約 157 MB になりました。

Python streaming は 1 行ずつ処理するため、最大 RSS が約 18 MB まで下がりました。Python でも設計を変えればメモリ使用量は大きく改善します。

Rust release は 1 行ずつ読み、集計に必要な `HashMap` と latency vector だけを保持します。この測定では最大 RSS が約 3 MB でした。Rust の所有権と streaming 設計により、必要な状態だけを明確に持つ形になります。

Rust rayon は処理時間は短くなりましたが、最大 RSS は Rust streaming より増えました。並列化は無料ではありません。input 全体を `String` として読み、thread ごとの partial aggregate を merge するため、I/O や memory の tradeoff が出ます。

## この結果から分かること

```text
Python が常に悪いわけではない。
Python も streaming にすれば大きく改善する。
Rust は release build で処理時間とメモリ使用量を強く制御しやすい。
rayon は速くなる場合があるが、memory と merge cost を確認する必要がある。
debug build と release build を混同してはいけない。
```

Rust のメリットは、単に「速い」ではありません。

```text
何を所有するか
何を借りるか
どこで allocation するか
どの error を分類するか
どの処理を streaming できるか
どこを並列化できるか
```

これらを設計として扱えることが、速度とメモリ使用量の制御につながります。

## 次に測ること

100k rows は小さいため、次は 1m rows と 10m rows を測ります。

```bash
python3 scripts/generate_perf_logs.py --rows 1000000 --seed 42 --out /tmp/perf_logs_1m.jsonl
python3 scripts/generate_perf_logs.py --rows 10000000 --seed 42 --out /tmp/perf_logs_10m.jsonl
```

追加で見ること:

```text
Python naive がどこでメモリ的に厳しくなるか
Python streaming と Rust streaming の差が data size でどう変わるか
Rust rayon がどの size から有利になるか
latency vector を保持しない approximate p95 にすると memory がどう変わるか
serde_json parse が支配的か、HashMap aggregation が支配的か
```

## より公平に測るために

```text
同じ machine で測る
release build の Rust を使う
何度か実行して中央値を見る
可能なら hyperfine を使う
出力は /tmp に redirect する
他の重い process を止める
```

`hyperfine` がある場合:

```bash
hyperfine \
  'python3 projects/perf_log_python/analyze_streaming.py /tmp/perf_logs_1m.jsonl > /tmp/python_streaming_summary.json' \
  'target/release/perf_log_rust /tmp/perf_logs_1m.jsonl > /tmp/rust_summary.json'
```

この lab の目的は、Python を否定することではありません。Python と Rust の設計上の違いを、速度・メモリ・error handling の数字として観察することです。

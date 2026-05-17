# Performance Lab Results

このファイルは、[PERFORMANCE_LAB.md](PERFORMANCE_LAB.md) の測定例です。

数値は実行環境、CPU、storage、Python version、Rust build mode、同時に動いている process によって変わります。ここで見るべきものは絶対値ではなく、同じ端末・同じデータで比較した傾向です。

## 測定環境

```text
OS:
macOS

date:
2026-05-14

seed:
42
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

## 100k rows: 小規模確認

100k rows は、実装が正しく動くか、集計値が一致するかを確認するための小さい benchmark です。起動時間や shell / redirect の影響がまだ見えやすいため、Rust と Python の傾向を見るには 1m rows も測ります。

| implementation | program elapsed | real time | max RSS | rows/sec | notes |
| --- | ---: | ---: | ---: | ---: | --- |
| Python naive | 383.661 ms | 0.46 s | 156,876,800 bytes | 260,646 | 全行を list に保持 |
| Python streaming | 350.050 ms | 0.40 s | 17,940,480 bytes | 285,673 | 1 行ずつ処理 |
| Rust release | 77.835 ms | 0.49 s | 2,998,272 bytes | 1,284,771 | `BufRead` streaming |
| Rust rayon | 24.119 ms | 0.48 s | 20,758,528 bytes | 4,146,080 | `par_lines` + merge |

## 同じ集計になっているか

100k rows では、4 実装の主要集計が一致しました。

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

## 1m rows: 本命比較

1m rows では、処理時間とメモリ使用量の差がよりはっきり見えます。

データ生成:

```bash
python3 scripts/generate_perf_logs.py --rows 1000000 --seed 42 --out /tmp/perf_logs_1m.jsonl
```

測定コマンドは 100k rows と同じで、入力 path と出力 path だけを `1m` に変えます。

| implementation | program elapsed | real time | max RSS | rows/sec | notes |
| --- | ---: | ---: | ---: | ---: | --- |
| Python naive | 4516.838 ms | 4.95 s | 1,416,396,800 bytes | 221,393 | 全行を list に保持 |
| Python streaming | 3460.909 ms | 3.61 s | 30,883,840 bytes | 288,941 | 1 行ずつ処理 |
| Rust release | 715.234 ms | 1.36 s | 12,386,304 bytes | 1,398,144 | `BufRead` streaming |
| Rust rayon | 145.813 ms | 0.54 s | 165,134,336 bytes | 6,858,114 | `par_lines` + merge |

1m rows の主要集計も一致しました。

```text
total_lines:
1000000

ok_lines:
999012

broken_lines:
988

total_bytes:
22660713064

avg_latency_ms:
35.42

p95_latency_ms:
118
```

`program elapsed` では、Rust release は Python streaming より約 4.8 倍速く、Rust rayon は Python streaming より約 23.7 倍速い結果になりました。

最大 RSS では、Python naive は約 1.4 GB、Python streaming は約 31 MB、Rust release は約 12 MB でした。Python でも streaming によって大きく改善しますが、Rust streaming はさらに少ないメモリで処理できました。

Rust rayon は速い一方で、input 全体を読む実装のため最大 RSS が約 165 MB まで増えました。並列化は速さと memory の tradeoff を持つことが分かります。

## 観察

Python naive は全行を読み込んでから `json.loads` した event を `list` に保持します。そのため、1m rows では最大 RSS が約 1.4 GB になりました。

Python streaming は 1 行ずつ処理するため、1m rows でも最大 RSS が約 31 MB に収まりました。Python でも設計を変えればメモリ使用量は大きく改善します。

Rust release は 1 行ずつ読み、集計に必要な `HashMap` と latency vector だけを保持します。1m rows では最大 RSS が約 12 MB でした。Rust の所有権と streaming 設計により、必要な状態だけを明確に持つ形になります。

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

1m rows で傾向が見えたら、次は 10m rows を任意で測ります。

```bash
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

## 次に読む

- 前へ: [docs/labs/PERFORMANCE_LAB.md](PERFORMANCE_LAB.md)
- 次へ: [docs/tracks/FUTURE_RUST_DOMAINS.md](../tracks/FUTURE_RUST_DOMAINS.md)
- 関連: [projects/perf_log_python/README.md](../../projects/perf_log_python/README.md), [projects/perf_log_rust/README.md](../../projects/perf_log_rust/README.md)

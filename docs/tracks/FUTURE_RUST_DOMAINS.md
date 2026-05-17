# Future Rust Domains

この文書は、今後注目される分野・技術と Rust の相性を整理したものです。

Rust は万能言語ではありません。特に強いのは、次が同時に必要になる領域です。

```text
高い性能
メモリ安全性
並行処理の安全性
GC pause を避けたい実行特性
C/C++ に近い低レイヤー制御
長期保守
障害時の責任境界
```

結論として、Zero to Rust の次に狙うなら、優先度が高いのは次です。

```text
1. Cloud Native / AI Infrastructure
2. Data Systems / Storage
3. WebAssembly / Edge Computing
4. Systems / Security
5. Embedded / no_std
6. Developer Tools / CLI
```

この教材の延長として最も自然なのは、`final_kvs_server` を cloud native な Rust service へ育てることです。

Rust の速度やメモリ効率を先に体感したい場合は、[PERFORMANCE_LAB.md](../labs/PERFORMANCE_LAB.md) を使ってください。大量 JSONL ログ処理を Python と Rust で比較し、Cloud Native、Data Systems、AI / ML infrastructure に共通する data processing の基礎を測定できます。

## 1. Cloud Native / AI Infrastructure

今後かなり注目度が高い領域です。

Kubernetes、OpenTelemetry、service mesh、AI inference gateway、job runner、data pipeline などは、Rust の性能、安全性、低メモリ性、並行処理の強みが出やすい領域です。

Rust と相性が良い理由:

```text
高性能な service を作れる
低メモリな sidecar / agent に向く
proxy / gateway と相性が良い
observability collector や exporter を作りやすい
AI inference 周辺の data pipeline で効率を出しやすい
```

Rust で作るとよいもの:

```text
OpenTelemetry collector plugin 的なもの
AI inference gateway
job runner
queue worker
proxy
high-performance API gateway
Kubernetes controller
```

学ぶべき技術:

```text
tokio
axum
tower
hyper
tonic / gRPC
OpenTelemetry
Prometheus
Kubernetes
Docker / OCI
GitOps
```

この教材との接続:

```text
final_kvs_server
-> axum + tokio 化
-> tracing / metrics 強化
-> Docker 化
-> Kubernetes deploy
-> load test
-> runbook 強化
```

おすすめ度:

```text
最優先。
Zero to Rust の現在の題材を最も自然に実務へ伸ばせる。
```

## 2. Data Systems / Storage

Rust は KVS、database、query engine、storage engine と相性が良いです。

理由は、GC がなく、buffer、page、WAL、snapshot、lock、I/O の責任を明確にしやすいためです。

Rust で作るとよいもの:

```text
WAL-based KVS
LSM tree
queue
log-structured storage
query engine
vector index
stream processor
```

学ぶべき技術:

```text
WAL
snapshot
compaction
mmap
async I/O
benchmarking
property testing
fuzzing
failure injection
```

この教材との接続:

```text
kvs_std
-> final_kvs_server
-> snapshot
-> WAL compaction
-> benchmark
-> failure test
-> replication log
```

おすすめ度:

```text
非常に高い。
既に KVS、WAL、TTL を扱っているため、教材資産をそのまま伸ばせる。
```

## 3. WebAssembly / Edge Computing

Rust 公式も WebAssembly を主要領域として扱っています。

Wasm は browser 内だけでなく、edge、plugin、sandboxed runtime、serverless extension としても注目されています。

Rust と相性が良い理由:

```text
小さい binary を作りやすい
GC なしで実行特性を読みやすい
sandbox と相性が良い
plugin runtime に向く
host / guest 境界を型で設計しやすい
```

Rust で作るとよいもの:

```text
Wasm plugin system
edge function
画像 / 音声 / テキスト処理 module
policy engine
sandboxed extension runtime
```

学ぶべき技術:

```text
wasm32-wasi
WASI
component model
wasmtime
Spin
Cloudflare Workers
Fastly Compute
```

この教材との接続:

```text
KVS command parser
-> Wasm plugin として切り出す
-> host から呼ぶ
-> sandbox boundary を設計する
-> error と ABI を設計する
```

おすすめ度:

```text
かなり高い。
Rust の binary size、性能、sandbox の強みが出る。
```

## 4. Systems / Security

Rust は C/C++ に近い制御を持ちながら、所有権と借用で多くの memory safety 問題を防げます。

そのため、daemon、security-sensitive service、network service、filesystem tool、C/C++ library wrapper と相性が良いです。

Rust と相性が良い理由:

```text
C/C++ に近い制御
メモリ安全
thread safety
FFI 可能
GC なし
外部入力を大量に扱う service に強い
```

Rust で作るとよいもの:

```text
daemon
network service
packet processor
security scanner
filesystem tool
Linux service agent
C/C++ library wrapper
```

学ぶべき技術:

```text
unsafe
FFI
Linux system programming
io_uring
nix crate
eBPF 周辺
Miri
fuzzing
sanitizer
```

この教材との接続:

```text
final_kvs_server
-> systemd service 化
-> signal handling
-> file permission
-> socket tuning
-> fuzzing
-> unsafe boundary の最小化
```

おすすめ度:

```text
高い。
ただし難度も高いため、Advanced Track の unsafe / FFI / compiler internals と合わせて進める。
```

## 5. Embedded / no_std

Rust 公式は embedded も主要領域として扱っています。

heap なし、静的 allocation、C SDK との相互運用、interrupt と共有状態の設計などで Rust の所有権が効きます。

Rust と相性が良い理由:

```text
no_std
低メモリ
所有権による resource 管理
割り込みと共有状態の明示
C SDK と接続できる
```

Rust で作るとよいもの:

```text
sensor logger
embedded KVS
firmware component
device driver
CAN / UART / SPI tool
```

学ぶべき技術:

```text
no_std
embedded-hal
heapless
RTIC
defmt
probe-rs
Tock
```

おすすめ度:

```text
専門性を取りに行くなら高い。
target 固有の知識が必要なため、汎用 backend より学習の文脈を選ぶ。
```

## 6. Developer Tools / CLI

Rust 公式も CLI を主要領域として扱っています。

単一 binary 配布、高速起動、クロスプラットフォーム、ファイル処理、エラー設計、テストしやすさが強みです。

Rust と相性が良い理由:

```text
single binary
高速起動
クロスプラットフォーム
ファイル処理が得意
テストしやすい
ユーザーへ配布しやすい
```

Rust で作るとよいもの:

```text
grep / search tool
formatter
linter
migration tool
log analyzer
data converter
developer productivity tool
```

学ぶべき技術:

```text
clap
serde
tracing
ignore
walkdir
rayon
ratatui
insta
```

おすすめ度:

```text
最初の実戦に最適。
Cloud Native や Data Systems へ進む前の実用成果物として作りやすい。
```

## 7. AI / ML 周辺インフラ

Rust は model training 本体よりも、AI / ML を支える周辺インフラで強みを出しやすいです。

Rust と相性が良い理由:

```text
低レイテンシな gateway
高スループットな data pipeline
vector database 周辺
model serving の I/O
Python との FFI / extension 境界
```

Rust で作るとよいもの:

```text
inference gateway
batch job runner
feature pipeline
vector index service
embedding cache
Python extension module
```

学ぶべき技術:

```text
tokio
axum
tonic
pyo3
arrow / parquet 周辺
vector index
OpenTelemetry
```

おすすめ度:

```text
高い。
ただし ML model 自体より、周辺の data / serving / infra を狙う方が Rust の強みが出やすい。
```

## 優先順位

Zero to Rust の流れから考えると、次の順番が最も自然です。

```text
1. Cloud Native Rust
2. Data Systems Rust
3. WebAssembly / Edge Rust
4. Systems / Security Rust
5. Embedded / no_std Rust
6. Developer Tools / CLI
7. AI / ML Infrastructure Rust
```

最初に選ぶなら、次を推奨します。

```text
final_kvs_server を tokio + axum + tracing + metrics + Docker + Kubernetes に発展させる。
```

理由:

```text
今の教材資産を最大限使える
Rust の実務採用領域と合う
Cloud Native / AI infrastructure の流れにも合う
所有、失敗、共有、復旧、運用を全部使う
std-only から ecosystem へ責任を移す判断を実践できる
```

## 次に追加する教材案

次に教材として切り出すなら、候補名は次です。

```text
PRODUCTION_RUST_TRACK.md
CLOUD_NATIVE_RUST_TRACK.md
```

内容:

```text
final_kvs_server を axum 化
tokio runtime
structured tracing
Prometheus metrics
config file
graceful shutdown
request size limit
WAL compaction
snapshot
Docker image
Kubernetes manifest
load test
runbook
```

完了条件:

```text
Docker で起動できる
Kubernetes に deploy できる
health / metrics / logs を確認できる
WAL から復旧できる
load test で限界を説明できる
障害時の runbook を書ける
どの責任を crate / platform に任せたか説明できる
```

## 参考リンク

- Rust official: https://www.rust-lang.org/
- Rust official WebAssembly: https://www.rust-lang.org/what/wasm/
- Rust official CLI: https://www.rust-lang.org/what/cli/
- Rust official Embedded: https://www.rust-lang.org/what/embedded/
- CNCF Kubernetes / AI infrastructure survey: https://www.cncf.io/announcements/2026/01/20/kubernetes-established-as-the-de-facto-operating-system-for-ai-as-production-use-hits-82-in-2025-cncf-annual-cloud-native-survey/
- CNCF Dapr 2025 report: https://www.cncf.io/announcements/2025/04/01/cloud-native-computing-foundation-releases-2025-state-of-dapr-report-highlighting-adoption-trends-and-ai-innovations/
- Akamai WebAssembly edge/serverless: https://www.akamai.com/blog/cloud/unlocking-next-wave-edge-computing-serverless-webassembly
- OxidOS Automotive: https://oxidos.io/

## 次に読む

- 前へ: [docs/tracks/ADVANCED_TRACK.md](ADVANCED_TRACK.md)
- 次へ: [docs/tracks/COMPUTER_SCIENCE_TRACK.md](COMPUTER_SCIENCE_TRACK.md)
- 関連: [docs/INDEX.md](../INDEX.md), [docs/guide/ASSESSMENT.md](../guide/ASSESSMENT.md)

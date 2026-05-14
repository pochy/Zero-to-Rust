# CS 9 Exercises

## 1. 題材選択

次のどれかを選び、選んだ理由を書いてください。

```text
URL shortener
Redis 風 KVS
job queue
```

## 2. 設計文書

実装前に次を書いてください。

```text
data structure
API / protocol
storage
failure mode
concurrency
benchmark plan
```

テンプレート:

```text
computer_science/levels/cs_09_capstone/templates/CAPSTONE_DESIGN_TEMPLATE.md
```

## 3. 実装

最小機能だけを作ってください。

URL shortener:

```text
shorten
redirect lookup
duplicate handling
not found
```

KVS:

```text
set
get
delete
WAL
restore
```

job queue:

```text
enqueue
dequeue
ack
retry
dead letter
```

最初に参考実装を動かしてください。

```bash
rustc --edition=2021 computer_science/levels/cs_09_capstone/examples/capstone_kvs.rs -o /tmp/cs_capstone_kvs
printf 'SET name Rust\nGET name\nDELETE name\nGET name\n' | /tmp/cs_capstone_kvs /tmp/cs_capstone_kvs.wal

rustc --edition=2021 computer_science/levels/cs_09_capstone/examples/capstone_job_queue.rs -o /tmp/cs_capstone_job_queue
/tmp/cs_capstone_job_queue
```

参考実装をそのまま提出物にしないでください。次のどれかを追加して、自分の設計判断を入れます。

```text
KVS に TTL を追加する
KVS に recovery time 計測を追加する
job queue に max retry policy を設定できるようにする
job queue に dead letter 一覧を出す command を追加する
URL shortener として HTTP request parser に接続する
```

## 4. 測定

最低限、次を測ってください。

```text
処理件数
実行時間
memory usage
error case
recovery time
```

結果テンプレート:

```text
computer_science/levels/cs_09_capstone/templates/CAPSTONE_RESULTS_TEMPLATE.md
```

## 提出物

```text
CAPSTONE_DESIGN.md
CAPSTONE_RESULTS.md
source code
run command
```

必須ではないが推奨する追加提出物:

```text
failure_cases.md
benchmark_commands.sh
review_notes.md
```

## 進級チェック

```text
使った data structure と計算量を説明できるか
失敗時に何が保存され、何が失われるか説明できるか
測定結果から bottleneck の仮説を立てられるか
```

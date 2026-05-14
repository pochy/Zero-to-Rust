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

## 4. 測定

最低限、次を測ってください。

```text
処理件数
実行時間
memory usage
error case
recovery time
```

## 提出物

```text
CAPSTONE_DESIGN.md
CAPSTONE_RESULTS.md
source code
run command
```

## 進級チェック

```text
使った data structure と計算量を説明できるか
失敗時に何が保存され、何が失われるか説明できるか
測定結果から bottleneck の仮説を立てられるか
```


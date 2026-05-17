# CS 9: Capstone

## この Level でできるようになること

CS 1-8 の内容を使って、小さな system を設計、実装、測定、レビューできるようになります。

## まず知るべき言葉

```text
capstone
requirement
interface
invariant
failure mode
benchmark
observability
tradeoff
```

## なぜこれを学ぶのか

CS は分野ごとに分かれていますが、実務の system では一緒に現れます。

```text
URL shortener:
HashMap / DB index / HTTP / DNS / cache / transaction

Redis 風 KVS:
HashMap / WAL / TCP / memory / lock / persistence

job queue:
queue / retry / lock / transaction / worker / failure
```

最終課題では、1 つの system を作り、どの CS の考え方を使ったか説明します。

## 手順 1: 題材を選ぶ

次のどれかを選びます。

```text
小さな URL shortener
Redis 風の簡易 KVS
簡易 job queue
```

おすすめは、Zero to Rust の本編と接続しやすい Redis 風 KVS です。Web に寄せたい場合は URL shortener、並行処理と failure に寄せたい場合は job queue が向いています。

## 手順 2: CS 観点で設計する

必ず次を書きます。

```text
使う data structure
計算量
memory に持つもの
永続化するもの
network interface
failure mode
concurrency model
測定する指標
```

設計文書はテンプレートから始めます。

```bash
cp computer_science/levels/cs_09_capstone/templates/CAPSTONE_DESIGN_TEMPLATE.md /tmp/CAPSTONE_DESIGN.md
```

見るべき点:

```text
機能一覧より先に、data structure と failure mode を書く
何を memory に持ち、何を file / DB に保存するか分ける
どの操作の計算量を重視するか決める
```

## 手順 3: 測定して説明する

完成したら、速い/遅いを感想で言わず、測ります。

```text
request/sec
latency
memory usage
file size
startup time
recovery time
error rate
```

測った数字から、data structure、I/O、network、DB、lock のどこが効いているか説明します。

測定結果もテンプレートへ残します。

```bash
cp computer_science/levels/cs_09_capstone/templates/CAPSTONE_RESULTS_TEMPLATE.md /tmp/CAPSTONE_RESULTS.md
```

## 手順 4: 最小実装を動かす

Redis 風 KVS の最小例:

```bash
rustc --edition=2021 computer_science/levels/cs_09_capstone/examples/capstone_kvs.rs -o /tmp/cs_capstone_kvs
printf 'SET name Rust\nGET name\nDELETE name\nGET name\n' | /tmp/cs_capstone_kvs /tmp/cs_capstone_kvs.wal
```

見るべき点:

```text
HashMap が memory state を持つ
SET / DELETE は WAL に書く
GET は state を読むだけなので WAL に書かない
再起動時に WAL を replay して復元する
```

job queue の最小例:

```bash
rustc --edition=2021 computer_science/levels/cs_09_capstone/examples/capstone_job_queue.rs -o /tmp/cs_capstone_job_queue
/tmp/cs_capstone_job_queue
```

見るべき点:

```text
VecDeque が ready queue を持つ
HashMap が job id から job state を引く
ack / retry / dead letter は state transition である
attempts は failure handling の一部である
```

この 2 つは完成品ではありません。最終課題を作る前に、CS 1-8 の概念が 1 つの system にどう集まるかを見るための足場です。

## TypeScript / Go ならどう見えるか

TypeScript なら URL shortener や job queue の application logic は書きやすいです。Go なら HTTP server と worker が書きやすいです。Rust なら、memory、ownership、I/O failure、thread safety、persistence の責任が明確になります。

## よくあるつまずき

```text
機能を増やしすぎる
測定せずに速いと言う
failure mode を書かない
data structure の選択理由を書かない
DB や file に保存する責任を曖昧にする
```

## 次の Level に進む条件

この Level が最終課題です。完了条件は次です。

```text
1 つの system を動かせる
設計文書を書ける
測定結果を残せる
失敗時の挙動を説明できる
使った CS 概念を 5 つ以上説明できる
```

最低限、次のように説明してください。

```text
Data structure:
HashMap / VecDeque / BTreeMap / Vec のどれを使い、なぜか。

Algorithm:
lookup、scan、retry、restore の計算量はどうなるか。

Memory:
memory に置く state と file に残す state は何か。

I/O:
どの操作が file / network / stdout / stderr を使うか。

Failure:
invalid input、write failure、crash、duplicate、not found をどう扱うか。

Concurrency:
single-thread か、Mutex か、channel か。lock を持つ範囲はどこか。
```

## 公式 docs で確認する箇所

題材に応じて次を確認します。

```text
std::collections
std::io
std::net
std::thread
PostgreSQL docs
HTTP reference
```

## 次に読む

- 前へ: [computer_science/levels/cs_08_languages_compilers/exercises.md](../cs_08_languages_compilers/exercises.md)
- 次へ: [computer_science/levels/cs_09_capstone/exercises.md](exercises.md)
- 関連: [computer_science/CHECKPOINTS.md](../../CHECKPOINTS.md), [computer_science/SOLUTIONS.md](../../SOLUTIONS.md), [computer_science/glossary.md](../../glossary.md)

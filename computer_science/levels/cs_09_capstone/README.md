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


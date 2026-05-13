# Final Project Spec: std-only KVS Server

最終課題は、標準ライブラリだけで作るマルチスレッド KVS サーバーです。実装例は [projects/final_kvs_server](projects/final_kvs_server) にあります。

## 必須機能

```text
TCP command server
SET key value
SETEX key seconds value
GET key
DEL key
EXISTS key
TTL key
QUIT
WAL 永続化
TTL lazy expiration
簡易 admin HTTP
health check
metrics
unit tests
```

## 設計方針

```text
Store:
key/value と期限を所有する。

Command:
wire text を型に変換したもの。

Response:
表示前の結果。TCP 書き込みとは分ける。

WAL:
状態変更だけを記録する。読み取り操作は記録しない。

共有状態:
Store、metrics、WAL path を Mutex で守る。

admin HTTP:
学習用の最小実装。TLS、認証、完全な HTTP parser は範囲外。
```

## 失敗時の方針

```text
parse error:
ERROR を返し、Store は変更しない。

WAL write error:
Store は更新せず ERROR を返す。

GET missing:
正常な NOT_FOUND。エラーではない。

expired key:
読み取り時に削除する lazy expiration。

broken WAL:
起動時の復旧に失敗させる。
```

## 実行

```bash
cargo run -p final_kvs_server
```

別ターミナルから接続します。

```bash
nc 127.0.0.1 4000
SET name Rust
GET name
QUIT
```

admin HTTP:

```bash
curl http://127.0.0.1:4001/health
curl http://127.0.0.1:4001/metrics
curl http://127.0.0.1:4001/keys
```

## 完了条件

```text
所有、借用、失敗、共有、復旧、運用の設計判断を README に書ける
cargo test --workspace が通る
cargo clippy --workspace --all-targets が通る
WAL を削除した場合と残した場合の起動結果を説明できる
std-only の限界と、外部クレートへ移すべき責任を説明できる
```

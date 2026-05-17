# final_kvs_server

標準ライブラリだけで作る最終課題の KVS サーバーです。

この project は、Level 0-9 と `appendices/` の判断を 1 つに統合します。実務品質の HTTP/TLS/認証/構造化ログは範囲外ですが、所有、借用、失敗、共有、WAL、TTL、admin endpoint の責任境界を確認できます。

## 実行

```bash
cargo run -p final_kvs_server
```

既定値:

```text
APP_ADDR=127.0.0.1:4000
ADMIN_ADDR=127.0.0.1:4001
WAL_PATH=target/final_kvs_server.wal
```

TCP command:

```bash
nc 127.0.0.1 4000
SET name Rust
GET name
SETEX token 5 abc
TTL token
DEL name
QUIT
```

Admin:

```bash
curl http://127.0.0.1:4001/health
curl http://127.0.0.1:4001/metrics
curl http://127.0.0.1:4001/keys
```

## テスト

```bash
cargo test -p final_kvs_server
```

## 教材として読む順番

```text
README.md
DESIGN.md
../PROJECT_WALKTHROUGH.md
src/lib.rs
src/main.rs
RUNBOOK.md
EXERCISES.md
```

`DESIGN.md` はコードを責任境界として読むための文書です。`RUNBOOK.md` は障害を観察するための手順です。`EXERCISES.md` は、最終課題を自分の設計へ拡張するための演習です。

`src/lib.rs` は、次の順で読むと本編とのつながりが見えます。

```text
Command / Response / ParseError:
Level 5 と Level 6。文字列、正常な結果、入力エラーを分ける。

Store / Entry:
Level 3 と Level 8。key/value と TTL を所有する。

Metrics:
Level 8。運用で観察する値を持つ。

AppState:
Level 7 と Level 8。共有する実行時状態をまとめる。

parse_command:
Level 2 と Level 5。&str を借り、Command を返す。

run_tcp_server / run_admin_server:
Level 7 と Level 8。外側の I/O と中心ロジックをつなぐ。

tests:
Level 6。責任境界が壊れていないか確認する。
```

## 設計判断

```text
Store は key/value と expires_at を所有する。
Command は TCP 文字列を型にしたもの。
Response は wire text へ変換できる結果。
WAL は SET、SETEX、DEL だけを記録する。
WAL 書き込みに失敗した場合、Store は更新しない。
共有状態は Arc<Mutex<AppState>> に限定する。
admin HTTP は学習用の最小実装に限定する。
```

## std-only の限界

この project は学習目的です。実務では次を外部 crate や別コンポーネントへ任せる判断を検討します。

```text
HTTP parser
TLS
認証
構造化ログ
非同期 runtime
CLI parser
WAL compaction
```

## 次に読む

- 前へ: [projects/kvs_ecosystem/README.md](../kvs_ecosystem/README.md)
- 次へ: [projects/final_kvs_server/DESIGN.md](DESIGN.md)
- 関連: [projects/PROJECT_WALKTHROUGH.md](../PROJECT_WALKTHROUGH.md), [docs/guide/FINAL_PROJECT_SPEC.md](../../docs/guide/FINAL_PROJECT_SPEC.md)

# final_kvs_server Runbook

この runbook は、学習用サーバーで障害を再現し、確認し、説明するための手順です。

## 起動

```bash
APP_ADDR=127.0.0.1:4000 \
ADMIN_ADDR=127.0.0.1:4001 \
WAL_PATH=target/final_kvs_server.wal \
cargo run -p final_kvs_server
```

## Health Check

```bash
curl http://127.0.0.1:4001/health
```

期待値:

```text
ok
```

## Metrics

```bash
curl http://127.0.0.1:4001/metrics
```

見るべき点:

```text
commands:
正常に処理した command 数。

errors:
parse error や WAL write error の数。

started_at_unix:
起動時刻。
```

## TCP 動作確認

```bash
nc 127.0.0.1 4000
SET name Rust
GET name
QUIT
```

期待値:

```text
OK
VALUE Rust
BYE
```

## WAL 復旧確認

1. サーバーを起動する。
2. `SET name Rust` を送る。
3. サーバーを止める。
4. 同じ `WAL_PATH` で再起動する。
5. `GET name` が `VALUE Rust` を返すことを確認する。

この確認で見るべき点は、現在の `HashMap` ではなく、WAL の操作ログから状態が戻ることです。

## 起動しない

確認:

```text
APP_ADDR / ADMIN_ADDR が既に使われていないか
WAL_PATH の親ディレクトリへ書けるか
壊れた WAL 行がないか
```

判断:

```text
bind 失敗:
設定の問題。別ポートで起動する。

WAL restore 失敗:
復旧データの問題。壊れた行を特定し、バックアップから戻すか、教材として失敗を観察する。
```

## レスポンスが遅い

確認:

```text
metrics の commands が増えているか
WAL_PATH が遅い場所にないか
大量の /keys 呼び出しで Store 掃除が走っていないか
```

この実装は `Arc<Mutex<AppState>>` で全状態を守ります。WAL 書き込みや key 掃除中は他の操作が待ちます。これは学習用の単純化です。

## WAL が壊れた

確認:

```bash
sed -n '1,120p' target/final_kvs_server.wal
```

不正な行がある場合、起動時 restore は失敗します。

設計判断:

```text
壊れた行をスキップする:
可用性は上がるが、状態の正しさを説明しにくくなる。

起動を止める:
可用性は下がるが、復旧判断を人間に戻せる。
```

この教材では、起動を止める判断を優先します。

## 本番に出す前に必要なもの

```text
TLS
認証
request size limit
structured logging
WAL rotation / compaction
graceful shutdown
backpressure
load test
backup / restore procedure
```

# Solutions: final_kvs_server

## 1. WAL の壊れ方

回答例:

```text
起動時エラー:
BROKEN LINE は parse_command で UnknownCommand になり、restore_store_from_wal が InvalidData を返す。

スキップしない理由:
WAL は復旧の根拠なので、壊れた行を勝手に無視すると、どの操作が失われたか説明できなくなる。

スキップする設計のリスク:
可用性は上がるが、状態の正しさが曖昧になる。監査ログとしての価値が下がる。
```

許容できる別解:

```text
壊れた行を quarantine file に移し、起動は続ける。ただし、どの操作を捨てたか metrics と log に残す。
```

## 2. Metrics 追加

回答例:

```rust
struct Metrics {
    started_at: SystemTime,
    commands: u64,
    errors: u64,
    gets: u64,
    sets: u64,
    deletes: u64,
    not_found: u64,
}
```

counter を増やす場所:

```text
AppState::handle_command で Command の種類を見て増やす。
Response::NotFound が返ったとき not_found を増やす。
```

注意:

```text
Store の中で metrics を増やすと、Store が観測責任まで持つ。教材の設計では AppState がよい。
```

## 3. request size limit

回答例:

```text
上限:
学習用は 8 KiB。

超過時:
ERROR request too large を返して接続を閉じる。

WAL:
parse_command へ渡す前に拒否するため、Command が作られず WAL へ書かれない。
```

実装方針:

```text
reader.lines() では長すぎる行を読み切ってから検出することになる。
より厳密にするなら read_until で buffer length を見ながら読む。
```

## 4. graceful shutdown

回答例:

```text
std-only:
TcpListener を nonblocking にし、Arc<AtomicBool> の shutdown flag を見る。
admin endpoint に /shutdown を追加し、flag を true にする。

外部 crate:
ctrlc crate で Ctrl-C を受ける。
tokio なら signal handling と cancellation token を使う。
```

判断:

```text
教材では std-only で仕組みを学ぶ価値がある。
実務では signal handling を成熟 crate に任せる方がよい。
```

## 5. ecosystem 移行

回答例:

| 責任 | crate | 理由 |
| --- | --- | --- |
| TCP/HTTP server | `tokio`, `axum` | 非同期 I/O、router、HTTP 互換性 |
| JSON protocol | `serde`, `serde_json` | 手書き parser を避ける |
| CLI/config | `clap` | help、validation、env 連携 |
| logging | `tracing` | structured logs、span |
| error definitions | `thiserror`, `anyhow` | library と binary の責任分離 |
| metrics export | `metrics`, Prometheus exporter | 監視基盤へ接続 |

壊してはいけない API:

```text
Command の意味
Response の意味
WAL の復旧方針
GET missing はエラーではないという仕様
WAL 書き込み失敗時に Store を更新しない方針
```

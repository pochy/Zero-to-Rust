# final_kvs_server Design

この設計書は、最終課題をコードではなく責任境界として読むための文書です。

## Architecture

```text
TCP client
  |
  v
wire text
  |
  v
parse_command
  |
  v
Command
  |
  v
AppState
  |-- WAL append
  |-- Store execute
  |-- Metrics update
  |
  v
Response
  |
  v
wire text
```

Admin HTTP は別ポートで同じ `AppState` を読みます。

```text
GET /health  -> process health
GET /metrics -> command/error counters
GET /keys    -> current keys after lazy expiration
```

## Ownership Boundaries

```text
Command:
TCP から来た文字列を所有型へ変換したもの。

Store:
HashMap と Entry を所有する。呼び出し元の文字列寿命に依存しない。

Entry:
value と expires_at を所有する。

Response:
処理結果を所有する。TCP 書き込みとは分ける。

AppState:
Store、Metrics、WAL path をまとめる実行時状態。
```

`parse_command` は `&str` を借りるだけです。Command に変換した後は `String` を所有します。これにより、TCP buffer の寿命に依存しない設計になります。

## Failure Policy

```text
parse error:
Store を変更せず ERROR を返す。

WAL write error:
Store を変更せず ERROR を返す。

GET missing:
正常な NOT_FOUND。

expired key:
読み取り時に削除する。期限切れはエラーではない。

broken WAL:
起動時の restore を失敗させる。
```

WAL は Store 更新より先です。WAL 書き込み後に Store 更新前で落ちた場合、再起動時にその操作が再生される可能性があります。この教材では、SET/DEL は同じ操作が再適用されても最終状態を説明しやすい形にしています。

## Concurrency

共有状態は `Arc<Mutex<AppState>>` です。これは学習用に単純さを優先した設計です。

良い点:

```text
所有関係が分かりやすい
Store と metrics の整合性を保ちやすい
std-only で説明しやすい
```

限界:

```text
全操作が 1 つの lock に集まる
admin /keys が Store を掃除するため書き込み扱いになる
遅い WAL 書き込み中は他の操作が待つ
```

次の設計候補:

```text
Store 専用スレッドへ channel で Command を送る
WAL writer を別スレッドにする
Arc<RwLock<Store>> と別 Mutex<Metrics> に分割する
tokio + async Mutex へ移す
```

## Persistence

WAL に書く操作:

```text
SET
SETEX
DEL
```

WAL に書かない操作:

```text
GET
EXISTS
TTL
QUIT
```

読み取りは状態を観察するだけなので WAL へ書きません。ただし lazy expiration は読み取り時に内部状態を変える可能性があります。この教材では期限切れ削除を WAL に書かず、復旧後にも期限時刻から同じ結果になる設計として扱います。

## std-only Scope

この project が自分で持つ責任:

```text
TCP accept
line protocol
WAL format
simple HTTP response
metrics text format
thread spawning
lock management
```

実務で外部へ任せる候補:

```text
HTTP parser/server -> hyper, axum
async runtime -> tokio
CLI/config -> clap, config
serialization -> serde
logging -> tracing
error boilerplate -> thiserror, anyhow
metrics -> metrics, prometheus exporter
```

## Review Questions

```text
WAL 失敗時に Store を更新しない方針がコードに現れているか
Command と Response は TCP から独立しているか
Store は TcpStream を知らないか
Arc<Mutex<AppState>> の lock 範囲は説明できるか
admin HTTP の制限を README に明記しているか
```

## Connection To The Tutorial

この project は、各 Level の判断を次のように統合しています。

| Code area | Tutorial level | 判断 |
| --- | --- | --- |
| `Command` / `Response` | Level 5 | wire text を早めに型へ変換する |
| `ParseError` | Level 2, 6 | 入力エラーを状態結果から分ける |
| `Store` / `Entry` | Level 3, 8 | 状態と TTL を所有する境界を作る |
| WAL append / restore | Level 8 | 状態変更を操作ログとして保存する |
| `Arc<Mutex<AppState>>` | Level 7 | 共有状態を明示し、lock 範囲を説明する |
| admin HTTP | Level 8 | health、metrics、keys を運用入口にする |
| std-only scope | Level 9 | 自作する責任と crate に任せる責任を判断する |

コードレビューでは、行数や機能数よりも、この対応が崩れていないかを確認してください。

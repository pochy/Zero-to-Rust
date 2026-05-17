# 09: std-only から production ecosystem へ

## 目的

std-only で学ぶと、Rust の責任境界が見えます。しかし実務では、すべてを自作することが正解ではありません。この補講では、`final_kvs_server` を題材に、どの責任を外部 crate へ移すかを判断します。

## 移行の基本方針

```text
まず std-only で仕組みを理解する。
次に責任の重い部分を crate に任せる。
最後に、任せた責任と残った責任を文書化する。
```

外部 crate 採用は、学習の逃げではありません。信頼できる実装に責任を移し、自分のコードをドメイン設計へ集中させる判断です。

## TCP / HTTP

std-only:

```text
TcpListener
TcpStream
手書き HTTP response
```

移行候補:

```text
tokio:
async runtime と非同期 I/O。

hyper:
HTTP protocol の低レイヤー実装。

axum:
router、extractor、handler を使う web framework。
```

判断:

```text
学習用なら std-only。
HTTP 互換性、middleware、timeout、body limit が必要なら axum/hyper。
大量接続や他の async crate と組むなら tokio。
```

## Protocol / Serialization

std-only:

```text
SET key value
splitn
手書き Response
```

移行候補:

```text
serde
serde_json
postcard
bincode
```

判断:

```text
人間が telnet/nc で触る教材なら text protocol。
API contract や互換性が必要なら serde。
バイナリ効率が必要なら、format の versioning も含めて検討する。
```

## Error Handling

std-only:

```text
enum ParseError
impl Display
impl Error
io::Error
```

移行候補:

```text
thiserror:
library の分類可能なエラー。

anyhow:
binary 上位の文脈つきエラー。
```

判断:

```text
library 境界では thiserror。
main や CLI では anyhow。
分類不能な String エラーは避ける。
```

## Logging / Metrics

std-only:

```text
eprintln!
/metrics text
manual counters
```

移行候補:

```text
tracing
metrics
prometheus exporter
opentelemetry
```

判断:

```text
学習用なら eprintln と counters。
本番で検索、相関、span、集約が必要なら tracing。
監視基盤とつなぐなら metrics ecosystem。
```

## Persistence

std-only:

```text
WAL file
OpenOptions
line replay
```

移行候補:

```text
sled
redb
sqlite
postgres
```

判断:

```text
WAL の考え方を学ぶなら自作。
本番で durability、compaction、crash consistency が必要なら成熟した storage を検討する。
```

## CLI / Config

std-only:

```text
std::env::var
std::env::args
```

移行候補:

```text
clap
config
figment
dotenvy
```

判断:

```text
引数が少ない教材なら std。
help、subcommand、validation、env/file merge が必要なら clap/config。
```

## Migration Exercise

`final_kvs_server` を production ecosystem へ移す計画を書いてください。

```text
1. TCP command server を tokio に移すか。
2. admin HTTP を axum に移すか。
3. protocol を JSON にするか text のままにするか。
4. WAL を自作のまま残すか storage crate に移すか。
5. error type を thiserror にするか。
6. binary 上位を anyhow にするか。
7. logs を tracing にするか。
8. metrics を Prometheus 形式にするか。
```

各項目に、採用理由と採用しない場合のリスクを書いてください。

## 進級チェック

```text
std-only が学習に向く理由を説明できるか
std-only にこだわりすぎる危険を説明できるか
外部 crate に任せる責任と、自分のコードに残る責任を分けて説明できるか
```

## 次に読む

- 前へ: [appendices/08_professional_rust_map.md](08_professional_rust_map.md)
- 次へ: [docs/tracks/ADVANCED_TRACK.md](../docs/tracks/ADVANCED_TRACK.md)
- 関連: [appendices/README.md](README.md), [docs/guide/LEARNING_PATH.md](../docs/guide/LEARNING_PATH.md)

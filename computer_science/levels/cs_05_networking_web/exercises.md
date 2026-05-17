# CS 5 Exercises

## 1. request line parser

次の文字列を parse してください。

```text
GET /hello HTTP/1.1
POST /shorten HTTP/1.1
```

method、path、version に分けます。不正な形式は `Result` で返してください。

## 2. tiny TCP response

`TcpListener` を使い、接続されたら固定 response を返す program を作ってください。

```text
HTTP/1.1 200 OK
Content-Length: 2

OK
```

## 3. Cookie / Session 設計メモ

login 機能を想定して、Cookie に入れるもの、server 側に持つもの、DB に保存するものを分けてください。

## 提出物

```text
request_line_parser.rs
tiny_http_response.rs
session_design.md
```

## 進級チェック

```text
TCP connection が切れた場合と HTTP 404 を区別できるか
Cookie と Session を同じものとして扱っていないか
retry してよい request と危険な request を区別できるか
```

## 次に読む

- 前へ: [computer_science/levels/cs_05_networking_web/README.md](README.md)
- 次へ: [computer_science/levels/cs_06_databases/README.md](../cs_06_databases/README.md)
- 関連: [computer_science/CHECKPOINTS.md](../../CHECKPOINTS.md), [computer_science/SOLUTIONS.md](../../SOLUTIONS.md), [computer_science/glossary.md](../../glossary.md)

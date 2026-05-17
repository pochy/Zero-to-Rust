# CS 5: Networking And Web

## この Level でできるようになること

HTTP、TCP/IP、DNS、TLS、Cookie、Session を、Web application の障害や設計と結びつけて説明できるようになります。

## まず知るべき言葉

```text
IP
TCP
UDP
port
DNS
HTTP
TLS
Cookie
Session
latency
timeout
retry
```

## なぜこれを学ぶのか

Web application は、framework だけで動いているわけではありません。

```text
browser が DNS で IP を引く
TCP connection を張る
TLS handshake を行う
HTTP request を送る
server が DB や cache と通信する
response を返す
```

どこかが遅い、失敗する、詰まるだけで user には「サイトが遅い」と見えます。

## 手順 1: HTTP と TCP を分ける

```text
TCP: reliable byte stream
HTTP: request / response の application protocol
```

HTTP は message の意味を扱います。TCP は byte stream を届けます。HTTP error と TCP connection error は別の問題です。

request line を parse します。

```bash
rustc --edition=2021 computer_science/levels/cs_05_networking_web/examples/request_line_parser.rs -o /tmp/cs_request_line_parser
/tmp/cs_request_line_parser
```

見るべき点:

```text
GET /hello HTTP/1.1 は method、path、version に分かれる
不正な request line は Result の Err になる
HTTP message の parse error と TCP connection error は別である
```

最小 HTTP response も動かします。

```bash
rustc --edition=2021 computer_science/levels/cs_05_networking_web/examples/tiny_http_response.rs -o /tmp/cs_tiny_http_response
/tmp/cs_tiny_http_response
```

別 terminal から:

```bash
curl -i http://127.0.0.1:7878/hello
```

見るべき点:

```text
TcpListener は connection を受け取る
HTTP response は TCP stream へ byte として書く
Content-Length は body の byte length と一致させる必要がある
```

## 手順 2: DNS と TLS を障害要因として見る

DNS は domain name を IP address に解決します。TLS は暗号化と相手確認を行います。

```text
DNS が遅い
証明書が期限切れ
TLS handshake が失敗
intermediate certificate が欠ける
```

これらは application code を変更していなくても起こります。

retry 方針も設計します。

```bash
rustc --edition=2021 computer_science/levels/cs_05_networking_web/examples/retry_policy.rs -o /tmp/cs_retry_policy
/tmp/cs_retry_policy
```

見るべき点:

```text
GET 系の read は retry しやすい
order 作成や card 決済は重複実行が危険
timeout は成功したが response だけ失われた可能性がある
```

## 手順 3: Cookie と Session を分ける

```text
Cookie: browser 側に保存され request に付く情報
Session: server 側で user state を管理する仕組み
```

Cookie に何を入れるか、server 側で何を持つかは security と scalability に直結します。

Cookie header を parse します。

```bash
rustc --edition=2021 computer_science/levels/cs_05_networking_web/examples/cookie_parser.rs -o /tmp/cs_cookie_parser
/tmp/cs_cookie_parser
```

見るべき点:

```text
Cookie は key=value の集合として送られる
session_id は user の認証状態そのものではなく、server side session を引く key として扱う
Cookie に機密情報をそのまま入れない
```

## Rust で作るもの

小さな HTTP 風 protocol を作ります。

```text
GET /shorten?url=...
GET /r/{code}
```

最初は本物の HTTP framework ではなく、request line を parse するだけで構いません。protocol の構造を理解してから `axum` などへ進む方が、責任境界が見えます。

## TypeScript / Go ならどう見えるか

TypeScript/Next.js では HTTP や Cookie が framework に包まれます。Go は `net/http` で protocol を比較的素直に扱えます。Rust では最初に std の `TcpListener` を使うと、connection、read、write、error が見えます。

## よくあるつまずき

```text
HTTP と TCP を混同する
DNS を application の外側だと思って無視する
timeout と retry の方針を決めない
Cookie に入れてよい情報を考えない
Session store の failure を考えない
```

## 次の Level に進む条件

```text
HTTP と TCP の層の違いを説明できる
DNS と TLS が request 前に関与することを説明できる
Cookie と Session の責任分担を説明できる
timeout と retry の危険を説明できる
```

## 公式 docs で確認する箇所

```text
std::net::TcpListener
std::net::TcpStream
std::io::Read
std::io::Write
```

## 次に読む

- 前へ: [computer_science/levels/cs_04_os_cli_io/exercises.md](../cs_04_os_cli_io/exercises.md)
- 次へ: [computer_science/levels/cs_05_networking_web/exercises.md](exercises.md)
- 関連: [computer_science/CHECKPOINTS.md](../../CHECKPOINTS.md), [computer_science/SOLUTIONS.md](../../SOLUTIONS.md), [computer_science/glossary.md](../../glossary.md)

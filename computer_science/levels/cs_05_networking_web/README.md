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

## 手順 2: DNS と TLS を障害要因として見る

DNS は domain name を IP address に解決します。TLS は暗号化と相手確認を行います。

```text
DNS が遅い
証明書が期限切れ
TLS handshake が失敗
intermediate certificate が欠ける
```

これらは application code を変更していなくても起こります。

## 手順 3: Cookie と Session を分ける

```text
Cookie: browser 側に保存され request に付く情報
Session: server 側で user state を管理する仕組み
```

Cookie に何を入れるか、server 側で何を持つかは security と scalability に直結します。

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


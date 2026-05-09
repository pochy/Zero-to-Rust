# Level 5: TCP KVS の最小アプリケーションワークフロー

## この Level でできるようになること

TCP KVS の中心にある、入力、パース、実行、レスポンス生成の流れを設計できるようになります。

この Level では、いきなり本格的なサーバーを完成させることより、ネットワーク層とコアロジックを分けることを重視します。

## まず知るべき言葉

- TCP: 接続を張ってバイト列を送受信する通信方式。
- `TcpListener`: 接続を待ち受ける標準ライブラリの型。
- `TcpStream`: 接続済みの TCP ストリーム。
- プロトコル: リクエストとレスポンスの約束。
- コマンド: `SET key value` のような操作要求。
- レスポンス: `OK`、`VALUE value`、`NOT_FOUND` のような結果。
- ワークフロー: 入力から出力までの一連の流れ。

## なぜこれを学ぶのか

ネットワークアプリで初心者がやりがちな失敗は、接続処理の中にすべてを書くことです。

```text
TCP から 1 行読む
split する
HashMap を直接操作する
println 的にレスポンスを組み立てる
エラーもその場で文字列化する
```

この設計は最初は短く見えますが、テスト、並行化、永続化、HTTP 管理画面の追加で崩れます。

この Level では、次の流れを分けます。

```text
wire text
↓
Command
↓
Store operation
↓
Response
↓
wire text
```

## 手順 1: プロトコルの流れを実行する

```bash
rustc --edition=2021 levels/level_05_application_workflow/examples/tcp_kvs_workflow.rs -o /tmp/zero_to_rust_workflow
/tmp/zero_to_rust_workflow
```

期待する出力:

```text
> SET name Rust
< OK
> GET name
< VALUE Rust
> GET missing
< NOT_FOUND
> DEL name
< OK
> GET name
< NOT_FOUND
```

ここでは実際の TCP 接続を使わず、TCP から届いたと仮定した文字列を処理しています。先にコアのワークフローを安定させるためです。

## 手順 2: コマンドとレスポンスを型で読む

例では `Command` と `Response` を `enum` で表します。

```rust
enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

enum Response {
    Ok,
    Value(String),
    NotFound,
    Error(String),
}
```

文字列のまま処理を続けると、分岐が増えるほど壊れやすくなります。パース時に型へ変換すると、後続処理は「正しい形のコマンド」を扱えます。

## 手順 3: TCP 層を後から接続する

実際のサーバーでは、外側に次の層を置きます。

```text
TcpStream から 1 行読む
parse_command へ渡す
store.execute を呼ぶ
response.to_wire で文字列化する
TcpStream へ書く
```

重要なのは、`Store` が TCP を知らないことです。`Store` は `Command` を受け取って `Response` を返すだけにします。

## よくあるつまずき

```text
Q. TCP サーバーから先に作ってはいけませんか？
A. 作れます。ただし、プロトコルと store が未分離のままサーバー化すると、テストしづらくなります。
```

```text
Q. レスポンスは String だけでよくないですか？
A. 最初は動きますが、後で HTTP やテストを追加すると Response enum の方が扱いやすくなります。
```

```text
Q. value に空白を含めたい場合はどうしますか？
A. `splitn(3, ' ')` のように分割数を制限します。プロトコル仕様として明記することが重要です。
```

## 次の Level に進む条件

```text
wire text、Command、Response の違いを説明できる
Store が TcpStream を知らない設計の利点を説明できる
不正コマンドを panic ではなく ERROR にする理由を説明できる
```

## 公式 docs で確認する箇所

- std::net: https://doc.rust-lang.org/std/net/
- std::io::BufRead: https://doc.rust-lang.org/std/io/trait.BufRead.html
- enum: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

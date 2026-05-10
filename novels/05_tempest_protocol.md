# 05: テンペスト通信網と独自プロトコル

対応教材: [Level 5: アプリケーション編](../levels/level_05_application_workflow/README.md)

## 物語パート

村が大きくなると、声だけでは届かなくなった。

倉庫、鍛冶場、見張り台、薬草畑。各所から「木材が足りない」「薬草を送ってほしい」「牙狼族が通った」と連絡が飛ぶ。

伝令だけでは遅い。

「通信網を作ろう」

『告。ネットワークアプリケーションの段階に入りました』

「村なのに急にネットワーク」

『解。離れた相手とコマンドをやり取りするなら、通信層とプロトコル層が必要です』

俺は中央広場に、魔素で小さな通信塔を作った。

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(stream.try_clone()?);

        let mut line = String::new();
        reader.read_line(&mut line)?;

        stream.write_all(b"OK\n")?;
    }

    Ok(())
}
```

「これで、来た通信に `OK` を返す」

リグルドが感心したように頷く。

「おお、まるで王の許可印ですな」

「ただし、なんでも `OK` って返す王は危ない」

『肯定。次にプロトコルを定義します』

```text
SET key value
GET key
DEL key
EXISTS key
QUIT
```

「命令を決める。返事も決める」

```text
OK
VALUE value
NOT_FOUND
ERROR message
BYE
```

これで、テンペストの各拠点は同じ言葉で台帳に触れられる。

## 会話・独白パート

ゴブタが通信石に向かって叫んだ。

「SET meat lunch!」

倉庫番が返す。

「OK」

「GET meat!」

「VALUE lunch」

「おお、返ってきたっす！」

リグルドが眉をひそめる。

「しかし、命令が間違っていたら？」

「`ERROR message` を返す」

『解。プロトコルは成功時だけでなく、失敗時の応答も決める必要があります』

「通信層、プロトコル層、Store 層を分けるんだよな」

『肯定。TCP 接続を受ける処理の中で、文字列分割、保存処理、レスポンス生成をすべて混ぜると保守不能になります』

「王宮で料理も鍛冶も裁判も全部やるみたいなものか」

リグルドが真顔で言う。

「それは国が滅びますな」

「コードも同じだ」

大賢者が静かに告げた。

『提案。`handle_client() -> parse_command() -> store.execute(command) -> response.to_wire()` の流れを意識してください』

## 大賢者による解説

『告。Level 5 の目的は、ネットワーク層とロジック層を分けることです』

`TcpListener` は TCP 接続を待ち受ける型です。`incoming()` で接続を順に受け取ります。

`TcpStream` は接続済みの相手との読み書きを表します。`BufReader` は読み込みを効率化し、行単位の読み取りに使います。

『解。ネットワークアプリでは、最低でも 3 層に分けます』

```text
TCP 入出力層
プロトコル解析層
KVS 操作層
```

この分離により、TCP を使わないテストでも `parse_command` や `Store` を検証できます。

悪い設計:

```text
接続処理の中で split し、HashMap を直接操作し、レスポンス文字列も作る。
```

良い設計:

```text
入力文字列を Command に変換する。
Command を Store に渡す。
結果を Response に変換する。
Response を wire format に変換する。
```

## Rust 任務

読むもの:

- [tcp_kvs_workflow.rs](../levels/level_05_application_workflow/examples/tcp_kvs_workflow.rs)
- [Level 5 exercises](../levels/level_05_application_workflow/exercises.md)

考えること:

```text
SET key value はどの enum variant に変換するか。
GET key の結果がないとき、Store は何を返すか。
Response はどこで文字列に変換するか。
接続が途中で切れたとき、panic せず処理できるか。
```

## 初出用語・関数の説明

- `std::net::TcpListener`: TCP 接続を待ち受ける標準ライブラリの型。
- `TcpStream`: TCP 接続された相手との通信路を表す型。
- `bind`: 指定したアドレスとポートで待ち受けを開始する関数。
- `incoming()`: 入ってくる接続を順に返すメソッド。
- `std::io::BufReader`: 読み込みをバッファリングする型。
- `BufRead`: 行単位読み取りなどを提供する trait。
- `read_line`: 1 行を読み込むメソッド。
- `Write`: 書き込み機能を表す trait。
- `write_all`: 指定したバイト列をすべて書き込むメソッド。
- `try_clone`: `TcpStream` の別ハンドルを作るメソッド。
- プロトコル: 通信で使う命令と応答の決まり。
- wire format: 通信路に流す実際の文字列やバイト列の形式。

## 進級チェック

次の問いに答えられたら、次章へ進む。

```text
TcpListener と TcpStream の役割を説明できるか。
プロトコル層と Store 層を分ける理由を説明できるか。
不正コマンドで panic しない設計を説明できるか。
レスポンス末尾の改行が必要な理由を説明できるか。
```

夜、通信塔に初めて複数の光が灯った。

「国っぽくなってきたな」

『告。肯定。ただし、動いたことと正しいことは別です』

「次はそこか」

『解。テストです』

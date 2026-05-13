# kvs_ecosystem

主要クレートを採用する実務寄り KVS 演習です。

`kvs_std` と同じ題材を、`serde`、`clap`、`thiserror`、`anyhow`、`tracing`、`tokio` で書き直します。目的は「クレートを覚える」ことではなく、どの責任を標準ライブラリで持ち、どの責任を成熟した crate に任せるかを判断することです。

## 実行

```bash
cargo run -p kvs_ecosystem -- --request '{"op":"set","key":"name","value":"Rust"}'
cargo run -p kvs_ecosystem -- --request '{"op":"get","key":"missing"}'
```

このサンプルは 1 起動で 1 リクエストを処理する CLI です。起動間で状態は保持しません。Tokio は async runtime 採用の形を示すために使っています。

## テスト

```bash
cargo test -p kvs_ecosystem
```

## 採用した crate と責任

| crate | 任せる責任 |
| --- | --- |
| `serde` / `serde_json` | JSON wire format の parse / serialize |
| `clap` | CLI 引数と help |
| `thiserror` | 分類可能な library error |
| `anyhow` | binary の上位エラー文脈 |
| `tracing` | 構造化ログ |
| `tokio` | async runtime |

## std-only と比べる

std-only 版では、wire text、エラー文字列、ログ出力を自分で持ちます。この版では、それらの一部を crate に任せます。実務ではこの判断が重要です。自作すると学べますが、保守責任も背負います。

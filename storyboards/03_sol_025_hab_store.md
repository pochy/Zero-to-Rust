# Sol 025: HAB 在庫台帳

対応教材: [Level 3: データ設計と構造設計](../levels/level_03_design/README.md)

## 状況

二十五日目。

マークは食料、工具、ケーブル、ヒドラジン、ジャガイモ、予備フィルタを数え直していた。問題は、紙のメモが増えすぎたことだ。

「在庫台帳が必要だ。火星版 KVS だ」

キーは `potato:raw`、値は `32`。キーは `filter:co2`、値は `ok`。単純だ。単純なものほど、設計を間違えると長く苦しむ。

彼は `HashMap<String, String>` を使うことにした。

```bash
rustc --edition=2021 levels/level_03_design/examples/kvs_store.rs -o /tmp/zero_to_rust_kvs
/tmp/zero_to_rust_kvs
```

期待する出力:

```text
name = Rust
exists lang = false
deleted name = true
exists name = false
```

## ログ

**LOG 025.1**

マーク:

「`Store` を作る。名前がいい。火星で必要なものはだいたい Store だ。酸素 Store、水 Store、食料 Store、そして精神安定のための冗談 Store」

コードの中心:

```rust
pub struct Store {
    data: HashMap<String, String>,
}
```

マーク:

「`Store` がキーと値を所有する。これが重要だ。外から借りた `&str` を中に保存すると、元の文字列が先に消えたときに詰む。火星では『先に消えた』はだいたい致命的だ」

彼は悪い設計をメモに書いた。

```rust
struct Store<'a> {
    data: HashMap<&'a str, &'a str>,
}
```

マーク:

「これは上級者が目的を持ってやるならいい。だが、今の俺がやると、在庫台帳が誰かの付箋に依存する。付箋が飛んだら在庫が消える。火星ではやめよう」

## 会話

管制:

「なぜ `set` は `String` を受け取る？」

マーク:

「保存するからだ。`Store` が所有する。ジャガイモを倉庫に入れるなら、倉庫が責任を持つ」

管制:

「なぜ `get` は `&str` を受け取る？」

マーク:

「探すだけだからだ。探すたびにキーを所有する必要はない。倉庫の棚番号を読み上げるだけで、棚を持ち帰る必要はない」

管制:

「戻り値が `Option<&String>` なのは？」

マーク:

「存在しないかもしれない。そして存在しても、値の所有者は `Store` のままだ。見せてもらっているだけだ」

## Rust 任務

1. `kvs_store.rs` を実行する。
2. `set`、`get`、`delete` の引数と戻り値を読む。
3. `exists(&self, key: &str) -> bool` を追加する。
4. `Option<&String>` と `Option<String>` の違いを、在庫台帳の責任として説明する。

演習:

- [Level 3 exercises](../levels/level_03_design/exercises.md)

## マークの独り言

「`main.rs` に全部書きたくなる。分かる。HAB の床に工具を全部広げると、最初の三十分は効率がいい」

「だが、酸素生成機の部品と、食料計算と、通信アンテナのネジが同じ箱に入ったら終わりだ」

```text
main.rs:
起動と入出力。

store.rs:
データ操作。

command.rs:
コマンド解析。

error.rs:
失敗の表現。
```

マーク:

「責務分離はきれいに見せるためではない。三週間後の俺が修理中に泣かないためだ」

## 進級チェック

次の問いに答えられたら、Sol 041 へ進む。

```text
`Store` が `String` を所有する理由を説明できるか。
`get(&self, key: &str)` が借用でよい理由を説明できるか。
`Option<&String>` と `Option<String>` の違いを説明できるか。
`main.rs` からコアロジックを分ける理由を説明できるか。
```

マークのメモ:

「在庫が分かった。食料は足りない。だが、少なくともデータの所有者は分かった」


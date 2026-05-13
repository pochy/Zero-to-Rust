# Level 9 Exercises

## 1. 小さく変更する

`packet_reader.rs` に `read_u32_be` を追加してください。

短い入力を渡したときに `None` が返ることも確認します。

## 2. 出力を比較する

同じ 2 バイトを big endian と little endian として読んだ場合の値を比較してください。

```text
[0x12, 0x34]
big endian    -> 0x1234
little endian -> 0x3412
```

## 3. 設計判断を書く

最終課題の KVS サーバーについて、次の判断を書いてください。

```text
どの型が key と value を所有するか
どの関数は &str を受け取るか
どの状態を Arc<Mutex<T>> で共有するか
WAL 書き込み失敗時にどうするか
std-only では不足する本番要件は何か
```

## 提出物

```text
1. read_u32_be の実装
2. エンディアン比較の説明
3. 最終課題の設計メモ
```

## 進級チェック

```text
低レイヤー処理で境界チェックが必要な理由を説明できるか？
最終課題の所有、失敗、共有、復旧、運用を自分の設計として説明できるか？
```

## 追加演習: std-only と ecosystem を比較する

`projects/kvs_std` と `projects/kvs_ecosystem` を読み、次を比較してください。

```text
serde に任せた責任
clap に任せた責任
thiserror と anyhow の役割の違い
tracing を使うと println と何が変わるか
tokio を使う理由が今回の小さい例で本当にあるか
```

採用した crate と採用しなかった crate の両方に理由を書いてください。

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
どの状態を共有ロックで守るか
WAL 書き込み失敗時にどうするか
std-only では不足する本番要件は何か
```

Rust で共有ロックを使う場合、候補の 1 つが `Arc<Mutex<T>>` です。Level 9 では、まず「どの状態を共有する必要があるか」を先に決めます。

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

## 学習記録

[CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md) の Level 9 と Projects Checkpoint を A/B/C で自己評価してください。

[STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md) には、次を書きます。

```text
unsafe を使わない理由、または使う場合の safety 条件
std-only で自分が持つ責任
外部 crate に任せる責任
```

`unsafe` は発展項目です。使わない設計を選んだ場合も、十分な設計判断として扱います。

## 追加演習: std-only と ecosystem を比較する

`projects/kvs_std` と `projects/kvs_ecosystem` を読み、次を比較してください。

この追加演習は発展です。以下の crate 名は、実務でよく使う外部ライブラリです。今すぐ暗記せず、「標準ライブラリだけで自分が持つ責任」と「crate に任せる責任」を比較するために使います。

```text
serde: JSON などへの変換を任せる
clap: CLI 引数の解釈を任せる
thiserror / anyhow: エラー表現や文脈付けを任せる
tracing: 構造化ログを任せる
tokio: 非同期 I/O とタスク実行を任せる
```

```text
serde に任せた責任
clap に任せた責任
thiserror と anyhow の役割の違い
tracing を使うと println と何が変わるか
tokio を使う理由が今回の小さい例で本当にあるか
```

採用した crate と採用しなかった crate の両方に理由を書いてください。

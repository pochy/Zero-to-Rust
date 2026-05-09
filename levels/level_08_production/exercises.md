# Level 8 Exercises

## 1. 小さく変更する

`wal_restore.rs` に `DEL old` だけでなく、存在するキーを削除する操作を追加してください。

復元後にそのキーが存在しないことを確認します。

## 2. 出力を比較する

WAL の操作順を変えて、最終状態がどう変わるか比較してください。

```text
SET name Rust
DEL name
SET name Ferris
```

と

```text
SET name Rust
SET name Ferris
DEL name
```

を比較します。

## 3. 設計判断を書く

WAL 書き込みに失敗したとき、次のどちらを選ぶか考えてください。

```text
Store を更新せず ERROR を返す
Store は更新し WARNING を出す
```

復旧性、可用性、データ整合性の観点で判断します。

## 提出物

```text
1. 削除操作を追加した復元結果
2. 操作順を変えた最終状態の比較
3. WAL 書き込み失敗時の設計判断
```

## 進級チェック

```text
操作ログの順序が状態を決める理由を説明できるか？
復旧性と可用性のトレードオフを説明できるか？
```

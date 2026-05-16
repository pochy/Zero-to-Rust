# Level 3 Exercises

## 1. 小さく変更する

`kvs_store.rs` に `exists(&self, key: &str) -> bool` を追加してください。

`get(key).is_some()` を使う実装と、`HashMap::contains_key` を使う実装を比較してください。

## 2. 出力を比較する

同じキーに 2 回 `set` した場合、古い値がどうなるか確認してください。

```rust
store.set("name".to_string(), "Rust".to_string());
store.set("name".to_string(), "Ferris".to_string());
```

## 3. 設計判断を書く

`get` の戻り値を次のどちらにするか、用途ごとに判断を書いてください。

```rust
Option<&String>
Option<String>
```

`Option` は「値があるか、ないか」を表します。`&String` は Store の中の値を読むだけ、`String` は値を複製して渡す、と考えてください。

## 提出物

```text
1. exists の実装
2. 同じキーを上書きした実行結果
3. Option<&String> と Option<String> の使い分け
```

## 進級チェック

```text
構造体がデータを所有することの意味を説明できるか？
API の引数と戻り値から、所有と借用の境界を読めるか？
```

## 学習記録

[CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md) の Level 3 を A/B/C で自己評価してください。

[STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md) には、次を書きます。

```text
Store が所有する値
呼び出し元から借りるだけでよい値
trait を導入する理由、またはまだ導入しない理由
```

## 追加演習: trait 導入の是非

`Store` trait を作るべきか判断してください。

この追加演習は発展です。`trait` は「複数の実装を同じ約束で扱う」ための道具で、Level 3 の必須理解ではありません。まだ難しければ、補講を読んだ後に戻ってください。

```rust
trait Store {
    fn set(&mut self, key: String, value: String);
    fn get(&self, key: &str) -> Option<&str>;
}
```

今すぐ導入する場合、導入しない場合、将来 WAL や remote store が出た場合で判断を書いてください。

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

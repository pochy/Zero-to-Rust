# 02: trait、generics、抽象化

## 目的

Rust の抽象化は、継承よりも trait と generics を中心に組み立てます。重要なのは「抽象化できるか」ではなく、「どの変更軸を抽象化するか」です。

## generics は型を後で決める仕組み

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}
```

この関数は `Vec<String>` にも `Vec<i32>` にも使えます。ただし、`T` にどんな操作ができるかは制限されています。比較したいなら境界が必要です。

```rust
fn contains<T: PartialEq>(items: &[T], needle: &T) -> bool {
    items.iter().any(|item| item == needle)
}
```

## trait は振る舞いの約束

```rust
trait Store {
    fn set(&mut self, key: String, value: String);
    fn get(&self, key: &str) -> Option<&str>;
}
```

trait を入れる理由は「かっこいいから」ではありません。メモリ実装、WAL 実装、リモート実装など、複数の実装を同じ契約で扱いたいときに意味があります。

## `impl Trait` と generics

```rust
fn print_all(items: impl IntoIterator<Item = String>) {
    for item in items {
        println!("{}", item);
    }
}
```

`impl Trait` は引数や戻り値を簡潔に書けます。公開 API では、読みやすさと将来の互換性を考えて選びます。

## associated type

trait の中で「実装ごとに決まる型」を表すときに使います。

```rust
trait Parser {
    type Output;
    type Error;

    fn parse(&self, input: &str) -> Result<Self::Output, Self::Error>;
}
```

`Parser<Output = Command>` のように、実装側が出力型を決められます。Iterator の `Item` も associated type です。

## `dyn Trait` は実行時ディスパッチ

```rust
fn run_store(store: &mut dyn Store) {
    store.set("name".to_string(), "Rust".to_string());
}
```

`dyn Trait` は異なる具体型を同じ箱で扱えます。その代わり、実行時ディスパッチや object safety の制約があります。性能よりも、境界を安定させたいときに選びます。

## 早すぎる trait の危険

悪い導入:

```text
まだ実装が 1 つしかない
何が変わるか分かっていない
テストのためだけに全体を trait 化する
```

良い導入:

```text
保存先をメモリと WAL で差し替える
時計やファイルシステムをテストで差し替える
同期版と非同期版の境界を分ける
外部依存を内側へ漏らしたくない
```

## 進級チェック

```text
generics と trait の違いを説明できるか
trait を導入する変更軸を説明できるか
impl Trait と dyn Trait の使い分けを説明できるか
associated type が必要な場面を説明できるか
```

## 公式 docs

- https://doc.rust-lang.org/book/ch10-00-generics.html
- https://doc.rust-lang.org/book/ch10-02-traits.html
- https://doc.rust-lang.org/book/ch17-02-trait-objects.html

## 次に読む

- 前へ: [appendices/01_ownership_lifetimes.md](01_ownership_lifetimes.md)
- 次へ: [appendices/03_iterators_patterns_macros.md](03_iterators_patterns_macros.md)
- 関連: [appendices/README.md](README.md), [docs/guide/LEARNING_PATH.md](../docs/guide/LEARNING_PATH.md)

# Level 3: データ設計と構造設計

## この Level でできるようになること

`struct` と `HashMap` を使い、インメモリ KVS のコアを設計できるようになります。

この Level の中心は、データをどこに所有させるかを決めることです。

## まず知るべき言葉

- KVS: key-value store。キーから値を取得する保存方式。
- `HashMap<K, V>`: キーと値を対応づける標準コレクション。
- `struct`: 意味のある状態をまとめる型。
- `impl`: 型に関数を結びつける場所。
- `&self`: メソッドが自分の中身を読むだけの指定。
- `&mut self`: メソッドが自分の中身を変更する指定。
- `Option<T>`: 値があるかないかを表す型。
- API 境界: 呼び出し側と実装側の責任が切り替わる場所。

この Level では、構造体に参照を保存する高度な設計は扱いません。まず `Store` が必要なデータを自分で所有する、という単純な形から始めます。`Option` は「存在しないキーがある」ことを表すために使います。

## なぜこれを学ぶのか

Rust では、構造体が何を所有するかが設計そのものになります。

```rust
pub struct Store {
    data: HashMap<String, String>,
}
```

この `Store` はキーと値を所有します。呼び出し側から借りた `&str` を内部に持つのではなく、必要な値を `String` として保存します。

初心者はコピーを避けようとして、いきなり参照を構造体に持たせがちです。

```rust
struct Store<'a> {
    data: HashMap<&'a str, &'a str>,
}
```

この設計は、保存された値が元の文字列の寿命に依存します。`'a` はライフタイム注釈という高度な記法です。学習初期の KVS では、まず `Store` が所有する設計にします。

## 手順 1: KVS の最小操作を実行する

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

見るべき点は、`set` は `String` を受け取り、`get` と `delete` は `&str` を受け取ることです。

## 手順 2: API の所有権を読む

`Store::set` は次の形です。

```rust
pub fn set(&mut self, key: String, value: String)
```

`&mut self` は、このメソッドが `Store` の中身を変更するという意味です。保存する値は `Store` が所有するため、`String` を受け取ります。

`Store::get` は次の形です。

```rust
pub fn get(&self, key: &str) -> Option<&String>
```

`&self` は、このメソッドが `Store` の中身を読むだけという意味です。検索キーは読むだけなので `&str` で十分です。戻り値は `Store` 内の値への参照なので、呼び出し側は値を所有しません。

## 手順 3: モジュール分割を考える

この Level の例は 1 ファイルですが、実際のアプリでは次のように分けます。

```text
main.rs      起動と入出力
store.rs     データ操作
command.rs   コマンド解析
error.rs     エラー定義
```

分割の目的は見た目を整えることではありません。テストしやすくし、変更の影響範囲を狭くすることです。

## よくあるつまずき

```text
Q. すべて String を clone すれば楽では？
A. 楽に見えますが、所有が曖昧になります。保存するための clone なのか、エラー回避の clone なのかを分けて考えます。
```

```text
Q. get は Option<String> を返すべきですか？
A. 値を複製して渡したいなら Option<String> です。読むだけなら Option<&String> で十分です。
```

```text
Q. Option<&String> の & が難しく見えます。
A. 今は「値そのものを渡さず、Store の中にある値を読むだけ」と考えれば十分です。参照の詳しい規則は補講で深掘りします。
```

```text
Q. HashMap を public にしてよいですか？
A. 直接公開すると不変条件を守りにくくなります。まず Store のメソッド経由で操作させます。
```

## 次の Level に進む条件

```text
Store が String を所有する理由を説明できる
set と get で引数の型が違う理由を説明できる
Option が存在しないキーを表す方法だと説明できる
main と store を分ける理由を説明できる
```

## 公式 docs で確認する箇所

- Defining and Instantiating Structs: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
- std::collections: https://doc.rust-lang.org/std/collections/

## Rust らしさをさらに深掘りする

Rust の `struct` は、単なるデータの箱ではなく所有境界です。`Store` が `HashMap<String, String>` を所有することで、呼び出し元の文字列寿命から独立します。

追加で読む箇所:

- [所有権、借用、ライフタイム完全補講](../../appendices/01_ownership_lifetimes.md)
- [trait、generics、抽象化](../../appendices/02_traits_generics.md)

次の問いを追加で考えてください。

```text
Store に &str を保存すると、誰の寿命に依存するか
get が Option<&String> を返すと、呼び出し側は何を所有しないか
HashMap を public にしたとき、どの不変条件が壊れやすいか
```

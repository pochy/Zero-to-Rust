# Level 3: データ設計と構造設計

## この Level でできるようになること

`struct`、`enum`、`HashMap` を使い、インメモリ KVS のコアを設計できるようになります。

この Level の中心は、データをどこに所有させるかを決めることです。

## まず知るべき言葉

- KVS: key-value store。キーから値を取得する保存方式。
- `HashMap<K, V>`: キーと値を対応づける標準コレクション。
- `struct`: 意味のある状態をまとめる型。
- `impl`: 型に関数を結びつける場所。
- `Option<T>`: 値があるかないかを表す型。
- API 境界: 呼び出し側と実装側の責任が切り替わる場所。

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

この設計は、保存された値が元の文字列の寿命に依存します。学習初期の KVS では、まず `Store` が所有する設計にします。

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

保存する値は `Store` が所有するため、`String` を受け取ります。

`Store::get` は次の形です。

```rust
pub fn get(&self, key: &str) -> Option<&String>
```

検索キーは読むだけなので `&str` で十分です。戻り値は `Store` 内の値への参照なので、呼び出し側は値を所有しません。

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
- Enums and Pattern Matching: https://doc.rust-lang.org/book/ch06-00-enums.html
- std::collections: https://doc.rust-lang.org/std/collections/

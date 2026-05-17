# Level 1: Rust に触る

## この Level でできるようになること

Rust の最小プログラムを実行し、`String` と `&str` の違いを観察できるようになります。

ここでは所有権を完全に理解する必要はありません。まず、値を「渡す」と「借りる」は違う、という感覚を作ります。

## まず知るべき言葉

- `rustc`: Rust ソースをコンパイルするコマンド。
- `cargo`: Rust のビルド、実行、テスト、依存管理を行う標準ツール。
- `main`: 実行プログラムの入口。
- `let`: 変数束縛を作る構文。
- `mut`: 変更可能な変数束縛にする指定。
- `String`: 文字列データを所有する型。
- `&str`: 文字列を借りて見る型。

この Level では、ライフタイム注釈や詳しいメモリ構造を覚える必要はありません。まずは `String` は「持つ」、`&str` は「借りて読む」、`mut` は「同じ変数名で中身を変えてよい印」と考えてください。分からない語が出たら [glossary.md](../../docs/reference/glossary.md) を確認します。

## なぜこれを学ぶのか

Rust では、関数の引数を見るだけで「この関数は値を所有するのか、借りるだけなのか」が分かります。

```rust
fn take_name(name: String) {}
fn borrow_name(name: &str) {}
```

`String` を受け取る関数は、呼び出し元から所有権を受け取ります。`&str` を受け取る関数は、呼び出し元の文字列を一時的に借ります。

後の学習では `clone()` という「値を複製する操作」も出てきます。ただし、最初はコンパイルエラーを複製で消す前に「この関数は本当に所有すべきか」を考えます。

## 手順 1: 最小プログラムを実行する

```bash
rustc --edition=2021 levels/level_01_intro/examples/hello_ownership.rs -o /tmp/zero_to_rust_hello
/tmp/zero_to_rust_hello
```

期待する出力:

```text
borrowed: Rust
owned: Rust
```

`borrowed` の行は、`&name` を渡して表示しています。`owned` の行は、`name` の所有権を渡して表示しています。

## 手順 2: コメントを外してエラーを読む

`hello_ownership.rs` の最後にあるコメントを読んでください。

```rust
// println!("{}", name);
```

このコメントを外すと、所有権を渡した後の `name` を使おうとしているためコンパイルエラーになります。

エラーを見たら、すぐ修正しないでください。まず次を確認します。

```text
どの値が移動したか
どの関数に渡したか
その関数は本当に所有する必要があるか
```

## 手順 3: 借用に変えて設計を観察する

`take_name(name)` を `borrow_name(&name)` に変えると、`name` はその後も使えます。

これは単なるテクニックではありません。関数の責任が変わったということです。

```text
String を受け取る:
この関数が値を最後まで扱う。

&str を受け取る:
この関数は値を読むだけで、所有者は呼び出し元のまま。
```

## 手順 4: `mut` は変更可能性の指定だと確認する

`mut` は所有権を戻す魔法ではありません。同じ変数の中身を後で変えてよい、という指定です。

```rust
let mut label = String::from("Rust");
label.push_str(" tutorial");
println!("{}", label);
```

一方で、次の 2 つは別の話です。

```text
mut:
同じ変数の中身を変えてよいか。

所有権:
値をどの関数や変数が最後まで管理するか。
```

## よくあるつまずき

```text
Q. なぜ代入しただけで元の変数が使えなくなるのですか？
A. String はヒープ上のデータを所有するため、単純コピーではなく所有権の移動になります。
```

```text
Q. いつ &str を使えばよいですか？
A. 関数が文字列を読むだけなら、まず &str を検討します。所有する必要があるなら String を受け取ります。
```

```text
Q. mut を付ければ所有権の問題は解決しますか？
A. 解決しません。mut は変更可能性の指定であり、所有権の移動とは別の概念です。
```

## 次の Level に進む条件

次を説明できれば Level 2 に進めます。

```text
String と &str の違い
所有権を渡した後に変数が使えなくなる理由
関数が所有するべき場合と借りるだけでよい場合
```

## 公式 docs で確認する箇所

- Hello, World: https://doc.rust-lang.org/book/ch01-02-hello-world.html
- Ownership: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- String: https://doc.rust-lang.org/std/string/struct.String.html

## Rust らしさをさらに深掘りする

`String` と `&str` の違いは、Rust の最初の壁であり、最後まで使う判断軸です。`String` は所有、`&str` は借用です。この違いが読めると、関数シグネチャが設計書として見えるようになります。

追加で読む箇所:

- [所有権、借用、ライフタイム完全補講](../../appendices/01_ownership_lifetimes.md)

次の観点で `hello_ownership.rs` を読み直してください。

```text
borrow_name はなぜ &str でよいか
take_name が String を受け取る設計は本当に必要か
move した値を使えないことは、どの事故を防いでいるか
```

## 次に読む

- 前へ: [levels/level_00_philosophy/exercises.md](../level_00_philosophy/exercises.md)
- 次へ: [levels/level_01_intro/exercises.md](exercises.md)
- 関連: [docs/guide/CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md), [docs/guide/STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md)

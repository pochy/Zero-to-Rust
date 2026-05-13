# Level 2: 型、制御構文、エラー処理

## この Level でできるようになること

`std::env`、`std::fs`、`std::io`、`Result`、`?` を使い、小さな `cat` 風 CLI を作れるようになります。

この Level の中心は、ファイル I/O は失敗するものとして設計することです。

## まず知るべき言葉

- CLI: ターミナルから引数を渡して動かすプログラム。
- 標準出力: `println!` や `print!` が書き出す先。
- `std::env::args`: コマンドライン引数を読む標準 API。
- `std::fs::read_to_string`: ファイルを文字列として読む関数。
- `std::io::Error`: I/O 失敗を表す標準エラー型。
- `Result<T, E>`: 成功値 `T` または失敗値 `E` を表す型。
- `?`: 失敗を呼び出し元へ返す演算子。

## なぜこれを学ぶのか

Rust では、失敗を例外として隠すのではなく、関数の戻り値に含めます。

```rust
fn main() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string("message.txt")?;
    print!("{}", content);
    Ok(())
}
```

`read_to_string` は、ファイルがない、権限がない、UTF-8 ではない、といった理由で失敗します。その失敗を `unwrap()` で落とすのではなく、`Result` として呼び出し元へ返します。

## 手順 1: データファイルを読む

```bash
rustc --edition=2021 levels/level_02_basics/examples/mini_cat.rs -o /tmp/zero_to_rust_cat
/tmp/zero_to_rust_cat levels/level_02_basics/data/message.txt
```

期待する出力:

```text
Rust is a language for explicit responsibility.
Errors are part of the design.
```

見るべき点は、ファイル内容そのものではなく、`main` が `Result<(), io::Error>` を返していることです。

## 手順 2: 存在しないファイルを指定する

```bash
/tmp/zero_to_rust_cat levels/level_02_basics/data/missing.txt
```

期待する動きは、panic ではなくエラー終了です。エラーメッセージには OS 由来の情報が含まれます。

ここで学ぶべきことは、失敗が起きないように祈るのではなく、失敗を呼び出し元へ返せる形にしておくことです。

## 手順 3: `unwrap()` と比較する

`mini_cat.rs` の `?` を `unwrap()` に置き換えると、ファイルがない場合に panic します。

学習中に `unwrap()` を見ることはあります。しかし、I/O のように失敗が自然な処理では、次の設計を優先します。

```text
失敗を型に含める
呼び出し元へ返す
上位層で表示や復旧を決める
```

## よくあるつまずき

```text
Q. エラー型を全部 String にしてよいですか？
A. 最初は簡単に見えますが、エラーの種類が失われます。標準 API が io::Error を返すなら、まずそれを保ちます。
```

```text
Q. main が Result を返せるのは特別ですか？
A. Rust の main は Result を返せます。小さな CLI では、エラーを素直に返す入口として便利です。
```

```text
Q. expect は unwrap より良いですか？
A. メッセージを付けられる点では良いですが、失敗時に panic する点は同じです。回復可能な失敗には Result を使います。
```

## 次の Level に進む条件

```text
ファイル読み込みが失敗する理由を 3 つ言える
Result<T, E> の T と E を説明できる
? が何を呼び出し元へ返すか説明できる
unwrap を使う前に失敗の扱いを考えられる
```

## 公式 docs で確認する箇所

- Recoverable Errors with Result: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
- std::fs: https://doc.rust-lang.org/std/fs/
- std::io: https://doc.rust-lang.org/std/io/

## Rust らしさをさらに深掘りする

Rust では、失敗は例外として隠すものではなく、戻り値の型に含める設計要素です。`Result<T, E>` を見ると、呼び出し側は「成功値」と「失敗理由」の両方を意識できます。

追加で読む箇所:

- [エラー、テスト、品質](../../appendices/04_error_testing_quality.md)
- [Cargo、workspace、ecosystem](../../appendices/05_cargo_ecosystem.md)

この Level のコードを、次の観点で見直します。

```text
io::Error を String に変換すると何を失うか
main が Result を返すと、誰が失敗を表示するか
expect のメッセージは復旧可能性を増やすか
```

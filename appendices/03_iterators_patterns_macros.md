# 03: Iterator、pattern、macro

## 目的

Rust の日常コードは、所有権だけでなく iterator、pattern matching、closure、macro に支えられています。これらは短く書くための飾りではなく、データの流れと状態の形を明確にする道具です。

## Iterator は処理の列を表す

```rust
let lines: Vec<&str> = content
    .lines()
    .filter(|line| line.contains("Rust"))
    .collect();
```

`lines()` は文字列を所有しません。元の `content` を借りて、各行の `&str` を返します。ここでも所有と借用が効いています。

## `iter`、`iter_mut`、`into_iter`

```text
iter:
読むだけ。要素は &T。

iter_mut:
変更する。要素は &mut T。

into_iter:
コレクションを消費する。要素は T。
```

この 3 つは、関数引数の `&T`、`&mut T`、`T` と同じ発想です。

## closure は環境を捕まえる関数

```rust
let pattern = "Rust";
let matches = content.lines().filter(|line| line.contains(pattern));
```

closure は周囲の変数を借用、可変借用、所有のいずれかで捕まえます。スレッドに渡す closure では `move` が出てきます。

```rust
std::thread::spawn(move || {
    println!("{}", pattern);
});
```

`move` は値をスレッドへ移す設計判断です。

## pattern matching は状態を分解する

```rust
match response {
    Response::Value(value) => println!("{}", value),
    Response::NotFound => println!("missing"),
    Response::Error(message) => eprintln!("{}", message),
    Response::Ok => {}
}
```

enum と match を使うと、状態の種類をコンパイラに検査させられます。新しい variant を足したとき、処理漏れを見つけやすいのが Rust らしさです。

## `if let` と `while let`

```rust
if let Some(value) = store.get("name") {
    println!("{}", value);
}
```

`match` が大きすぎるとき、成功側だけを短く書けます。失敗側を無視してよい理由がある場合に使います。

## macro は構文を作る仕組み

`println!`、`vec!`、`format!` は macro です。関数ではできない可変個引数や構文展開を扱えます。

```rust
macro_rules! kv {
    ($key:expr => $value:expr) => {
        (String::from($key), String::from($value))
    };
}
```

macro は強力ですが、読みにくさも増えます。まず関数、trait、generics で表せないか考えます。

## 進級チェック

```text
iter / iter_mut / into_iter を所有権で説明できるか
closure が何を借りるか、何を move するか説明できるか
match が enum の状態漏れを防ぐ理由を説明できるか
macro を導入する前に関数で足りるか判断できるか
```

## 公式 docs

- https://doc.rust-lang.org/book/ch13-00-functional-features.html
- https://doc.rust-lang.org/book/ch18-00-patterns.html
- https://doc.rust-lang.org/book/ch20-05-macros.html

## 次に読む

- 前へ: [appendices/02_traits_generics.md](02_traits_generics.md)
- 次へ: [appendices/04_error_testing_quality.md](04_error_testing_quality.md)
- 関連: [appendices/README.md](README.md), [docs/guide/LEARNING_PATH.md](../docs/guide/LEARNING_PATH.md)

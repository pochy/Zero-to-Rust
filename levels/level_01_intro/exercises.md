# Level 1 Exercises

## 1. 小さく変更する

`hello_ownership.rs` の `"Rust"` を自分の名前や別の単語に変えて実行してください。

```bash
rustc --edition=2021 levels/level_01_intro/examples/hello_ownership.rs -o /tmp/zero_to_rust_hello
/tmp/zero_to_rust_hello
```

## 2. 出力を比較する

`take_name(name)` の前後に `println!("{}", name);` を置いた場合、どちらがコンパイルできるかを確認してください。

エラーが出た場合は、エラー全文を読み、どの値が移動したかを書いてください。

## 3. 設計判断を書く

次の 2 つの関数名を考えてください。

```rust
fn save_user_name(name: String) {}
fn print_user_name(name: &str) {}
```

どちらが所有権を受け取るべきか、なぜそう考えるかを書いてください。

## 提出物

```text
1. 実行した出力
2. 所有権移動によるコンパイルエラーの説明
3. String を受け取る関数と &str を受け取る関数の設計判断
```

## 進級チェック

```text
関数シグネチャを見て、その関数が所有するのか借りるのかを説明できるか？
コンパイルエラーを clone で消す前に、所有権の設計を考えられるか？
```

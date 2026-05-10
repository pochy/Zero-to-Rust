# Sol 006: 最初のログと所有権

対応教材: [Level 1: Rust に触る](../levels/level_01_intro/README.md)

## 状況

六日目。

マークは生きている。HAB はまだ空気を保持している。通信はない。代わりに、ローカル端末には Rust コンパイラがある。

「人類史上、火星で最初にやるべきことが `Hello, Rust!` になるとは思わなかった」

彼は端末に向かい、最小プログラムを実行した。

```bash
rustc --edition=2021 levels/level_01_intro/examples/hello_ownership.rs -o /tmp/zero_to_rust_hello
/tmp/zero_to_rust_hello
```

期待する出力:

```text
borrowed: Rust
owned: Rust
```

出力は地味だった。火星も地味だった。だが、地味なものほど命を救う。

## ログ

**LOG 006.1**

マーク:

「今日は `String` と `&str` をやる。正直、名前が短すぎる。`I-own-this-heap-allocated-UTF8-buffer` と `I-am-just-looking-at-your-string` くらいにしてくれれば初心者に優しい」

端末のコードを読む。

```rust
fn take_name(name: String) {}
fn borrow_name(name: &str) {}
```

マーク:

「`take_name` は所有権を取る。つまり、そいつは俺のジャガイモ袋を持って部屋を出る。`borrow_name` は借りるだけ。袋は俺の手元に残る」

彼はコメントを外そうとして、少し手を止めた。

```rust
// println!("{}", name);
```

マーク:

「ここを外せば、コンパイラが怒る。いいぞ。火星で怒ってくれる存在は貴重だ」

コンパイルエラーが出た。

マーク:

「つまり、俺は `name` を `take_name` に渡した。所有権が移動した。だから、そのあとで `name` を使うのは、出発したローバーのタイヤをまだ倉庫にあると思い込むようなものだ」

## 会話

マーク:

「管制、聞こえていないと思うが、俺は今、文字列に所有者がいることを学んでいる」

無線:

「......」

マーク:

「沈黙は承認とみなす」

マーク:

「もし `take_name` が本当に所有する必要がないなら、`&str` に変える。これで `name` はまだ使える。つまり、工具を貸しただけで、工具箱は俺のものだ」

彼はメモを残す。

```text
String を渡す:
相手に所有権を渡す。

&str を渡す:
相手は読むだけ。所有者は変わらない。
```

## Rust 任務

1. [hello_ownership.rs](../levels/level_01_intro/examples/hello_ownership.rs) を実行する。
2. `take_name(name)` の後で `println!("{}", name);` を試す。
3. コンパイルエラーから、どの値がどこへ移動したかを読む。
4. `take_name(name)` を `borrow_name(&name)` に変えた場合、何が変わるか説明する。

演習:

- [Level 1 exercises](../levels/level_01_intro/exercises.md)

## マークの独り言

「初心者はコンパイルが通らないと `clone()` を入れる。分かる。俺だって酸素が足りなければ酸素を複製したい。だが、火星はそんなに親切ではない」

「`clone()` は酸素タンクを本当にもう一本作る行為だ。タンクのラベルを書き換えるだけではない。コストがある。責任も増える」

「だから、まず問う」

```text
この関数は本当に所有する必要があるか。
読むだけではだめか。
所有権を渡したあと、呼び出し元はまだ使う必要があるか。
```

## 進級チェック

次の問いに答えられたら、Sol 012 へ進む。

```text
`String` と `&str` の違いを、自分の言葉で説明できるか。
所有権が移動した後、その値が使えない理由を説明できるか。
`clone()` を使う前に、関数の責務を見直せるか。
```

マークのメモ:

「今日は死ななかった。あと、`String` を雑に渡すと未来の自分が死ぬことを学んだ」


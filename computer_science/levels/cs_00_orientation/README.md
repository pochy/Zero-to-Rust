# CS 0: Orientation

## この Level でできるようになること

CS を「大学の科目名」ではなく、実務で判断するための道具として捉えられるようになります。

```text
なぜ data structure を学ぶのか
なぜ computer systems を学ぶのか
なぜ OS / network / DB が Web application に効くのか
なぜ数学は後回しにしてもよいが、無視はできないのか
```

## まず知るべき言葉

```text
abstraction
cost model
data structure
algorithm
memory
I/O
latency
throughput
consistency
```

## なぜこれを学ぶのか

実務の問題は、表面上は framework や library の問題に見えます。

```text
画面が遅い
API が詰まる
DB が遅い
memory を食う
queue が詰まる
lock wait が増える
cache が効かない
```

しかし根にあるのは、data structure、algorithm、I/O、network、DB、OS、concurrency の問題であることが多いです。CS を学ぶと、問題を「どの層で起きているか」に分解できます。

## 手順 1: いまの実務問題を CS に翻訳する

次の表を書いてください。

| 普段の問題 | CS の言葉 |
| --- | --- |
| 検索が遅い | index、scan、Big-O |
| UI の tree が重い | tree traversal、diff、memoization |
| API が遅い | network latency、DB query plan、serialization |
| file 処理で memory を食う | streaming、buffering、allocation |
| 並行処理が怖い | thread、lock、shared state |

## 手順 2: Rust と CS の接点を見る

Rust は、CS のコストをコードに見せます。

```text
String は heap allocation を持つ
Vec は連続 memory を持つ
HashMap は hashing と ownership を持つ
&str は借用 view である
Result は failure を型に出す
Arc<Mutex<T>> は shared mutable state を明示する
```

このため、Rust で CS を学ぶと「抽象の裏側」を見落としにくくなります。

## 手順 3: 学習順を決める

最初は次の順に進みます。

```text
CS 1: data structure
CS 2: algorithm
CS 3: computer systems
CS 4: OS / CLI / I/O
CS 5: network / web
CS 6: database
```

数学、compiler、distributed systems は重要ですが、最初から重くしすぎない方が継続しやすいです。

## よくあるつまずき

```text
全部の CS 分野を一気にやろうとする
動画を見ただけで実装しない
Big-O を暗記して、実測しない
Rust の文法学習と CS 学習を切り離してしまう
```

## 次の Level に進む条件

```text
CS を学ぶ目的を 3 つ書ける
自分の実務や興味を CS の用語に翻訳できる
Rust で CS を学ぶ利点を説明できる
```

## 公式 docs で確認する箇所

```text
Rust std::vec::Vec
Rust std::collections::HashMap
Rust std::io
Rust std::thread
```


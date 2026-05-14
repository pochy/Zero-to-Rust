# CS 1: Data Structures

## この Level でできるようになること

`Vec`、`HashMap`、stack、queue、LRU Cache を、用途、計算量、memory の観点で説明できるようになります。

## まず知るべき言葉

```text
array
Vec
HashMap
stack
queue
deque
LRU
hashing
collision
amortized cost
```

## なぜこれを学ぶのか

data structure は、性能と設計の最初の分岐です。

```text
順番に読むなら Vec が強い
key で探すなら HashMap が強い
最後に入れたものを戻すなら stack が自然
先に入れたものから処理するなら queue が自然
古いものを捨てる cache には LRU が使える
```

data structure を選ぶとは、速くしたい操作を選び、遅くなってよい操作を受け入れることです。

## 手順 1: Vec と HashMap を比較する

同じ user list を、`Vec<User>` と `HashMap<UserId, User>` で持つ場合を考えます。

```text
全件表示: Vec が自然
id lookup: HashMap が自然
sort 済み表示: Vec が自然
重複 id の防止: HashMap が自然
```

Rust では、どちらが key と value を所有するかも重要です。

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_01_data_structures/examples/vec_vs_hashmap.rs -o /tmp/cs_vec_vs_hashmap
/tmp/cs_vec_vs_hashmap
```

見るべき点:

```text
Vec は全件走査で id を探す
HashMap は key で直接探す
名前順表示では Vec を sort する方が自然
```

## 手順 2: stack と queue を作る

Rust では `Vec<T>` で stack を表せます。

```rust
let mut stack = Vec::new();
stack.push("parse");
stack.push("evaluate");
assert_eq!(stack.pop(), Some("evaluate"));
```

queue には `VecDeque<T>` が向いています。

```rust
use std::collections::VecDeque;

let mut queue = VecDeque::new();
queue.push_back("job-1");
queue.push_back("job-2");
assert_eq!(queue.pop_front(), Some("job-1"));
```

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_01_data_structures/examples/stack_queue.rs -o /tmp/cs_stack_queue
/tmp/cs_stack_queue
```

見るべき点:

```text
undo は最後に入れた操作から戻す
job queue は先に入れた job から処理する
API の形が使い方の制約になる
```

## 手順 3: LRU Cache を設計する

LRU Cache は、最近使われていない item を捨てる cache です。

必要な操作:

```text
get(key): key があれば value を返し、最近使った扱いにする
put(key, value): 追加または更新する
capacity を超えたら一番古い key を捨てる
```

考えること:

```text
HashMap だけだと lookup は速いが古さ順を持てない
VecDeque だけだと古さ順は持てるが lookup が遅い
両方を組み合わせると責任が分かれるが、整合性を保つ必要がある
```

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_01_data_structures/examples/lru_cache.rs -o /tmp/cs_lru_cache
/tmp/cs_lru_cache
```

期待する観察:

```text
get a によって a が最近使われた扱いになる
put d で capacity を超える
b が一番古いため削除される
```

## TypeScript / Go ならどう見えるか

TypeScript では `Array` と `Map` が便利です。ただし memory layout や allocation は見えにくくなります。Go では slice と map が近い比較対象です。Rust では ownership と borrow のため、cache が value を所有するのか、外から借りるのかを早めに決める必要があります。

## よくあるつまずき

```text
HashMap は常に O(1) だと思い込む
Vec の途中挿入や削除の cost を無視する
LRU を HashMap だけで作ろうとする
clone で所有権問題を隠し、allocation を増やす
```

## 次の Level に進む条件

```text
Vec と HashMap の使い分けを説明できる
stack と queue の API の違いを説明できる
LRU Cache に必要な 2 種類の責任を説明できる
```

## 公式 docs で確認する箇所

```text
std::vec::Vec
std::collections::HashMap
std::collections::VecDeque
```

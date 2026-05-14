# CS 2: Algorithms

## この Level でできるようになること

recursion、binary search、sort、tree traversal、graph traversal、Big-O を使って、処理量を見積もれるようになります。

## まず知るべき言葉

```text
recursion
base case
binary search
sort
DFS
BFS
tree
graph
Big-O
```

## なぜこれを学ぶのか

algorithm は、入力が大きくなったときに差が出ます。

```text
100 件なら何でも動く
10 万件なら O(n^2) は目立つ
1000 万件なら allocation と I/O も問題になる
```

Rust の `PERFORMANCE_LAB.md` で体感したように、同じ処理でも実装方針で速度と memory は大きく変わります。

## 手順 1: binary search を書く

sort 済み配列で、中央を見て探索範囲を半分にします。

重要なのは、code より invariant です。

```text
探索対象は常に left..right の中にある
範囲が空になったら存在しない
中央より小さければ左へ
中央より大きければ右へ
```

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_02_algorithms/examples/binary_search.rs -o /tmp/cs_binary_search
/tmp/cs_binary_search
```

見るべき点:

```text
見つかった場合は index が返る
存在しない場合は None になる
left..right の範囲が毎回狭くなる
```

## 手順 2: tree を DFS / BFS で読む

tree は file browser、DOM、AST、category、organization chart などに出ます。

```text
DFS: 深く潜る
BFS: 近い階層から見る
```

UI の tree 表示、Markdown outline、parser では、tree traversal が頻出します。

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_02_algorithms/examples/tree_walk.rs -o /tmp/cs_tree_walk
/tmp/cs_tree_walk
```

見るべき点:

```text
DFS は root から深く潜る
BFS は root に近い階層から順に見る
同じ tree でも traversal で順序が変わる
```

## 手順 3: graph を読む

graph は dependency、route、workflow、state machine に出ます。

```text
node: 対象
edge: 関係
directed graph: 向きがある
cycle: 戻ってくる経路がある
```

依存関係の build order、job の実行順、workflow の validation は graph の問題です。

補助例として、Markdown heading parser も動かします。

```bash
rustc --edition=2021 computer_science/levels/cs_02_algorithms/examples/heading_parser.rs -o /tmp/cs_heading_parser
/tmp/cs_heading_parser
```

見るべき点:

```text
line を 1 行ずつ見る
# の数を heading level に変換する
本文行は捨てる
```

## TypeScript / Go ならどう見えるか

TypeScript では recursion や tree object が書きやすい一方、深い recursion の stack や object allocation は見えにくくなります。Go は slice と map で graph を表しやすいです。Rust では tree/graph の ownership が難所になるため、arena、index、Rc/RefCell、Vec adjacency list などの設計判断が出ます。

## よくあるつまずき

```text
Big-O を暗記して実測しない
binary search の境界条件を曖昧にする
tree と graph を同じ感覚で扱う
cycle の存在を忘れる
Rust で graph を参照だらけにして borrow checker と戦う
```

## 次の Level に進む条件

```text
binary search の invariant を説明できる
DFS と BFS の違いを説明できる
tree と graph の違いを説明できる
O(n^2) がなぜ危険か説明できる
```

## 公式 docs で確認する箇所

```text
slice::binary_search
slice::sort
Vec
VecDeque
```

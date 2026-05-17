# CS 2 Exercises

## 1. binary search

`Vec<i32>` に対して binary search を自分で実装してください。

確認する case:

```text
先頭にある
末尾にある
中央にある
存在しない
空配列
要素 1 件
```

## 2. Markdown heading parser

Markdown の heading だけを取り出し、level と title を表示してください。

入力例:

```text
# Title
text
## Section
### Child
```

出力例:

```text
1 Title
2 Section
3 Child
```

## 3. tree file browser

小さな tree 構造を Rust の struct で表し、DFS と BFS の両方で表示してください。

## 提出物

```text
binary_search.rs
heading_parser.rs
tree_walk.rs
algorithm_notes.md
```

## 進級チェック

```text
binary search の left/right 更新を説明できるか
DFS と BFS の出力順の違いを説明できるか
再帰を loop と stack で書き換えられるか
```

## 次に読む

- 前へ: [computer_science/levels/cs_02_algorithms/README.md](README.md)
- 次へ: [computer_science/levels/cs_03_computer_systems/README.md](../cs_03_computer_systems/README.md)
- 関連: [computer_science/CHECKPOINTS.md](../../CHECKPOINTS.md), [computer_science/SOLUTIONS.md](../../SOLUTIONS.md), [computer_science/glossary.md](../../glossary.md)

# CS 1 Exercises

## 1. Vec と HashMap の比較

1000 件の user を用意し、次を Rust で書いてください。

```text
Vec<User> から id を探す
HashMap<UserId, User> から id を探す
全 user を名前順に表示する
```

どの操作でどちらが自然か説明してください。

## 2. stack / queue

次を作ってください。

```text
undo stack
job queue
```

`Vec` と `VecDeque` のどちらを使ったか、その理由を書いてください。

## 3. LRU Cache

capacity 3 の LRU Cache を作ってください。

操作例:

```text
put a 1
put b 2
put c 3
get a
put d 4
```

このとき `b` が捨てられることを確認してください。

## 提出物

```text
data_structures_notes.md
lru_cache.rs
```

## 進級チェック

```text
HashMap だけで LRU を作ると何が足りないか説明できるか
Vec の強みと弱みを memory layout と結びつけて説明できるか
```


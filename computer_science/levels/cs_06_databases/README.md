# CS 6: Databases

## この Level でできるようになること

DB index、B-tree、transaction、lock、query plan、WAL を、実務の query performance と整合性に結びつけて説明できるようになります。

## まず知るべき言葉

```text
table
row
index
B-tree
scan
transaction
isolation
lock
deadlock
WAL
EXPLAIN
```

## なぜこれを学ぶのか

DB は Web application の中心です。しかし、多くの人は query を書けても、DB がどう実行しているかを見ません。

```text
index がないから全件 scan している
index はあるが条件に合っていない
transaction が長すぎて lock を持ち続ける
N+1 query で round trip が増える
WAL と fsync が write latency に効く
```

DB を学ぶと、application の遅さを SQL、index、transaction、network、application loop に分解できます。

## 手順 1: index を考える

index は「探すための別の data structure」です。

```text
read は速くなる
write は index 更新分だけ重くなる
storage を使う
条件に合わない index は使われない
```

index は無料ではありません。

## 手順 2: transaction と lock を考える

transaction は複数操作をまとまりとして扱います。

```text
BEGIN
UPDATE accounts SET balance = balance - 100 WHERE id = 1
UPDATE accounts SET balance = balance + 100 WHERE id = 2
COMMIT
```

途中で失敗した場合に整合性を守るには、lock、isolation、rollback が関係します。

## 手順 3: WAL と KVS を接続する

Zero to Rust の WAL は、DB の考え方への入口です。

```text
先に log に書く
その後 memory state を更新する
再起動時に log から復元する
```

production DB はさらに、page、buffer pool、checkpoint、replication、transaction log などを持ちます。

## TypeScript / Go ならどう見えるか

TypeScript の ORM は query を隠しやすいです。Go は SQL を直接扱う場面が多く、transaction の境界が見えやすいです。Rust では `sqlx` や `diesel` へ進む前に、KVS と WAL で永続化の責任を理解しておくと効果的です。

## よくあるつまずき

```text
index は多いほど良いと思う
EXPLAIN を読まずに query を直す
transaction を長く開きっぱなしにする
lock wait を application bug と切り分けられない
ORM が生成した SQL を見ない
```

## 次の Level に進む条件

```text
index が read/write に与える tradeoff を説明できる
transaction が何を守るか説明できる
lock がなぜ必要で、なぜ危険か説明できる
WAL の目的を説明できる
```

## 公式 docs で確認する箇所

```text
PostgreSQL EXPLAIN
PostgreSQL Indexes
PostgreSQL Transaction Isolation
```


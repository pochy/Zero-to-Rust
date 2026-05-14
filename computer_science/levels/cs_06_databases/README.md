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

HashMap index で scan と lookup を比較します。

```bash
rustc --edition=2021 computer_science/levels/cs_06_databases/examples/simple_index.rs -o /tmp/cs_simple_index
/tmp/cs_simple_index
```

見るべき点:

```text
scan は rows を順番に見る
index lookup は short_code から position を引く
index を作るには memory と更新 cost が必要になる
```

BTreeMap で range query を見ます。

```bash
rustc --edition=2021 computer_science/levels/cs_06_databases/examples/btree_index.rs -o /tmp/cs_btree_index
/tmp/cs_btree_index
```

見るべき点:

```text
BTreeMap は key order を保つ
range query は timestamp の範囲検索に向いている
HashMap は key order を持たない
```

## 手順 2: transaction と lock を考える

transaction は複数操作をまとまりとして扱います。

```text
BEGIN
UPDATE accounts SET balance = balance - 100 WHERE id = 1
UPDATE accounts SET balance = balance + 100 WHERE id = 2
COMMIT
```

途中で失敗した場合に整合性を守るには、lock、isolation、rollback が関係します。

transfer を transaction 的に扱う例を動かします。

```bash
rustc --edition=2021 computer_science/levels/cs_06_databases/examples/transaction_sim.rs -o /tmp/cs_transaction_sim
/tmp/cs_transaction_sim
```

見るべき点:

```text
from と to の両方を確認してから state を変更する
途中失敗したときに片方だけ更新されると整合性が壊れる
実際の DB transaction はこの atomicity を一般化する
```

lock contention も観察します。

```bash
rustc --edition=2021 computer_science/levels/cs_06_databases/examples/lock_contention.rs -o /tmp/cs_lock_contention
/tmp/cs_lock_contention
```

見るべき点:

```text
Mutex は同時に 1 つの thread だけが中に入れる
lock を持ったまま長く処理すると他の thread が待つ
transaction が長いと DB lock wait が増える問題と似ている
```

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

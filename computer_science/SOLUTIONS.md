# Computer Science Solutions

この文書は、CS Track の演習に対する回答例とレビュー観点です。

丸写し用ではありません。CS では、同じ問題に複数の妥当な設計があります。ここでは、答えそのものよりも、判断理由の書き方を重視します。

## CS 0: Orientation

回答例:

```text
最近困った問題:
大量ログ処理が遅い。

CS の分類:
I/O、streaming、allocation、HashMap aggregation。

Rust で見る点:
BufRead で 1 行ずつ読む。
String を増やしすぎない。
HashMap が key/value を所有する境界を見る。

次に作る課題:
mini grep または performance lab。
```

レビュー観点:

```text
「CS を学びたい」ではなく、どの問題を切り分けたいかを書けているか。
```

## CS 1: Data Structures

### Vec と HashMap

回答例:

```text
Vec:
全件表示、sort、順序が重要な UI 表示に向いている。
id lookup は O(n) になる。

HashMap:
id から user を探す lookup に向いている。
平均的には速いが、hashing、memory overhead、順序なしという tradeoff がある。
```

### LRU Cache

回答例:

```text
HashMap:
key から value を探す。

VecDeque:
古い key から捨てるための順序を持つ。

注意:
get した key を recent に移動する必要がある。
HashMap と VecDeque の整合性を保つ必要がある。
```

レビュー観点:

```text
速い操作だけでなく、整合性を壊しやすい場所を書けているか。
```

## CS 2: Algorithms

### Binary Search

回答例:

```text
invariant:
target が存在するなら、常に left..right の範囲にある。

更新:
middle が target より小さいなら left = middle + 1。
middle が target より大きいなら right = middle。

終了:
left == right になったら範囲が空なので存在しない。
```

### DFS / BFS

回答例:

```text
DFS:
深く潜る。file tree の全 path 展開、parser、backtracking に向く。

BFS:
近い階層から見る。最短距離、level order 表示、queue 処理に向く。
```

レビュー観点:

```text
出力順の違いを実例で説明できているか。
```

## CS 3: Computer Systems

### String と Vec

回答例:

```text
String:
UTF-8 text。pointer、length、capacity を持ち、中身は heap。

Vec<u8>:
byte sequence。text とは限らない。

&str:
borrowed UTF-8 text。所有しない。

&[u8]:
borrowed byte sequence。所有しない。
```

### clone と borrow

回答例:

```text
clone:
新しい所有値を作る。大きな String や Vec では allocation と copy が起きる。

borrow:
読むだけなら &str や &[u8] でよい。所有権を移さない。
```

レビュー観点:

```text
size_of の結果と heap 上の中身のサイズを区別しているか。
```

## CS 4: OS, CLI, And I/O

### mini grep

回答例:

```text
File を開く。
BufReader で包む。
lines で 1 行ずつ読む。
一致行は stdout に出す。
matches count や error は stderr に出す。
```

### process と thread

回答例:

```text
process:
OS から見た独立した実行単位。別 address space を持つ。

thread:
process 内の実行単位。memory を共有できる。

Rust:
process は std::process::Command。
thread は std::thread::spawn。
```

レビュー観点:

```text
I/O failure を Result と exit code に反映できているか。
```

## CS 5: Networking And Web

### HTTP と TCP

回答例:

```text
TCP:
reliable byte stream。connection reset や timeout が起こる。

HTTP:
request / response の application protocol。404 や 500 は HTTP response。

重要:
TCP connection failure と HTTP status error は別の層の失敗。
```

### Cookie / Session

回答例:

```text
Cookie:
browser に保存される。session_id などの key を入れる。

Session:
server 側で user state を持つ。

避けること:
password や秘密情報を Cookie にそのまま入れる。
```

### Retry

回答例:

```text
GET:
副作用がなければ retry しやすい。

CreateOrder / ChargeCard:
timeout 時に server 側で成功している可能性がある。
idempotency key なしの無条件 retry は危険。
```

レビュー観点:

```text
失敗を network / protocol / application の層に分けているか。
```

## CS 6: Databases

### Index

回答例:

```text
scan:
全 row を順に見る。O(n)。

HashMap index:
short_code から row position を引く。平均的には速い。

BTreeMap:
key order を持つため range query に向く。

tradeoff:
index は read を速くするが、write 時に index 更新が必要になる。
```

### Transaction

回答例:

```text
transaction に入れる:
DB 内の整合性を守る update / insert。

入れない:
email 送信など rollback できない外部副作用。

理由:
transaction が失敗して rollback しても、送信済み email は戻せない。
```

レビュー観点:

```text
read performance だけでなく write cost と lock を説明しているか。
```

## CS 7: Operating Systems

### OS Error

回答例:

```text
not found:
path が存在しない。

permission denied:
OS が権限不足として拒否した。

is directory:
application は file として扱いたいが、path は directory。
```

### CPU Bound / Blocking

回答例:

```text
CPU bound:
CPU core 数、分割 overhead、cache の影響を受ける。

blocking wait:
sleep、file、network 待ちでは thread を増やすと待ち時間を重ねられる場合がある。

注意:
thread を増やせば必ず速くなるわけではない。
```

レビュー観点:

```text
Rust ownership と OS virtual memory を別の層として説明しているか。
```

## CS 8: Languages And Compilers

### Lexer / Parser / AST

回答例:

```text
lexer:
文字列を Token の列にする。

parser:
Token の列を AST にする。

AST:
program の構造を tree として表す。

evaluator:
AST をたどって値を計算する。
```

### Error

回答例:

```text
syntax error:
1 + * 2 のように構文として読めない。

runtime error:
10 / 0 のように構文は正しいが評価できない。
```

レビュー観点:

```text
operator precedence を AST の形で説明できているか。
```

## CS 9: Capstone

### KVS

回答例:

```text
data structure:
HashMap<String, String> が memory state を持つ。

storage:
SET / DELETE を WAL に書く。
GET は state を読むだけなので WAL に書かない。

restore:
起動時に WAL を先頭から replay する。

failure:
WAL write が失敗したら memory state を更新しない方がよい。
```

### Job Queue

回答例:

```text
data structure:
VecDeque が ready queue を持つ。
HashMap が job id から job state を引く。

state transition:
Ready -> InProgress -> Done
Ready -> InProgress -> Ready
Ready -> InProgress -> Dead

failure:
worker が失敗したら attempts を増やし、max を超えたら Dead に送る。
```

### Results

回答例:

```text
測るもの:
operation count、elapsed time、ops/sec、WAL file size、restore time、error count。

bottleneck 仮説:
小さい data では parse と stdout が目立つ。
data が増えると WAL replay と HashMap memory が目立つ。
concurrency を入れると Mutex lock 範囲が目立つ。
```

レビュー観点:

```text
機能説明だけでなく、data structure、failure、measurement をまとめて説明しているか。
```

## 次に読む

- 前へ: [computer_science/CHECKPOINTS.md](CHECKPOINTS.md)
- 次へ: [computer_science/RESOURCE_GUIDE.md](RESOURCE_GUIDE.md)

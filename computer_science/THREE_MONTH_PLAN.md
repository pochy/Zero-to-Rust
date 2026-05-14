# Three Month CS Plan

この計画は、最初の 3 か月で実務に効く CS の芯を作るためのものです。

大学 CS の全体を一気にやるより、まずは次の順で進めます。

```text
1 か月目: アルゴリズム入門
2 か月目: 低レイヤ入門
3 か月目: Web に効く CS
```

主実装は Rust です。ただし、TypeScript や Go の経験がある場合は、比較実装をしても構いません。

## 1 か月目: アルゴリズム入門

学ぶこと:

```text
配列
Vec
HashMap
stack
queue
recursion
sort
binary search
Big-O
```

作るもの:

```text
LRU Cache
簡易検索エンジン
Markdown heading parser
tree file browser
```

なぜ最初にここを学ぶのか:

```text
状態管理は data structure の問題である
cache は eviction policy の問題である
検索は index と scan の問題である
tree UI は tree traversal の問題である
差分更新は graph / tree / dependency の問題である
performance 改善は Big-O と allocation の問題である
```

Rust で見るべき点:

```text
Vec<T> は連続した heap memory を使う
HashMap<K, V> は key と value を所有する
stack / queue は API の制約で使い方を表す
recursion は ownership と borrow の範囲を意識する
sort は比較関数と allocation の有無を見る
```

進級条件:

```text
O(1), O(log n), O(n), O(n log n), O(n^2) の違いを例で説明できる
HashMap と Vec の使い分けを説明できる
LRU Cache でなぜ HashMap だけでは足りないか説明できる
tree を DFS / BFS で走査できる
```

## 2 か月目: 低レイヤ入門

学ぶこと:

```text
2 進数
byte
memory
stack
heap
pointer 的な考え方
process
thread
file I/O
buffering
```

作るもの:

```text
簡易 grep
簡易 wc
file copy CLI
小さな HTTP server
```

なぜここを学ぶのか:

```text
遅い処理の多くは CPU だけでなく memory、allocation、I/O、syscall に原因がある
大きな file を一度に読むと memory を使いすぎる
小さな read/write を大量に行うと syscall cost が増える
process と thread の違いを知らないと運用時の挙動を説明しにくい
```

Rust で見るべき点:

```text
String と Vec<u8> の違い
&[u8] と &str の違い
BufReader / BufWriter の意味
thread::spawn の ownership boundary
Result で I/O failure を扱う理由
```

進級条件:

```text
stack と heap の違いを説明できる
buffering がなぜ速くなるか説明できる
process と thread の違いを説明できる
large file を streaming 処理できる
```

## 3 か月目: Web に効く CS

学ぶこと:

```text
HTTP
TCP/IP
DNS
TLS
Cookie
Session
DB index
Transaction
Lock
Query plan
```

作るもの:

```text
小さな URL shortener
Redis 風の簡易 KVS
PostgreSQL EXPLAIN を読む練習
簡易 job queue
```

なぜここを学ぶのか:

```text
Web application は network、DB、OS、runtime の上で動く
HTTP は application protocol だが、TCP の失敗や latency の影響を受ける
DNS と TLS は user から見えないが障害原因になる
DB index は速くする道具だが、write cost と lock の tradeoff がある
Transaction は便利だが isolation level を理解しないと事故る
```

Rust で見るべき点:

```text
request / response を enum や struct で表す
network error を Result で分類する
shared state を Arc<Mutex<T>> や channel で扱う
KVS の WAL と DB transaction の考え方を接続する
```

進級条件:

```text
HTTP と TCP の層の違いを説明できる
DNS が何を解決しているか説明できる
Cookie と Session の責任分担を説明できる
index が read を速くし write を重くし得る理由を説明できる
transaction と lock の関係を説明できる
```

## 3 か月後の到達基準

3 か月後に目指すのは、CS の全分野を完了することではありません。実務で頻出する問題を、下のように切り分けられる状態です。

```text
これは algorithm の問題か
これは data structure の問題か
これは memory allocation の問題か
これは I/O の問題か
これは network latency の問題か
これは DB index / lock / transaction の問題か
これは application 設計の責任境界の問題か
```

この切り分けができると、Rust だけでなく TypeScript、Go、Python、SQL、infra の debugging がかなり楽になります。


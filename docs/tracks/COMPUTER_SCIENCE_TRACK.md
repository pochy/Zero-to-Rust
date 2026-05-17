# Computer Science Track

この文書は、Zero to Rust のあとに進むための Computer Science 学習トラックです。

目的は「大学 CS の全科目を順番に消化すること」ではありません。エンジニアとして日々の設計、性能改善、障害調査、Web application、DB、インフラ、AI/ML 周辺へ効く順に、CS の芯を Rust で実装しながら身につけることです。

Rust を主軸にします。ただし、TypeScript や Go を否定しません。TypeScript は UI、状態管理、tree 構造、cache、検索の文脈で CS を見やすい言語です。Go は CLI、HTTP server、process、file I/O、concurrency を小さく試しやすい言語です。このトラックでは、主実装は Rust に寄せ、必要な箇所で「TypeScript ならどう見えるか」「Go ならどう見えるか」を比較します。

## なぜ CS を学ぶのか

Rust を学ぶだけでも、所有権、借用、失敗、共有、復旧の設計はかなり鍛えられます。ただし、実務で難しい問題は言語機能だけでは解けません。

```text
なぜこの処理は遅いのか
なぜこのデータ構造を選ぶのか
なぜ index が効くのか
なぜ lock で詰まるのか
なぜ network 越しの処理は失敗前提なのか
なぜ cache は速いが難しいのか
なぜ parser や compiler の考え方が application 設計に効くのか
```

CS は、これらを偶然ではなく説明可能にするための基礎体力です。

## 最初に学ぶ順番

おすすめ順は次です。

```text
1. データ構造とアルゴリズム
2. コンピュータの仕組み
3. OS、ネットワーク、DB
4. 数学
5. コンパイラ、分散システム、AI/ML 向け基礎
```

最初から数学へ寄りすぎる必要はありません。AI/ML、graphics、cryptography、distributed systems を深める段階で、離散数学、線形代数、確率統計、微分を足していく方が続きやすくなります。

## 3 か月の入口

詳しい日程は [computer_science/THREE_MONTH_PLAN.md](../../computer_science/THREE_MONTH_PLAN.md) にあります。

| 期間 | 主題 | 作るもの |
| --- | --- | --- |
| 1 か月目 | アルゴリズム入門 | LRU Cache、簡易検索エンジン、Markdown heading parser、tree file browser |
| 2 か月目 | 低レイヤ入門 | `grep`、`wc`、file I/O CLI、簡易 HTTP server |
| 3 か月目 | Web に効く CS | URL shortener、Redis 風 KVS、PostgreSQL `EXPLAIN` 読解、job queue |

最初のゴールは、次を説明できる状態です。

```text
HashMap や tree が何を解決するか説明できる
Big-O で処理量をざっくり見積もれる
HTTP、TCP、DNS、TLS の役割を区別できる
DB index がなぜ速いか説明できる
process、thread、memory の基本がわかる
小さな CLI と HTTP server を Rust で作れる
```

## Level 構成

| Level | 主題 | 到達点 |
| --- | --- | --- |
| CS 0 | Orientation | CS を実務の判断と接続できる |
| CS 1 | Data Structures | Vec、HashMap、stack、queue、LRU を説明できる |
| CS 2 | Algorithms | recursion、binary search、sort、tree、graph、Big-O を使える |
| CS 3 | Computer Systems | binary、CPU、memory、stack/heap、cache、allocation を説明できる |
| CS 4 | OS, CLI, I/O | file I/O、process、thread、CLI の責任を分けられる |
| CS 5 | Networking And Web | HTTP、TCP/IP、DNS、TLS、cookie/session を説明できる |
| CS 6 | Databases | index、B-tree、transaction、lock、query plan を説明できる |
| CS 7 | Operating Systems | syscall、scheduler、filesystem、virtual memory を説明できる |
| CS 8 | Languages And Compilers | lexer、parser、AST、VM、compiler の流れを説明できる |
| CS 9 | Capstone | URL shortener / KVS / queue を CS 観点で設計できる |

教材本体は [computer_science/README.md](../../computer_science/README.md) から始めます。

## 外部教材の使い方

外部教材は、順番通りに全部やる対象ではなく、目的別の参考文献として使います。詳しくは [computer_science/RESOURCE_GUIDE.md](../../computer_science/RESOURCE_GUIDE.md) にまとめています。

```text
CS50x:
CS 全体の入口として使う。

MIT 6.006:
アルゴリズムとデータ構造を深めるときに使う。

CMU 15-213:
program がどう実行され、情報を保存し、通信するかを学ぶ。

Nand2Tetris:
NAND gate から computer、assembler、VM、compiler まで作る。

Teach Yourself Computer Science:
CS 全体の地図として使う。

OSSU:
大学 CS 相当を長期で体系的に進める場合に使う。
```

## Rust で CS を学ぶ意味

Rust は CS の抽象を隠しすぎません。

```text
Vec は連続メモリである
HashMap は hashing と collision に依存する
String は heap allocation を持つ
&str は借用された view である
thread 間共有には Send / Sync の制約が出る
I/O は失敗するので Result が必要になる
```

つまり Rust で CS を学ぶと、データ構造、メモリ、I/O、並行性、失敗の責任がコードに現れます。これは、TypeScript や Python で隠れていた仕組みを理解する助けになります。

ただし、CS は Rust だけのものではありません。最終的な目的は「Rust の書き方を増やす」ことではなく、言語が変わっても通用する判断を身につけることです。

## 完了後に言えること

このトラックを終えても、「CS の全てを理解した」とは言いません。CS は広く、研究領域も実務領域も更新され続けます。

ただし、次は言える状態を目指します。

```text
代表的なデータ構造を、用途、計算量、メモリ配置で説明できる
処理速度の問題を、algorithm、allocation、I/O、network、DB のどこにあるか切り分けられる
Web application の裏側にある HTTP、TCP、DNS、TLS、DB transaction を説明できる
OS が process、thread、file、memory をどう見せているか説明できる
parser、compiler、VM の基本構造を説明できる
小さな system を、性能、失敗、永続化、並行性の観点でレビューできる
```

これができると、Rust だけでなく TypeScript、Go、Python、SQL、infra の見え方もかなり変わります。

## 次に読む

- 前へ: [docs/tracks/FUTURE_RUST_DOMAINS.md](FUTURE_RUST_DOMAINS.md)
- 次へ: [computer_science/README.md](../../computer_science/README.md)
- 関連: [docs/INDEX.md](../INDEX.md), [docs/guide/ASSESSMENT.md](../guide/ASSESSMENT.md)

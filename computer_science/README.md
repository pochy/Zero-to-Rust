# Computer Science Track

このディレクトリは、Zero to Rust の発展として CS を学ぶ教材です。

Rust の基礎を終えたあとに進むことを想定していますが、Rust 本編と並行して読んでも構いません。CS の目的は、言語や framework の使い方を増やすことではなく、性能、データ構造、メモリ、I/O、DB、ネットワーク、失敗、並行性を説明できるようになることです。

## 入口

まず読むもの:

```text
COMPUTER_SCIENCE_TRACK.md
computer_science/THREE_MONTH_PLAN.md
computer_science/RESOURCE_GUIDE.md
computer_science/glossary.md
```

短く始めたい場合は、CS 0 と CS 1 から進めてください。

## Level 一覧

| Level | フォルダ | 主題 |
| --- | --- | --- |
| CS 0 | [cs_00_orientation](levels/cs_00_orientation/README.md) | CS を学ぶ理由と実務への接続 |
| CS 1 | [cs_01_data_structures](levels/cs_01_data_structures/README.md) | Vec、HashMap、stack、queue、LRU |
| CS 2 | [cs_02_algorithms](levels/cs_02_algorithms/README.md) | recursion、sort、search、tree、graph |
| CS 3 | [cs_03_computer_systems](levels/cs_03_computer_systems/README.md) | CPU、binary、memory、cache |
| CS 4 | [cs_04_os_cli_io](levels/cs_04_os_cli_io/README.md) | file I/O、process、thread、CLI |
| CS 5 | [cs_05_networking_web](levels/cs_05_networking_web/README.md) | HTTP、TCP/IP、DNS、TLS |
| CS 6 | [cs_06_databases](levels/cs_06_databases/README.md) | index、transaction、lock、query plan |
| CS 7 | [cs_07_operating_systems](levels/cs_07_operating_systems/README.md) | syscall、scheduler、filesystem、virtual memory |
| CS 8 | [cs_08_languages_compilers](levels/cs_08_languages_compilers/README.md) | lexer、parser、AST、VM、compiler |
| CS 9 | [cs_09_capstone](levels/cs_09_capstone/README.md) | URL shortener / KVS / queue の統合設計 |

## 学び方

各 Level は次の順番で進めます。

```text
1. README.md で言葉とメンタルモデルを読む
2. Rust で小さい実装課題を作る
3. TypeScript / Go なら何が隠れるか比較する
4. exercises.md で少し変更する
5. 次に進む条件を自分の言葉で説明する
```

CS は暗記科目ではありません。重要なのは、問題を見たときに「どの抽象で考えるべきか」を選べることです。

## 既存 Rust 教材との接続

| Zero to Rust | CS Track |
| --- | --- |
| Level 3 HashMap KVS | CS 1 data structures |
| Level 4 mini grep | CS 2 algorithms, CS 4 CLI/I/O |
| Level 5 TCP KVS | CS 5 networking |
| Level 7 thread pool | CS 4 process/thread, CS 7 OS |
| Level 8 WAL | CS 6 databases |
| Level 9 binary processing | CS 3 computer systems |
| PERFORMANCE_LAB.md | CS 1-4 performance reasoning |

Rust 本編では「所有、失敗、共有」を学びました。CS Track では、その判断がどのデータ構造、OS 機能、network、DB の上に乗っているかを学びます。

## 進級の考え方

各 Level の最後に、必ず次を言語化してください。

```text
何を速くしたいのか
何をメモリに持つのか
何を捨てるのか
何が失敗し得るのか
どこが同時に実行されるのか
どの抽象が隠しているコストは何か
```

答えが曖昧な場合は、実装量を増やすより、同じ課題を小さく測定してください。


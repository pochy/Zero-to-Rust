# Computer Science Checkpoints

このファイルは、CS Track の各 Level を終えたあとに「次へ進んでよいか」を判断するための確認表です。

CS は暗記量で進めるより、問題を見たときにどの層で考えるべきかを切り分けられることが重要です。

```text
data structure の問題か
algorithm の問題か
memory / allocation の問題か
I/O の問題か
network / DB / OS の問題か
concurrency / failure の問題か
```

## 使い方

各 Level の `README.md` と `exercises.md`、`examples/` を終えたら、次の 3 段階で自己評価します。

```text
A: 自分の言葉で説明でき、コードも変更できる
B: 説明はできるが、コード変更では迷う
C: 動かせたが、なぜそうなるか説明できない
```

`B` は次へ進んでも構いません。ただし、迷った概念を `STUDY_JOURNAL.md` または自分の CS notes に残してください。

`C` の場合は、次へ進む前に同じ Level の example を 1 つ小さく変更します。

## CS 0: Orientation

合格ライン:

```text
CS を学ぶ目的を実務上の問題と接続して説明できる
Rust で CS を学ぶ利点を説明できる
自分の課題を data structure / I/O / DB / network などに分類できる
```

よくある C 判定:

```text
CS を大学科目名の一覧としてだけ見ている
実装せずに外部教材を見ることが目的になっている
Rust の文法学習と CS 学習を切り離している
```

戻る場所:

```text
computer_science/levels/cs_00_orientation/README.md
COMPUTER_SCIENCE_TRACK.md
```

## CS 1: Data Structures

合格ライン:

```text
Vec と HashMap の使い分けを説明できる
stack と queue の取り出し順を API と結びつけて説明できる
LRU Cache に HashMap と順序管理が必要な理由を説明できる
```

よくある C 判定:

```text
HashMap は常に O(1) で万能だと思っている
Vec の途中挿入や削除の cost を説明できない
LRU の「最近使った順」をどこで持つか説明できない
```

戻る場所:

```text
computer_science/levels/cs_01_data_structures/README.md
computer_science/levels/cs_01_data_structures/examples/
```

## CS 2: Algorithms

合格ライン:

```text
binary search の left/right invariant を説明できる
DFS と BFS の違いを出力順で説明できる
tree と graph の違い、cycle の有無を説明できる
Big-O と実測を両方使って考えられる
```

よくある C 判定:

```text
binary search の境界条件を雰囲気で書いている
DFS と BFS を「どちらも探索」とだけ理解している
Big-O を暗記して実測や memory locality を見ない
```

戻る場所:

```text
computer_science/levels/cs_02_algorithms/README.md
computer_science/levels/cs_02_algorithms/examples/
```

## CS 3: Computer Systems

合格ライン:

```text
String / Vec の size と heap 上の中身を区別できる
byte と UTF-8 text を区別できる
clone と borrow の cost を説明できる
Vec と LinkedList の O(n) が同じでも locality で差が出ることを説明できる
```

よくある C 判定:

```text
String をただの文字列としてしか見ていない
size_of::<String>() が文字列全体の byte 数だと思っている
clone を所有権エラーの回避策としてだけ使っている
```

戻る場所:

```text
computer_science/levels/cs_03_computer_systems/README.md
computer_science/levels/cs_03_computer_systems/examples/
```

## CS 4: OS, CLI, And I/O

合格ライン:

```text
large file を streaming する理由を説明できる
stdout / stderr / exit code を使い分けられる
process と thread の違いを説明できる
I/O bound と CPU bound を区別できる
```

よくある C 判定:

```text
大きな file を常に read_to_string で読む
error message を stdout に出している
process 起動と thread 起動を同じものとして扱っている
```

戻る場所:

```text
computer_science/levels/cs_04_os_cli_io/README.md
computer_science/levels/cs_04_os_cli_io/examples/
```

## CS 5: Networking And Web

合格ライン:

```text
HTTP と TCP の層の違いを説明できる
request line の parse error と connection error を分けられる
Cookie と Session の責任分担を説明できる
retry してよい処理と危険な処理を idempotency で説明できる
```

よくある C 判定:

```text
HTTP 404 と TCP 接続失敗を同じ失敗として扱っている
Cookie に機密情報を直接入れる設計をしている
timeout した write 系 request を無条件 retry している
```

戻る場所:

```text
computer_science/levels/cs_05_networking_web/README.md
computer_science/levels/cs_05_networking_web/examples/
```

## CS 6: Databases

合格ライン:

```text
scan と index lookup の違いを説明できる
HashMap index と BTreeMap range query の違いを説明できる
transaction が守る atomicity を説明できる
Mutex contention と DB lock wait を対応づけて説明できる
```

よくある C 判定:

```text
index は増やすほど良いと思っている
transaction に email 送信など外部副作用を入れている
lock を持ったまま重い処理をしている
```

戻る場所:

```text
computer_science/levels/cs_06_databases/README.md
computer_science/levels/cs_06_databases/examples/
```

## CS 7: Operating Systems

合格ライン:

```text
OS error を io::ErrorKind として分類できる
CPU bound と blocking wait で thread の効き方が違うことを説明できる
virtual memory と Rust ownership を別の層として説明できる
scheduler により thread の実行順が固定されないことを説明できる
```

よくある C 判定:

```text
OS が返す error をすべて同じ失敗として扱っている
thread を増やせば必ず速くなると思っている
RSS と Rust の所有権を混同している
```

戻る場所:

```text
computer_science/levels/cs_07_operating_systems/README.md
computer_science/levels/cs_07_operating_systems/examples/
```

## CS 8: Languages And Compilers

合格ライン:

```text
lexer と parser の責任を分けられる
Token と AST の違いを説明できる
operator precedence が AST の形に反映されることを説明できる
syntax error と runtime error を分けられる
bytecode VM を stack machine として説明できる
```

よくある C 判定:

```text
lexer と parser を 1 つの文字列処理として混ぜている
AST を文字列のまま扱っている
division by zero を parse error として扱っている
```

戻る場所:

```text
computer_science/levels/cs_08_languages_compilers/README.md
computer_science/levels/cs_08_languages_compilers/examples/
```

## CS 9: Capstone

合格ライン:

```text
1 つの system を設計、実装、測定できる
使った data structure と計算量を説明できる
memory state と persisted state を分けて説明できる
failure mode と recovery strategy を書ける
測定結果から bottleneck の仮説を立てられる
```

よくある C 判定:

```text
機能追加だけをして、測定や失敗設計を書いていない
WAL、application log、debug log を混同している
queue の retry と duplicate execution の危険を説明できない
```

戻る場所:

```text
computer_science/levels/cs_09_capstone/README.md
computer_science/levels/cs_09_capstone/templates/
```

## 次に読む

- 前へ: [computer_science/levels/cs_09_capstone/exercises.md](levels/cs_09_capstone/exercises.md)
- 次へ: [computer_science/SOLUTIONS.md](SOLUTIONS.md)

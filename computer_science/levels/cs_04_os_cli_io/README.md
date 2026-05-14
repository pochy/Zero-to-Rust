# CS 4: OS, CLI, And I/O

## この Level でできるようになること

file I/O、buffering、process、thread、CLI tool の設計を説明できるようになります。

## まず知るべき言葉

```text
file descriptor
stdin
stdout
stderr
buffering
syscall
process
thread
exit code
```

## なぜこれを学ぶのか

CLI tool は CS の良い教材です。入力、出力、失敗、I/O、buffer、process の考え方が小さくまとまっています。

```text
grep: 検索、streaming、buffering
wc: byte、line、word の数え方
cat: file I/O と error handling
HTTP server: socket、thread、shared state
```

## 手順 1: grep を作る

大きな file を読むとき、全体を memory に持つ必要はありません。

```text
File を開く
BufReader で包む
1 行ずつ読む
keyword を含む行だけ出す
```

これは `PERFORMANCE_LAB.md` の streaming 処理と同じ考え方です。

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_04_os_cli_io/examples/mini_grep.rs -o /tmp/cs_mini_grep
/tmp/cs_mini_grep Rust START_HERE.md
```

見るべき点:

```text
BufReader で 1 行ずつ読む
一致行は stdout に出る
matches count は stderr に出る
file open や read の失敗は Result と exit code で扱う
```

## 手順 2: wc を作る

`wc` は単純に見えますが、何を数えるかを決める必要があります。

```text
byte count
line count
word count
unicode scalar count
grapheme count
```

実務でも「数える」ときは定義が重要です。

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_04_os_cli_io/examples/mini_wc.rs -o /tmp/cs_mini_wc
/tmp/cs_mini_wc START_HERE.md
```

見るべき点:

```text
byte count と word count は違う
text として解釈する処理には UTF-8 の前提が入る
line の定義には newline の扱いが関係する
```

## 手順 3: process と thread を分ける

```text
process: OS から見た独立した実行単位
thread: process 内の並行実行単位
```

Rust では `std::process::Command` で process を起動し、`std::thread::spawn` で thread を作れます。

process の実行:

```bash
rustc --edition=2021 computer_science/levels/cs_04_os_cli_io/examples/process_launcher.rs -o /tmp/cs_process_launcher
/tmp/cs_process_launcher rustc --version
```

見るべき点:

```text
外部 program は別 process として起動される
success と exit status を確認できる
起動失敗と program の異常終了は別の失敗である
```

thread の実行:

```bash
rustc --edition=2021 computer_science/levels/cs_04_os_cli_io/examples/thread_counter.rs -o /tmp/cs_thread_counter
/tmp/cs_thread_counter
```

見るべき点:

```text
thread::spawn に move で chunk の所有権を渡す
join で thread の結果を回収する
出力順は scheduler に依存するため固定されない
```

## TypeScript / Go ならどう見えるか

Node.js では event loop と stream が中心になります。Go は goroutine と channel で並行処理を扱いやすいです。Rust は thread に渡す値の ownership が明確なので、どの data を共有するか、移動するかを設計する練習になります。

## よくあるつまずき

```text
large file を read_to_string で全部読む
stdout と stderr を混ぜる
exit code を適当にする
thread を増やせば速くなると思い込む
I/O bound と CPU bound を区別しない
```

## 次の Level に進む条件

```text
buffering がなぜ効くか説明できる
process と thread の違いを説明できる
stdout / stderr / exit code を使い分けられる
```

## 公式 docs で確認する箇所

```text
std::io
std::fs::File
std::io::BufReader
std::process::Command
std::thread
```

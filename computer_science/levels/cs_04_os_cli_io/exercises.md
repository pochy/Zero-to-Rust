# CS 4 Exercises

## 1. mini grep

keyword と file path を受け取り、keyword を含む行だけ表示する CLI を作ってください。

条件:

```text
BufReader を使う
file が開けない場合は stderr に出す
見つからない場合も正常終了にする
```

## 2. mini wc

次を数える CLI を作ってください。

```text
bytes
lines
words
```

`String` ではなく byte で数える場合と、text として数える場合の違いを書いてください。

## 3. process launcher

`std::process::Command` を使って外部 command を起動し、exit status を表示してください。

## 提出物

```text
mini_grep.rs
mini_wc.rs
process_launcher.rs
cli_notes.md
```

## 進級チェック

```text
large file を streaming する理由を説明できるか
stderr と exit code の役割を説明できるか
process 起動と thread 起動の違いを説明できるか
```


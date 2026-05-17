# CS 8 Exercises

## 1. lexer

次の token を扱う lexer を作ってください。

```text
number
+
-
*
/
(
)
```

空白は無視します。不正な文字は error にしてください。

## 2. parser

四則演算の AST を作ってください。

最初は precedence を無視して構いません。その後、`*` と `/` が `+` と `-` より先に評価されるように直してください。

## 3. evaluator

AST を評価してください。

確認:

```text
1 + 2 * 3 = 7
(1 + 2) * 3 = 9
10 / 2 + 4 = 9
```

## 提出物

```text
lexer.rs
parser.rs
evaluator.rs
language_notes.md
```

## 進級チェック

```text
token と AST の違いを説明できるか
parse error と runtime error を分けられるか
enum が AST に向いている理由を説明できるか
```

## 次に読む

- 前へ: [computer_science/levels/cs_08_languages_compilers/README.md](README.md)
- 次へ: [computer_science/levels/cs_09_capstone/README.md](../cs_09_capstone/README.md)
- 関連: [computer_science/CHECKPOINTS.md](../../CHECKPOINTS.md), [computer_science/SOLUTIONS.md](../../SOLUTIONS.md), [computer_science/glossary.md](../../glossary.md)

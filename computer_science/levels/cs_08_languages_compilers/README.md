# CS 8: Languages And Compilers

## この Level でできるようになること

lexer、parser、AST、type checking、bytecode VM、compiler の基本的な流れを説明できるようになります。

## まず知るべき言葉

```text
token
lexer
parser
AST
grammar
interpreter
bytecode
VM
compiler
type checker
```

## なぜこれを学ぶのか

言語処理系を学ぶと、普段使っている言語や framework の見方が変わります。

```text
error message がなぜ出るか
syntax と semantics の違い
type system が何を防いでいるか
macro や template が何を生成しているか
SQL parser や query planner が何をしているか
```

Nand2Tetris は、hardware から assembler、VM、compiler までを project としてつなげる教材です。この Level では、Rust で小さな expression language を作り、言語処理の流れを体験します。

## 手順 1: lexer を作る

入力文字列を token に分けます。

```text
"1 + 2 * 3"
Number(1), Plus, Number(2), Star, Number(3)
```

lexer は文字の列を意味のある単位に変換します。

実行:

```bash
rustc --edition=2021 computer_science/levels/cs_08_languages_compilers/examples/lexer.rs -o /tmp/cs_lexer
/tmp/cs_lexer
```

見るべき点:

```text
文字列が Token の列に変換される
空白は捨てられる
不正な文字は lexer error になる
```

## 手順 2: parser と AST を作る

token の列から tree を作ります。

```text
1 + 2 * 3

Add(
  Number(1),
  Mul(Number(2), Number(3))
)
```

operator precedence を扱うと、parser の設計が見えてきます。

parser と evaluator をまとめて動かします。

```bash
rustc --edition=2021 computer_science/levels/cs_08_languages_compilers/examples/expression_language.rs -o /tmp/cs_expression_language
/tmp/cs_expression_language
```

見るべき点:

```text
1 + 2 * 3 は 7 になる
(1 + 2) * 3 は 9 になる
parser は precedence を AST の形に反映する
syntax error と runtime error は別である
```

## 手順 3: evaluator を作る

AST を評価して結果を出します。

```text
1 + 2 * 3 = 7
```

ここまで作ると、compiler の前段が何をしているか見えます。

bytecode VM の最小例も動かします。

```bash
rustc --edition=2021 computer_science/levels/cs_08_languages_compilers/examples/bytecode_vm.rs -o /tmp/cs_bytecode_vm
/tmp/cs_bytecode_vm
```

見るべき点:

```text
Instruction の列を stack machine が実行する
Push は値を stack に積む
Add / Mul は stack から値を取り出して結果を戻す
compiler は source code を別の表現へ変換する道具だと見える
```

## TypeScript / Go ならどう見えるか

TypeScript は AST や transpiler の文脈で理解しやすいです。Go は parser package が標準にあり、言語処理系の教材に向いています。Rust は enum と pattern matching が AST 表現に強く、parser の失敗を `Result` で扱いやすいです。

## よくあるつまずき

```text
lexer と parser の責任を混ぜる
syntax error と runtime error を混同する
AST を文字列のまま扱う
operator precedence を後回しにして壊れる
compiler は全部 machine code を出すものだと思い込む
```

## 次の Level に進む条件

```text
lexer と parser の違いを説明できる
AST を enum で表せる
syntax error と evaluation error を分けられる
interpreter と compiler の違いを説明できる
```

## 公式 docs で確認する箇所

```text
Rust enum
match
Result
Iterator
```

## 次に読む

- 前へ: [computer_science/levels/cs_07_operating_systems/exercises.md](../cs_07_operating_systems/exercises.md)
- 次へ: [computer_science/levels/cs_08_languages_compilers/exercises.md](exercises.md)
- 関連: [computer_science/CHECKPOINTS.md](../../CHECKPOINTS.md), [computer_science/SOLUTIONS.md](../../SOLUTIONS.md), [computer_science/glossary.md](../../glossary.md)

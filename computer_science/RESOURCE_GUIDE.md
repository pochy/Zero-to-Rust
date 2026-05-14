# CS Resource Guide

このガイドは、外部教材をどう使うかを整理するためのものです。全部を順番にやる必要はありません。Zero to Rust では、手元で小さく作り、必要なタイミングで外部教材を参照する方針にします。

## CS50x

URL: <https://pll.harvard.edu/course/cs50-introduction-computer-science>

使いどころ:

```text
CS 全体の入口
programming、algorithm、data structure、memory、security、web development の俯瞰
最初に全体像を掴みたいとき
```

Harvard の説明では、CS50x は computer science と programming の導入で、algorithmic thinking、abstraction、algorithms、data structures、resource management、security、software engineering、web development などを扱います。

注意点:

```text
経験者には易しい部分もある
Rust 主軸ではない
全体像を掴んだら、手元の Rust 実装へ戻る
```

## MIT 6.006 Introduction to Algorithms

URL: <https://archive.org/details/MIT6.006S20>

使いどころ:

```text
algorithm と data structure をしっかり学ぶ
計算問題の modeling
performance analysis
recursion、sorting、hashing、tree、graph、dynamic programming
```

Internet Archive の説明では、MIT 6.006 は computational problems の mathematical modeling、common algorithms、algorithmic paradigms、data structures、performance measures、analysis techniques を扱います。

注意点:

```text
数学的な説明が増える
最初から全部見るより、CS 1-2 の復習や深掘りに使う
```

## CMU 15-213 Introduction to Computer Systems

URL: <https://www.cs.cmu.edu/~213/>

使いどころ:

```text
program がどう実行されるか
情報がどう保存されるか
network でどう通信するか
performance、portability、robustness を低レイヤから理解する
```

CMU の説明では、15-213 は programmer's view で computer systems を扱い、program execution、information storage、communication、machine-level code、performance、memory、networking、concurrent computation などを扱います。

注意点:

```text
C / assembly の比重が高い
Rust の所有権と memory model を理解したあとに読むと効果が高い
```

## Nand2Tetris

URL: <https://www.nand2tetris.org/course>

使いどころ:

```text
computer を下から作る感覚を得る
Boolean logic
memory
computer architecture
machine language
assembler
VM
compiler
OS
```

Nand2Tetris の course page では、12 projects を通じて Hardware と Software の両方を進めます。Boolean Logic から始まり、Memory、Computer Architecture、Machine Language、Assembler、VM、Compiler へ進みます。

注意点:

```text
実務 Web に直結するというより、computer の全体像を作る教材
CS 8 の言語処理系や CS 3 の computer systems と相性が良い
```

## Teach Yourself Computer Science

URL: <https://teachyourselfcs.com/>

使いどころ:

```text
CS 全体の地図
programming
computer architecture
algorithms and data structures
math for CS
operating systems
computer networking
databases
languages and compilers
distributed systems
```

Teach Yourself Computer Science は、独学者や bootcamp 出身者が CS の穴を埋めるための subject guide です。9 分野を提示し、それぞれに教材を対応づけています。

注意点:

```text
長期戦向け
全部を一気にやると重い
この Track では地図として使い、手元の実装で先に体験する
```

## OSSU Computer Science

URL: <https://github.com/ossu/computer-science>

使いどころ:

```text
大学 CS 相当を長期で体系的に進めたい
online material を使って自己学習したい
CS の幅を formal に埋めたい
```

OSSU は、online materials を使った complete education in computer science として整理されています。undergraduate computer science majors の degree requirements を意識した構成です。

注意点:

```text
かなり重い
短期で実務に効かせたい場合は最初の選択にしない
この Track を終えたあと、長期ロードマップとして使う
```

## この教材での優先順位

最初の 3 か月は、次を優先します。

```text
1. 手元で Rust 実装する
2. 出力と性能を観察する
3. なぜそうなるか説明する
4. 外部教材で用語と理論を補強する
```

外部教材から入ると、理解した気分になりやすいです。実装から入ると、わからないところが具体化されます。


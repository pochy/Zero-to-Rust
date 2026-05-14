# CS Glossary

この用語集は、CS Track で頻出する言葉を短く確認するためのものです。

## Algorithm

問題を解く手順です。重要なのは「動くか」だけではなく、入力が大きくなったときに時間とメモリがどう増えるかです。

## Big-O

入力サイズ `n` が増えたとき、処理量がどの程度増えるかを大まかに表す記法です。

```text
O(1): 入力サイズにほぼ依存しない
O(log n): 入力が倍になっても少しだけ増える
O(n): 入力に比例する
O(n log n): 実用的な sort でよく出る
O(n^2): 入力が増えると急に重くなる
```

## Data Structure

データの持ち方です。`Vec`、`HashMap`、stack、queue、tree、graph などがあります。データ構造を選ぶとは、速くしたい操作と遅くなってよい操作を選ぶことです。

## Vec

Rust の可変長配列です。連続した memory に値を置くため、順番に読む処理に強いです。途中への挿入や削除は、要素移動が必要になることがあります。

## HashMap

key から value を探すための data structure です。平均的には高速ですが、hashing、collision、memory overhead を理解する必要があります。

## Stack

最後に入れたものを最初に取り出す構造です。関数呼び出し、undo、parser、DFS などで使います。

## Queue

最初に入れたものを最初に取り出す構造です。task queue、BFS、job processing などで使います。

## Tree

親子関係を持つ構造です。DOM、file system、AST、index、UI component tree などで使います。

## Graph

node と edge で関係を表す構造です。dependency、routing、social graph、workflow、state transition などで使います。

## Binary Search

sort 済みの列に対して、探索範囲を半分ずつ減らす方法です。`O(log n)` の代表例です。

## Recursion

関数が自分自身を呼ぶ構造です。tree、parser、divide and conquer で自然に使えます。終了条件を間違えると止まりません。

## Byte

8 bit の単位です。text、file、network packet、image、binary format は最終的には byte の列として扱われます。

## Stack Memory

関数呼び出しに対応して使われる memory 領域です。サイズや寿命が比較的明確です。

## Heap Memory

実行中に動的に確保される memory 領域です。`String`、`Vec`、`Box` などは heap allocation と関係します。

## Pointer

memory 上の場所を指す値です。Rust では参照、raw pointer、smart pointer など複数の形があります。

## Process

OS から見た実行中の program です。独立した address space を持ちます。

## Thread

process の中で並行に実行される流れです。memory を共有できるため速い一方、data race や lock が問題になります。

## Syscall

program が OS に仕事を頼む入口です。file read、write、network、process 操作などで使われます。

## Buffering

小さな I/O をまとめて扱うことです。syscall の回数を減らし、性能を改善できます。

## TCP

信頼性のある byte stream を提供する transport protocol です。HTTP/1.1 や HTTP/2 の土台になります。

## DNS

domain name を IP address へ解決する仕組みです。Web 障害の原因になることがあります。

## TLS

通信を暗号化し、相手の正当性を確認する仕組みです。HTTPS で使われます。

## Cookie

browser が server から受け取り、以後の request に付ける小さな情報です。session id などに使われます。

## Session

server 側で user の状態を管理する仕組みです。Cookie と組み合わせて使われることが多いです。

## Index

DB で目的の row を速く探すための補助構造です。read を速くする一方、write や storage の cost が増えます。

## Transaction

複数の操作をひとまとまりとして扱う仕組みです。整合性を守るために使います。

## Lock

同時実行される処理が同じ資源を壊さないように保護する仕組みです。待ちや deadlock の原因にもなります。

## Query Plan

DB が query をどう実行するかの計画です。`EXPLAIN` で確認できます。

## Lexer

source code の文字列を token に分ける処理です。

## Parser

token の列から構造を作る処理です。AST を作ることが多いです。

## AST

Abstract Syntax Tree の略です。program や expression の構造を tree として表します。

## VM

Virtual Machine の略です。bytecode などを実行する抽象 machine です。

## Distributed System

複数の machine が協調して動く system です。network failure、partial failure、consistency、replication が重要になります。


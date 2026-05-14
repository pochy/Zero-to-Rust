# CS 7: Operating Systems

## この Level でできるようになること

OS が process、thread、file、memory、scheduler、syscall をどう見せているか説明できるようになります。

## まず知るべき言葉

```text
kernel
user space
syscall
process
thread
scheduler
virtual memory
page
filesystem
permission
```

## なぜこれを学ぶのか

application は OS の上で動きます。OS の基本を知らないと、次の問題を説明しにくくなります。

```text
file が開けない
port が使えない
process が落ちる
memory が増え続ける
thread が詰まる
CPU 使用率が高い
disk I/O が遅い
```

OS は、hardware を直接扱う代わりに process、file、socket、virtual memory という抽象を提供します。

## 手順 1: syscall を意識する

program は file や network を直接操作しません。OS に依頼します。

```text
open
read
write
close
accept
fork / spawn
```

Rust の `std::fs` や `std::net` は、最終的に OS の機能を使います。

## 手順 2: scheduler を考える

thread は同時に動いているように見えますが、CPU core より多い runnable thread は scheduler が切り替えます。

```text
CPU bound: CPU 計算が中心
I/O bound: file / network 待ちが中心
blocking: 待っている間 thread が止まる
```

thread を増やせば速いとは限りません。

## 手順 3: virtual memory を考える

process は自分だけの memory 空間を持つように見えます。OS と CPU は virtual address を physical memory に対応づけます。

これにより process 同士の保護、memory mapping、paging などが可能になります。

## TypeScript / Go ならどう見えるか

Node.js は event loop が中心で、OS thread を直接意識しないことが多いです。Go は goroutine が軽量ですが、OS thread と scheduler の関係を runtime が隠します。Rust では std thread、async runtime、blocking I/O の違いが設計判断として出ます。

## よくあるつまずき

```text
process と thread を混同する
user space と kernel space を意識しない
thread を増やすほど速いと思う
blocking I/O と async I/O を混同する
memory leak と cache を区別しない
```

## 次の Level に進む条件

```text
syscall が何か説明できる
process と thread の違いを説明できる
scheduler がなぜ必要か説明できる
virtual memory の役割を説明できる
```

## 公式 docs で確認する箇所

```text
std::os
std::process
std::thread
std::fs
std::net
```


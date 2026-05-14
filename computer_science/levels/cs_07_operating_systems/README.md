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

file open error を分類します。

```bash
rustc --edition=2021 computer_science/levels/cs_07_operating_systems/examples/file_error_classifier.rs -o /tmp/cs_file_error_classifier
/tmp/cs_file_error_classifier
```

見るべき点:

```text
OS が返した error は io::ErrorKind として分類できる
存在しない file、directory、権限不足は別の失敗である
Rust の Result は OS failure を application に伝える境界になる
```

## 手順 2: scheduler を考える

thread は同時に動いているように見えますが、CPU core より多い runnable thread は scheduler が切り替えます。

```text
CPU bound: CPU 計算が中心
I/O bound: file / network 待ちが中心
blocking: 待っている間 thread が止まる
```

thread を増やせば速いとは限りません。

CPU bound の例を動かします。

```bash
rustc --edition=2021 computer_science/levels/cs_07_operating_systems/examples/cpu_bound_threads.rs -o /tmp/cs_cpu_bound_threads
/tmp/cs_cpu_bound_threads
```

blocking I/O に近い待ちの例も動かします。

```bash
rustc --edition=2021 computer_science/levels/cs_07_operating_systems/examples/blocking_io_threads.rs -o /tmp/cs_blocking_io_threads
/tmp/cs_blocking_io_threads
```

見るべき点:

```text
CPU bound は CPU core 数や分割 overhead の影響を受ける
blocking wait は thread を増やすと待ち時間を重ねられる場合がある
thread の出力順は scheduler に依存する
速くなるかどうかは workload と overhead 次第である
```

## 手順 3: virtual memory を考える

process は自分だけの memory 空間を持つように見えます。OS と CPU は virtual address を physical memory に対応づけます。

これにより process 同士の保護、memory mapping、paging などが可能になります。

memory 使用量を観察します。

```bash
rustc --edition=2021 computer_science/levels/cs_07_operating_systems/examples/memory_observer.rs -o /tmp/cs_memory_observer
/tmp/cs_memory_observer 128
```

別 terminal で:

```bash
ps -o pid,rss,command -p <printed-pid>
```

見るべき点:

```text
Vec<u8> を確保して page に触ると RSS に反映されやすい
process id を使うと OS の monitor tool で観察できる
Rust ownership と OS virtual memory は別の層の概念である
```

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

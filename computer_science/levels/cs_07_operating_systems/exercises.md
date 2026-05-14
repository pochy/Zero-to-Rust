# CS 7 Exercises

## 1. file permission error

読めない file、存在しない file、directory を file として開く case を試し、error を分類してください。

## 2. CPU bound と I/O bound

次の 2 つを比較してください。

```text
大量の数値計算
大量の file read
```

thread を増やしたとき、どちらが速くなりやすいか考察してください。

## 3. process memory 観察

大きな `Vec<u8>` を確保する program を作り、OS の monitor tool で memory 使用量を観察してください。

## 提出物

```text
os_error_notes.md
cpu_io_comparison.md
memory_observation.md
```

## 進級チェック

```text
OS が file error を返すことを Result と結びつけられるか
CPU bound と I/O bound の違いを説明できるか
virtual memory と Rust ownership を混同していないか
```


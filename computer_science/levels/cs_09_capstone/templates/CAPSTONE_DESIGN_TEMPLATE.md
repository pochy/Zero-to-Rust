# Capstone Design

## 1. 題材

```text
URL shortener / Redis 風 KVS / job queue
```

選んだ理由:

```text
ここに書く
```

## 2. Requirements

必須機能:

```text
ここに書く
```

やらないこと:

```text
ここに書く
```

## 3. Interface

command / HTTP / function API:

```text
ここに書く
```

正常応答:

```text
ここに書く
```

異常応答:

```text
ここに書く
```

## 4. Data Structure

使う構造:

```text
HashMap:
VecDeque:
BTreeMap:
Vec:
```

なぜそれを使うか:

```text
ここに書く
```

計算量:

```text
read:
write:
delete:
range:
```

## 5. Storage

memory に持つもの:

```text
ここに書く
```

file / WAL / DB に保存するもの:

```text
ここに書く
```

restore 手順:

```text
ここに書く
```

## 6. Failure Modes

```text
invalid input:
file open failure:
write failure:
network failure:
process crash:
duplicate request:
```

それぞれの扱い:

```text
ここに書く
```

## 7. Concurrency

共有する state:

```text
ここに書く
```

使う手段:

```text
Mutex / channel / single-thread / async runtime
```

lock を持ったまま行ってはいけない処理:

```text
ここに書く
```

## 8. Benchmark Plan

測るもの:

```text
operation count:
elapsed time:
ops/sec:
memory usage:
file size:
restore time:
error count:
```

測定コマンド:

```bash
ここに書く
```

## 9. CS Concepts Used

```text
data structure:
algorithm:
memory:
I/O:
network:
database:
OS:
concurrency:
```


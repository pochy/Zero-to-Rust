# Advanced Rust Track

この文書は、Zero to Rust を完走したあとに進むための発展学習ルートです。

本編を完了すると、Rust の中核判断はかなり強くなります。ただし、それは「Rust の全 API を知っている」状態ではありません。ここから先は、用途ごとに専門領域を選び、実務レベルまで深掘りします。

## 前提

この Advanced Track に入る前に、次を終えてください。

```text
Level 0-9 の README と exercises
appendices/ の必要箇所
projects/kvs_std
projects/kvs_ecosystem
projects/final_kvs_server
ASSESSMENT.md の最終口頭試問
```

最低ライン:

```text
所有権、借用、ライフタイムを API 設計として説明できる
Result、Option、panic、独自エラーを使い分けられる
Arc<Mutex<T>> の意味と限界を説明できる
std-only と ecosystem crate の責任分担を説明できる
unsafe を避ける理由と、使う場合の safety 条件を説明できる
```

## Track 1: Async Web And Services

目的:

```text
tokio、axum、tower、hyper の責任分担を理解し、実務 Web service を設計できるようにする。
```

学ぶこと:

```text
async runtime
task と cancellation
backpressure
timeout
graceful shutdown
middleware
extractor
HTTP error mapping
structured logging
metrics export
```

作るもの:

```text
final_kvs_server を axum + tokio に移植する。
TCP command server を残すか、HTTP JSON API に置き換えるか判断する。
```

設計課題:

```text
std-only 版の AppState はそのまま使うか
tokio::sync::Mutex と std::sync::Mutex のどちらを使うか
handler はどこまで domain error を知るべきか
shutdown 時に WAL writer をどう止めるか
```

完了条件:

```text
HTTP API の error response を仕様化できる
timeout と shutdown の方針を書ける
tracing と metrics で運用時の観察点を説明できる
```

## Track 2: Advanced Types And Lifetimes

目的:

```text
高度な型機能を、難しい記法としてではなく API 境界の表現として使えるようにする。
```

学ぶこと:

```text
generic associated types
higher-ranked trait bounds
phantom types
typestate pattern
borrowed vs owned API
zero-copy parser
Pin の基本
```

作るもの:

```text
borrowed command parser を作る。
Command<'a> が input buffer を借りる設計と、Command が String を所有する設計を比較する。
```

設計課題:

```text
zero-copy にする価値はあるか
参照を持つ struct は API 利用者へどんな制約を渡すか
型で状態遷移を表すと、どの不正操作を防げるか
```

完了条件:

```text
所有型 API と借用型 API の tradeoff を説明できる
ライフタイム注釈を「エラーを消す記号」ではなく、依存関係として説明できる
```

## Track 3: Procedural Macros

目的:

```text
macro_rules! を超えて、derive macro や attribute macro の責任を理解する。
```

学ぶこと:

```text
TokenStream
syn
quote
derive macro
attribute macro
compile-time error
macro crate の分離
```

作るもの:

```text
Command enum から help text または protocol document を生成する derive macro。
```

設計課題:

```text
macro で隠してよい重複は何か
生成コードのエラーを利用者にどう見せるか
普通の関数や trait で足りない理由は何か
```

完了条件:

```text
macro を導入すべき重複と、導入すべきでない複雑さを説明できる
生成コードを cargo expand 相当で確認する習慣を持てる
```

## Track 4: Unsafe, FFI, And Systems Boundaries

目的:

```text
unsafe を書くことではなく、safe abstraction と safety invariant を設計できるようにする。
```

学ぶこと:

```text
raw pointer
aliasing
validity
drop
MaybeUninit
repr(C)
C ABI
ownership across FFI
Miri
```

作るもの:

```text
C から呼べる小さい KVS handle API。
Rust 側で所有し、C 側には opaque pointer だけを渡す。
```

設計課題:

```text
誰が allocate し、誰が free するか
null pointer をどう扱うか
buffer length をどう検証するか
panic を FFI 境界から漏らさないために何をするか
```

完了条件:

```text
unsafe block ごとに safety comment を書ける
safe wrapper が守る invariant を説明できる
Miri で何を検査でき、何を検査できないか説明できる
```

## Track 5: Embedded And no_std

目的:

```text
標準ライブラリがない環境で、Rust の所有権と型設計をどう使うか理解する。
```

学ぶこと:

```text
core
alloc
panic handler
HAL
interrupt
critical section
heapless data structure
embedded error handling
```

作るもの:

```text
no_std で動く fixed-capacity KVS。
HashMap ではなく固定長配列または heapless collection を使う。
```

設計課題:

```text
メモリ確保できない環境で value をどう持つか
容量超過を Result でどう表すか
interrupt と共有状態をどう扱うか
```

完了条件:

```text
std、core、alloc の違いを説明できる
ヒープなしの制約を API 設計に反映できる
```

## Track 6: Performance And Profiling

目的:

```text
速そうな書き方ではなく、測定に基づいて改善する力を身につける。
```

学ぶこと:

```text
criterion
flamegraph
allocation profiling
lock contention
cache locality
benchmark noise
```

作るもの:

```text
kvs_std と final_kvs_server の benchmark suite。
parse、Store operation、WAL append、/keys の性能を分けて測る。
```

設計課題:

```text
clone を減らす価値は測定できているか
Mutex の競合はどこで起きているか
最適化で API が読みにくくなっていないか
```

完了条件:

```text
改善前後の数値を示せる
速度、可読性、安全性、保守性の tradeoff を説明できる
```

## Track 7: Data Systems And Distributed Rust

目的:

```text
単一プロセス KVS から、永続化、複製、一貫性、障害処理へ進む。
```

学ぶこと:

```text
snapshot
WAL compaction
replication log
consensus の入口
idempotency
retry
partial failure
```

作るもの:

```text
final_kvs_server に snapshot と WAL compaction を追加する。
次に leader/follower 風の replication log を設計する。
```

設計課題:

```text
WAL と snapshot の整合性をどう守るか
再送された SET/DEL をどう扱うか
ネットワーク分断時に何を保証しないか
```

完了条件:

```text
永続化と複製の違いを説明できる
障害時に守る保証と、守らない保証を明文化できる
```

## Track 8: Library Design And Public API

目的:

```text
自分だけが使う binary ではなく、他者が使う crate の API を設計できるようにする。
```

学ぶこと:

```text
semver
public error type
feature flag
documentation test
builder pattern
sealed trait
MSRV
```

作るもの:

```text
kvs_core crate を切り出し、CLI、TCP server、HTTP server から使う。
```

設計課題:

```text
public にする型と private にする型は何か
error type を将来拡張できるか
feature flag で依存をどう分けるか
```

完了条件:

```text
破壊的変更と非破壊的変更を説明できる
public API の所有型、借用型、error type を理由つきで選べる
```

## 進め方

全部を順番にやる必要はありません。目的に応じて選びます。

```text
Web service を作りたい:
Track 1 -> Track 6 -> Track 8

低レイヤーや組み込みへ進みたい:
Track 4 -> Track 5 -> Track 6

Rust の型を深く使いたい:
Track 2 -> Track 8 -> Track 3

データ基盤へ進みたい:
Track 6 -> Track 7 -> Track 1
```

## Advanced Completion Criteria

Advanced Track の到達点は、Rust の全 API 暗記ではありません。次を満たすことです。

```text
新しい crate を見たとき、どの責任を引き受けているか読める
public API の所有、借用、error、feature を設計できる
async、thread、lock、channel の選択理由を説明できる
unsafe を safe abstraction に閉じ込められる
測定に基づいて性能改善を判断できる
運用上の保証と保証しないことを文書化できる
```

ここまで来ると、「Rust のすべてを知っている」ではなく、「未知の Rust 領域を自分で読み、設計し、検証できる」と言えます。

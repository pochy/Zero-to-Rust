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

## Track 9: Compiler And Language Internals

目的:

```text
rustc、Rust Reference、RFC、MIR、borrow checker、trait solver を通じて、Rust の言語仕様と実装モデルを読む力を身につける。
```

この track は、Rust を使って設計する力から、Rust という言語そのものを理解する方向へ進むためのものです。compiler contributor になることだけが目的ではありません。実務で遭遇する難しいエラー、unsafe の議論、macro 展開、型推論の限界を、より正確に説明できるようにします。

学ぶこと:

```text
Rust Reference の読み方
RFC と edition の役割
rustc-dev-guide の読み方
parse / AST / HIR / MIR / LLVM の流れ
macro expansion
name resolution
type inference
trait solver
coherence
orphan rule
borrow checker
non-lexical lifetimes
Polonius の入口
const eval
unsafe memory model の仕様レベル
diagnostics がどう作られるか
```

作るもの:

```text
小さな Rust コードを題材に、コンパイル過程で何が起きるかを説明する notebook。

観察対象:
ownership move
trait bound error
lifetime error
macro expansion
async lowering
const eval error
unsafe precondition violation
```

設計課題:

```text
コンパイラが拒否しているのは構文か、型か、所有関係か、trait 制約かを分類する。
Rust Reference、The Book、rustc-dev-guide、RFC の役割を分ける。
言語仕様、compiler implementation、ecosystem convention を混同しない。
unsafe の議論で「実装上たまたま動く」と「仕様上保証される」を分ける。
```

読む一次情報:

```text
Rust Reference:
言語仕様に近い規則を確認する。

The Rust Programming Language:
学習者向けの説明で概念を確認する。

rustc-dev-guide:
compiler implementation の流れを確認する。

RFC:
なぜその機能が導入されたか、設計議論を確認する。

Unsafe Code Guidelines:
unsafe memory model の議論を確認する。
```

完了条件:

```text
代表的なコンパイルエラーを compiler phase と責任境界で説明できる
Rust Reference / RFC / rustc-dev-guide を読み分けられる
borrow checker と trait solver を高レベルに説明できる
coherence と orphan rule が API 設計へ与える影響を説明できる
unsafe の議論で「仕様上保証されること」と「今の実装で動くこと」を分けられる
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

Rust 言語そのものを深く理解したい:
Track 2 -> Track 4 -> Track 9

compiler / language design へ進みたい:
Track 9 -> Track 3 -> Track 8
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
Rust Reference、RFC、compiler internals を必要に応じて読める
compiler error を構文、型、所有、trait、unsafe guarantee に分類できる
```

ここまで来ると、「Rust のすべてを知っている」ではなく、「未知の Rust 領域を自分で読み、設計し、検証できる」と言えます。

## Advanced Track まで終えたら何が言えるか

Advanced Track まで完了した状態は、一般的な入門や中級教材の完了とはかなり違います。

この段階では、Rust を単に「書ける」だけではなく、次の層をまたいで考えられる状態を目指します。

```text
language:
所有権、借用、ライフタイム、型、trait、unsafe の規則を理解する。

library:
public API、error type、feature flag、semver、documentation test を設計する。

runtime:
thread、channel、lock、async runtime、task、shutdown を選択する。

systems:
FFI、no_std、performance、memory model、profiling を責任として扱う。

operations:
logging、metrics、health check、runbook、backpressure、failure mode を設計する。

compiler:
Rust Reference、RFC、rustc-dev-guide を読み、compiler error を分類する。
```

そのため、Advanced Track まで完了したなら、次のように言うのが正確です。

```text
Rust の全 API を暗記したわけではない。
Rust ecosystem の全 crate を知っているわけでもない。
しかし、Rust の主要領域を経験し、未知の Rust コード、crate、仕様、compiler 周辺の議論に自力で入っていける。
```

これは実務上はかなり高い到達点です。新しい crate や framework に出会ったときも、「使い方」だけでなく、次の問いで読めるようになります。

```text
この crate はどの責任を引き受けているか
どの型が所有し、どの API が借用しているか
失敗は public error としてどう表されているか
feature flag はどの依存や機能を切り替えているか
async runtime へ依存しているか
unsafe を使っている場合、safe abstraction はどこか
performance claim は測定で確認できるか
ドキュメントに書かれていない運用上の責任は何か
```

## 「Rust の全て」と言い切れない理由

Advanced Track は Rust の広い領域を扱いますが、それでも「Rust の全てを完全に理解した」とは言いません。

理由は、Rust が単一の固定された知識ではなく、複数の深い専門領域と進化する ecosystem から成るためです。

```text
rustc:
compiler implementation は変わり続ける。

language design:
RFC、edition、unstable feature、trait solver などは継続的に進化する。

unsafe memory model:
実務上の best practice と仕様上の保証を慎重に分ける必要がある。

async ecosystem:
tokio、tower、hyper、axum などは version と設計慣習が変わる。

embedded / no_std:
target、HAL、interrupt、memory layout によって判断が変わる。

performance:
CPU、allocator、I/O、lock contention、workload によって最適解が変わる。

domain:
database、compiler、network proxy、game engine、browser、kernel、ML runtime では、それぞれ別の制約がある。
```

したがって、到達点は次のように表現します。

```text
誤解を招く表現:
Rust の全てを理解した。

より正確な表現:
Rust の全体地図を持ち、主要領域を経験し、未知の領域を自力で読み解ける。

実務的な表現:
Rust で設計し、実装し、レビューし、性能・安全性・運用・公開 API まで含めて判断できる。
```

## Advanced Completion Self Review

Advanced Track を完了したかどうかは、読んだ文書量ではなく、次の問いへ答えられるかで判断します。

```text
1. async Web service を作るとき、tokio、axum、tower、hyper はそれぞれ何を担当しますか。
2. public API で String を受け取るべき場所と &str を受け取るべき場所を説明できますか。
3. borrowed parser と owned parser の tradeoff を説明できますか。
4. procedural macro を導入すべき重複と、導入すべきでない複雑さを分けられますか。
5. unsafe block ごとに safety invariant を書けますか。
6. FFI 境界で allocate、free、panic、null、buffer length をどう扱うか説明できますか。
7. no_std で std がないことを API 設計へ反映できますか。
8. benchmark の改善前後を数値で示し、測定の限界も説明できますか。
9. snapshot、WAL compaction、replication log の違いを説明できますか。
10. semver、MSRV、feature flag が public crate の利用者へ与える影響を説明できますか。
11. Rust Reference、The Book、RFC、rustc-dev-guide を目的別に読み分けられますか。
12. compiler error を構文、型、所有、trait、unsafe guarantee の観点で分類できますか。
```

すべてに自分のコードや読んだ crate を参照しながら答えられるなら、この教材の Advanced Track は完了です。

## 次に残る学び方

Advanced Track の後は、教材を増やすより、実際の専門領域へ入る方が効果的です。

```text
production service:
実トラフィック、監視、障害対応、SLO、deploy、security review を経験する。

open source crate:
issue、PR、semver、MSRV、docs、release を経験する。

compiler / language:
rustc-dev-guide を読み、small diagnostic improvement や documentation PR から入る。

unsafe / systems:
Miri、sanitizer、fuzzing、FFI boundary、memory layout を実例で検証する。

embedded:
実 target、HAL、interrupt、panic handler、memory constraints を扱う。

data systems:
WAL、snapshot、replication、recovery、consistency を failure test で検証する。
```

ここから先の学習は、Rust だけでなくドメインそのものの学習になります。Rust の強みは、そのドメイン知識を所有、失敗、共有、unsafe boundary、public API としてコードに落とせることです。

## Advanced Track の次にやること

Advanced Track まで本当に完了したら、次は教材を増やすより、制約のある実戦へ入ります。

おすすめ順は次です。

どの実戦領域を選ぶか迷う場合は、[FUTURE_RUST_DOMAINS.md](FUTURE_RUST_DOMAINS.md) も読んでください。Cloud Native、Data Systems、WebAssembly / Edge、Systems / Security、Embedded、CLI、AI / ML infrastructure と Rust の相性を整理しています。

Rust の速度やメモリ効率を Python と比較して体感したい場合は、[PERFORMANCE_LAB.md](../labs/PERFORMANCE_LAB.md) を先に実行してください。大量 JSONL ログ処理を題材に、streaming、allocation、GC、GIL、`BufRead`、`HashMap`、`Result` の違いを数値で確認できます。

## 1. 実プロダクトを 1 つ作る

教材のコードではなく、利用者、運用、変更、障害を持つ成果物を作ります。

候補:

```text
Rust 製 HTTP API
KVS / queue / job runner
CLI developer tool
proxy / gateway
small database / storage engine
no_std embedded tool
WebAssembly plugin runtime
```

見るべき点:

```text
仕様変更に耐えられる public API か
error は運用者と利用者の両方に説明できるか
ログ、metrics、health check は十分か
障害時にどこまで復旧できるか
依存 crate の更新方針を説明できるか
```

## 2. OSS crate に貢献する

小さな typo 修正だけでなく、test 追加、小さい bug fix、documentation 改善、diagnostic 改善に進みます。

経験できること:

```text
他人が読む public API
semver の重み
MSRV の制約
feature flag の互換性
CI matrix
review comment への対応
release note
```

OSS では、正しいコードを書くだけでは不十分です。保守者、既存利用者、将来の変更に対して説明できる設計が必要になります。

## 3. 専門領域を 1 つ選ぶ

Advanced Track のすべてを浅く続けるより、1 つの領域を深く選びます。

```text
Web backend:
tokio、axum、tower、hyper、tracing、metrics。

systems:
unsafe、FFI、memory layout、Miri、sanitizer、fuzzing。

embedded:
no_std、HAL、interrupt、panic handler、heapless。

compiler:
rustc、MIR、diagnostics、borrow checker、trait solver。

data systems:
WAL、snapshot、replication、compaction、recovery。

library design:
public API、semver、MSRV、docs.rs、feature flag。
```

選ぶ基準:

```text
自分が作りたいものに直結するか
今後の仕事や研究に必要か
Rust の強みが出る領域か
継続的に触れる codebase があるか
```

## 4. 既存の大きな Rust codebase を読む

全部理解しようとしないでください。目的は、設計判断を抜き出すことです。

候補:

```text
ripgrep:
CLI、検索、性能、エラー処理。

tokio:
async runtime、task、I/O、feature flag。

serde:
trait、derive、data model、ecosystem の中心設計。

hyper:
HTTP、async I/O、protocol boundary。

rust-analyzer:
compiler-like architecture、incremental analysis、large codebase。

uv / ruff:
高速 tooling、Python ecosystem との境界、performance-oriented Rust。

deno:
runtime、JavaScript/TypeScript 境界、V8 integration。

wasmtime:
WebAssembly runtime、sandbox、systems boundary。
```

読むときの問い:

```text
crate はどこで分割されているか
public API と internal API はどう分けているか
error type はどう設計されているか
feature flag は何を切り替えているか
unsafe はどこに閉じ込められているか
test は責任境界を守っているか
```

## 5. 自分の crate を公開する

公開すると、API 設計の重みが一気に現実になります。

最低限用意するもの:

```text
README
examples
docs.rs 向け documentation
unit tests
integration tests
CI
license
changelog
semver policy
MSRV policy
feature flag policy
```

公開前に答える問い:

```text
この crate は何をしないか
public error type は将来拡張できるか
依存 crate を利用者に押し付けていないか
feature flag の組み合わせはテストされているか
breaking change の基準は何か
```

## 推奨実戦課題: final_kvs_server を production-ish にする

この教材から最も自然につながる実戦課題は、`final_kvs_server` を発展させることです。

目的:

```text
教材用 std-only server を、運用を意識した Rust service へ近づける。
```

追加候補:

```text
axum + tokio 化
HTTP JSON API
structured tracing
prometheus metrics
config file
graceful shutdown
request size limit
WAL compaction
snapshot
integration tests
load test
Docker image
CI
README / runbook / release notes
```

この課題で確認する判断:

```text
std-only で持っていた責任を、どの crate に移すか
domain logic と transport layer を分けられているか
AppState の lock 設計を変えるべきか
WAL writer を同期にするか非同期にするか
shutdown 時にどこまで完了を待つか
metrics は利用者行動と内部状態の両方を観察できるか
load test で何を測るか
```

完了条件:

```text
README だけで起動、操作、停止、復旧ができる
runbook に代表的な障害と確認手順がある
CI で test、fmt、clippy が通る
integration test が主要 API と復旧を確認している
load test の結果をもとに限界を説明できる
本番投入しない場合でも、何が不足しているかを明記できる
```

ここまでやると、Rust の知識は教材の理解から、制約のある成果物を運用できる設計へ変わります。次にやるべきことは、学習テーマを増やすことではなく、責任ある成果物を作り、その変更と運用に向き合うことです。

## 次に読む

- 前へ: [appendices/09_from_std_to_production_ecosystem.md](../../appendices/09_from_std_to_production_ecosystem.md)
- 次へ: [docs/tracks/FUTURE_RUST_DOMAINS.md](FUTURE_RUST_DOMAINS.md)
- 関連: [docs/INDEX.md](../INDEX.md), [docs/guide/ASSESSMENT.md](../guide/ASSESSMENT.md)

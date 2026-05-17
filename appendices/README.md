# Rust Complete Map

このフォルダは、Level 0-9 を終えたあと、Rust の全体像を穴なく埋めるための補講です。

本編は「小さく動かす」「責任を説明する」「設計判断をする」順番を守ります。補講はその逆に、Rust の概念地図を広げます。分からない単語を先に暗記する場所ではなく、Level の途中で気になった概念を戻って確認する場所です。

## 読み方

```text
Level 0-3:
所有権、借用、型、Result を中心に読む。

Level 4-6:
Iterator、エラー設計、テスト、Cargo を読む。

Level 7-9:
Send/Sync、async、unsafe、FFI、パフォーマンスを読む。

最終課題:
professional map と projects/ を読み、採用判断を書く。
```

projects 側の読み方は [../projects/PROJECT_WALKTHROUGH.md](../projects/PROJECT_WALKTHROUGH.md) にあります。補講で概念を確認し、project で実装上の責任境界を読みます。

## 補講一覧

| ファイル | 目的 |
| --- | --- |
| [01_ownership_lifetimes.md](01_ownership_lifetimes.md) | 所有権、借用、ライフタイムを設計判断として整理する |
| [02_traits_generics.md](02_traits_generics.md) | trait、generics、associated type、dyn trait の使い分けを学ぶ |
| [03_iterators_patterns_macros.md](03_iterators_patterns_macros.md) | iterator、closure、pattern matching、macro_rules を理解する |
| [04_error_testing_quality.md](04_error_testing_quality.md) | エラー設計、panic、テスト、品質ゲートを実務目線で扱う |
| [05_cargo_ecosystem.md](05_cargo_ecosystem.md) | Cargo、workspace、edition、feature、主要クレートを判断する |
| [06_async_concurrency.md](06_async_concurrency.md) | thread と async、Future、runtime、Send/Sync を比較する |
| [07_unsafe_ffi_performance.md](07_unsafe_ffi_performance.md) | unsafe、FFI、no_std、性能改善の責任を学ぶ |
| [08_professional_rust_map.md](08_professional_rust_map.md) | Rust を仕事で使うための総合チェックリスト |
| [09_from_std_to_production_ecosystem.md](09_from_std_to_production_ecosystem.md) | std-only の最終課題を実務 ecosystem へ移す判断を学ぶ |

## この補講の前提

Rust の「全てを理解する」とは、全 API を暗記することではありません。

```text
どの責任を型に表すか
どこで所有するか
どこで借用するか
どこで失敗を返すか
どこから抽象化するか
どこから外部クレートに任せるか
どの unsafe 条件を人間が保証するか
```

この判断を自分の言葉で説明できる状態を、この教材の完成形とします。

## appendices まで終えたら何が言えるか

appendices まで完了したら、Rust の中核思想と、実務 Rust を学び続けるための判断軸は理解したと言えます。

具体的には、次を説明し、コードレビューで確認できる状態を目指します。

```text
所有権、借用、ライフタイム
Option、Result、panic、? の使い分け
struct、enum、trait、generics による責任境界
iterator、closure、pattern matching、macro の基本
Cargo、workspace、feature、crate 採用判断
thread、channel、Arc<Mutex<T>>
async と thread の違い
unsafe、FFI、no_std、performance の責任
std-only と ecosystem の比較
serde、clap、thiserror、anyhow、tracing、tokio の役割
```

ただし、appendices まで終えても、厳密に「Rust の全てを理解した」とは言いません。Rust には、さらに専門的な領域が残ります。

```text
本格的な async Web: axum、hyper、tower
procedural macro: syn、quote、proc_macro
高度な型: GAT、HRTB、typestate、Pin
unsafe の本格運用: aliasing、validity、Miri
FFI の実務設計
embedded / no_std 実装
performance profiling: flamegraph、criterion
public crate 設計: semver、MSRV、feature 設計
distributed systems / database 的な設計
compiler / rustc / type system の深部
```

したがって、この補講の到達点は次の表現が正確です。

```text
Rust の全 API を知っている状態ではない。
しかし、Rust の中核を理解し、未知の Rust 領域を自力で読みに行ける状態。
```

完走後にさらに進む場合は、[../ADVANCED_TRACK.md](../docs/tracks/ADVANCED_TRACK.md) を使ってください。Advanced Track では、async Web、advanced types、procedural macro、unsafe/FFI、embedded/no_std、performance、data systems、library design、compiler/language internals へ分岐します。`compiler / rustc / type system の深部` は、Advanced Track の `Track 9: Compiler And Language Internals` で扱います。

## 次に読む

- 前へ: [docs/guide/ASSESSMENT.md](../docs/guide/ASSESSMENT.md)
- 次へ: [appendices/01_ownership_lifetimes.md](01_ownership_lifetimes.md)
- 関連: [docs/guide/LEARNING_PATH.md](../docs/guide/LEARNING_PATH.md)

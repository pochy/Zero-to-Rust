# Rust Learning Checkpoints

このファイルは、各 Level を終えたあとに「次へ進んでよいか」を判断するための確認表です。

Rust は、文法を一度読んだだけでは身につきません。大事なのは、コードを見たときに次を説明できることです。

```text
誰が値を所有しているか
誰が一時的に借りているか
失敗がどこへ返るか
共有状態がどこに閉じているか
将来変更される軸がどこか
```

## 使い方

各 Level の `README.md` と `exercises.md` を終えたら、次の 3 段階で自己評価します。

```text
A: 自分の言葉で説明でき、コードも変更できる
B: 説明はできるが、コード変更では迷う
C: 動かせたが、なぜそうなるか説明できない
```

`B` は次へ進んでも構いません。ただし、該当する appendices を読み、`STUDY_JOURNAL.md` に迷った点を残してください。

`C` の場合は、次へ進む前に同じ Level の例をもう一度小さく変更します。Rust は後の Level ほど、前の曖昧さが大きくなります。

## Level 0: Philosophy

合格ライン:

```text
Rust を「速い言語」だけでなく、「責任を型に置く言語」と説明できる
所有、借用、解放、失敗を 1 つの処理で説明できる
なぜ clone や unwrap を安易に使わないのか説明できる
```

よくある C 判定:

```text
所有権をメモリの話だけとして理解している
コンパイラを邪魔な存在として見ている
Rust が拒否する設計の意味を言語化できない
```

戻る場所:

```text
levels/level_00_philosophy/README.md
appendices/01_ownership_lifetimes.md
```

## Level 1: First Rust

合格ライン:

```text
String を渡す関数と &str を受け取る関数の違いを説明できる
move 後に値が使えない理由を説明できる
コンパイルエラーを設計フィードバックとして読める
```

よくある C 判定:

```text
とりあえず clone すればよいと考えている
& を付ける場所を暗記で決めている
String と &str を単なる文字列型の違いとして扱っている
```

戻る場所:

```text
levels/level_01_intro/README.md
appendices/01_ownership_lifetimes.md
```

## Level 2: Basic CLI And Errors

合格ライン:

```text
main が Result を返す意味を説明できる
? が「失敗を隠す」のではなく「呼び出し元へ返す」仕組みだと説明できる
unwrap を使う条件を書ける
```

よくある C 判定:

```text
エラーが出たら unwrap や expect で止めればよいと思っている
Option と Result の違いを説明できない
ユーザー入力、I/O、プログラマミスを同じ失敗として扱っている
```

戻る場所:

```text
levels/level_02_basics/README.md
appendices/04_error_testing_quality.md
```

## Level 3: Data Design

合格ライン:

```text
Store が HashMap を所有する理由を説明できる
set が String を受け取り、get が &str を受け取る理由を説明できる
struct と impl で責任境界を作れる
```

よくある C 判定:

```text
すべての引数を String にしてしまう
内部表現と public API の境界を分けられない
早すぎる trait 化でコードを複雑にしている
```

戻る場所:

```text
levels/level_03_design/README.md
appendices/02_traits_generics.md
```

## Level 4: Improvement And Iteration

合格ライン:

```text
I/O、検索、表示を分ける理由を説明できる
Iterator を所有権の流れとして読める
見つからないことと失敗を分けて扱える
```

よくある C 判定:

```text
検索処理の中で表示まで行っている
Vec を作るべき場所と iterator で流す場所を区別できない
該当なしをエラーとして扱っている
```

戻る場所:

```text
levels/level_04_improvement/README.md
appendices/03_iterators_patterns_macros.md
```

## Level 5: Application Workflow

合格ライン:

```text
Command と Response を enum にする理由を説明できる
parse、execute、format を分ける理由を説明できる
wire format の互換性を設計事項として扱える
```

よくある C 判定:

```text
文字列処理と状態変更が 1 つの関数に混ざっている
GET missing を例外的な失敗として扱っている
あとから TCP をつなぐ境界が見えていない
```

戻る場所:

```text
levels/level_05_application_workflow/README.md
FINAL_PROJECT_SPEC.md
```

## Level 6: Evaluation

合格ライン:

```text
正常系、異常系、境界値のテストを分けて書ける
panic と Result の使い分けを説明できる
失敗分類を API の形に反映できる
```

よくある C 判定:

```text
動いた例だけをテストしている
エラー文言の完全一致だけに依存している
テストしやすい設計と実装後のテストを分けて考えていない
```

戻る場所:

```text
levels/level_06_evaluation/README.md
appendices/04_error_testing_quality.md
```

## Level 7: Concurrency

合格ライン:

```text
Arc と Mutex の役割を分けて説明できる
ロック範囲を短くする理由を説明できる
共有する設計と channel で渡す設計を比較できる
```

よくある C 判定:

```text
Arc<Mutex<T>> を万能な解決策として使っている
ロック中に I/O や重い処理をしている
スレッドの終了処理を設計していない
```

戻る場所:

```text
levels/level_07_integration/README.md
appendices/06_async_concurrency.md
```

## Level 8: Production

合格ライン:

```text
WAL の書き込み順序と復旧順序を説明できる
health、metrics、logs、config の責任を分けられる
std-only で背負う運用責任を説明できる
```

よくある C 判定:

```text
現在の HashMap を保存すれば永続化だと思っている
復旧時の壊れたログ行をどう扱うか決めていない
ログを println の置き換えとしてしか見ていない
```

戻る場所:

```text
levels/level_08_production/README.md
appendices/05_cargo_ecosystem.md
```

## Level 9: Professional Rust

合格ライン:

```text
境界チェックをバイナリ処理の中心として説明できる
unsafe を使わない判断と、使う場合の safety 条件を説明できる
std-only と ecosystem の境界を実務判断として説明できる
```

よくある C 判定:

```text
unsafe を高速化の魔法として扱っている
外部クレートを使うこと自体を良し悪しで判断している
最終課題の責任境界をコード参照つきで説明できない
```

戻る場所:

```text
levels/level_09_professional/README.md
appendices/07_unsafe_ffi_performance.md
appendices/09_from_std_to_production_ecosystem.md
```

## Projects Checkpoint

`projects/` に進んだら、次を比較します。

```text
kvs_std:
std-only で責任を自分で持つ実装。

kvs_ecosystem:
serde、clap、thiserror、anyhow、tracing、tokio に責任を任せる実装。

final_kvs_server:
TCP、TTL、WAL、admin HTTP、metrics、runbook を統合する実装。
```

合格ライン:

```text
どの責任を自分で持ち、どの責任を crate に任せたか説明できる
依存を増やす利点と、依存を増やさない利点の両方を説明できる
運用時に壊れる場所を 3 つ以上挙げ、検知方法と復旧方法を書ける
```

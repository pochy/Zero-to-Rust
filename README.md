# Zero to Rust

Rust を `std` 中心の基礎から実務 crate の採用判断まで、Level 0 から Level 9、補講、実務プロジェクトで段階的に学ぶチュートリアルです。

最初に読むファイルは [START_HERE.md](START_HERE.md) です。目的別に探す場合は [INDEX.md](INDEX.md) を使ってください。全体設計の背景は [TUTORIAL.md](TUTORIAL.md) にあります。全体の進行表は [LEARNING_PATH.md](LEARNING_PATH.md)、進級判断は [CHECKPOINTS.md](CHECKPOINTS.md)、学習記録は [STUDY_JOURNAL.md](STUDY_JOURNAL.md)、最終課題の仕様は [FINAL_PROJECT_SPEC.md](FINAL_PROJECT_SPEC.md) です。

## このチュートリアルの目的

この教材の目的は、Rust の構文を暗記することではありません。次の問いを、コードを書くたびに判断できるようになることです。

```text
このデータは誰が所有するのか
誰が一時的に借りるのか
いつ解放されるのか
失敗したときにどう回復するのか
スレッド間で共有してよいのか
長く保守できる構造になっているか
```

標準ライブラリから始める理由は、便利な外部クレートを否定するためではありません。`serde`、`tokio`、`clap`、`anyhow`、`tracing` などを使う前に、Rust の基礎体力である所有権、借用、`Result`、I/O、スレッド、ロック、モジュール設計を自分で判断できるようにするためです。

このリポジトリは 3 層構成です。

```text
levels/      まず進める本編。小さく動かし、設計判断を言語化する。
appendices/  Rust 全体を補完する深掘り。trait、async、unsafe、Cargo など。
projects/    Cargo workspace で作る実務演習。std-only 版と ecosystem 版を比較する。
```

## Level 0-9 ロードマップ

| Level | フォルダ | 作るもの | 設計判断 |
| --- | --- | --- | --- |
| 0 | `levels/level_00_philosophy` | 所有権キャンバス | Rust を「設計を型に表す言語」として捉える |
| 1 | `levels/level_01_intro` | Hello Rust と所有権の最小例 | 値を渡すことと借りることを区別する |
| 2 | `levels/level_02_basics` | `cat` 風 CLI | 失敗を `Result` で返す |
| 3 | `levels/level_03_design` | インメモリ KVS | データを所有する境界を決める |
| 4 | `levels/level_04_improvement` | `grep` 風検索 | I/O、探索、表示を分離する |
| 5 | `levels/level_05_application_workflow` | TCP KVS | プロトコル層と保存層を分ける |
| 6 | `levels/level_06_evaluation` | テストと失敗分類 | 「動いた」ではなく測定する |
| 7 | `levels/level_07_integration` | スレッドプール | 共有状態と並行処理を設計する |
| 8 | `levels/level_08_production` | WAL と運用設計 | 復旧性、設定、ログを設計に含める |
| 9 | `levels/level_09_professional` | バイナリ処理と最終課題 | std-only の価値と限界を判断する |

## 補講と実務プロジェクト

Level 0-9 を進めながら、必要に応じて [appendices](appendices/README.md) を参照します。補講では、所有権とライフタイム、trait/generics、iterator、macro、エラー設計、Cargo、async、unsafe、FFI、性能、実務レビュー観点を扱います。

Level 9 まで終えたら、Cargo workspace の実務演習へ進みます。

| Project | 目的 |
| --- | --- |
| `projects/kvs_std` | 標準ライブラリだけで KVS、TTL、WAL、テストを統合する |
| `projects/kvs_ecosystem` | `serde`、`clap`、`thiserror`、`anyhow`、`tracing`、`tokio` の採用判断を学ぶ |
| `projects/final_kvs_server` | TCP、WAL、TTL、admin HTTP、metrics を統合した最終成果物 |

project の読む順番と本編 Level との対応は [projects/PROJECT_WALKTHROUGH.md](projects/PROJECT_WALKTHROUGH.md) にまとめています。

各 Level の終わりでは [CHECKPOINTS.md](CHECKPOINTS.md) を使って、次へ進んでよいかを判断します。迷った点は [STUDY_JOURNAL.md](STUDY_JOURNAL.md) に残してください。

最終課題の自己レビューには [REVIEW_CHECKLIST.md](REVIEW_CHECKLIST.md) を使います。

完走後の理解確認には [ASSESSMENT.md](ASSESSMENT.md) を使います。

演習後の比較には [solutions](solutions/README.md) を使います。教える側、レビューする側の観点は [TEACHER_GUIDE.md](TEACHER_GUIDE.md) にまとめています。

ローカル Markdown リンクの確認は次で実行できます。

```bash
python3 tools/check_links.py
```

## まず動かす

Rust が入っているか確認します。

```bash
rustc --version
cargo --version
```

最初の例を実行します。

```bash
rustc --edition=2021 levels/level_01_intro/examples/hello_ownership.rs -o /tmp/zero_to_rust_hello
/tmp/zero_to_rust_hello
```

期待する出力は、`borrowed:` と `owned:` の 2 行です。ここで「借りた表示」と「所有権を受け取った表示」が分かれていることを観察します。

## 各 Level の使い方

各 Level は同じ順番で進めます。

```text
README.md を読む
examples/ を実行する
出力を観察する
exercises.md に取り組む
進級チェックに自分の言葉で答える
CHECKPOINTS.md で A/B/C 判定をする
STUDY_JOURNAL.md に判断を書く
公式 docs で一次情報を確認する
必要なら appendices/ で深掘りする
```

進級チェックに答えられない場合は、次の Level へ急がないでください。Rust は、曖昧な理解のまま進むほど後で難しく見える言語です。

## 最終到達点

最終的には、標準ライブラリだけで次を設計でき、さらに実務 crate を採用すべき境界も説明できる状態を目指します。

```text
TCP ベースのインメモリ KVS
TTL
WAL 永続化
スレッドプール
簡易 HTTP 管理エンドポイント
テスト
ログ
復旧手順
運用ドキュメント
crate 採用判断
Cargo workspace
async runtime の選択
unsafe を避ける判断と使う条件
```

完成したコードの量よりも、各判断を説明できることを重視します。プロの Rust 開発では、「コンパイルを通した」だけでは不十分です。所有、借用、失敗、共有、復旧の責任を、コードの境界に落とし込めることが重要です。

## 完走後の到達基準

この教材を終えた状態は、Rust の API をすべて暗記した状態ではありません。次を説明し、実装し、レビューできる状態です。

```text
所有権、借用、ライフタイムを API 設計として説明できる
型、enum、trait、generics で責任境界を表せる
Iterator、closure、pattern matching を所有権と結びつけて読める
Result、panic、独自エラー、テスト戦略を使い分けられる
Cargo workspace、edition、feature、crate 採用判断を説明できる
thread、channel、Arc/Mutex、async runtime の選択理由を説明できる
unsafe、FFI、no_std、性能改善の責任を説明できる
```

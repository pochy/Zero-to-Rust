# Zero to Rust

Rust を外部クレートに頼らず、`std` 中心で Level 0 から Level 9 まで段階的に学ぶチュートリアルです。

最初に読むファイルは [START_HERE.md](/Users/pochy/Projects/Zero-to-Rust/START_HERE.md) です。全体設計の背景は [TUTORIAL.md](/Users/pochy/Projects/Zero-to-Rust/TUTORIAL.md) にあります。

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

標準ライブラリだけを使う理由は、便利な外部クレートを否定するためではありません。`serde`、`tokio`、`clap`、`anyhow` などを使う前に、Rust の基礎体力である所有権、借用、`Result`、I/O、スレッド、ロック、モジュール設計を自分で判断できるようにするためです。

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
公式 docs で一次情報を確認する
```

進級チェックに答えられない場合は、次の Level へ急がないでください。Rust は、曖昧な理解のまま進むほど後で難しく見える言語です。

## 最終到達点

最終的には、標準ライブラリだけで次を設計できる状態を目指します。

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
```

完成したコードの量よりも、各判断を説明できることを重視します。プロの Rust 開発では、「コンパイルを通した」だけでは不十分です。所有、借用、失敗、共有、復旧の責任を、コードの境界に落とし込めることが重要です。

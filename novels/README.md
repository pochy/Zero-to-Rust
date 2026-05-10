# Rust 転生譚: 標準ライブラリで国を作るチュートリアル

このフォルダは、[TUTORIAL.md](../TUTORIAL.md) の内容を、スライム転生、内なる解析者、仲間集め、国家運営、魔王化、世界規模の危機という成長曲線に置き換えて読むストーリー風チュートリアルです。

[TENSURA.md](../TENSURA.md) で整理されている「弱い存在として目覚める」「捕食と解析で能力を得る」「名付けで仲間を増やす」「共同体を国家にする」「魔王として責任を負う」「神話級の危機へ至る」という構造を、Rust 学習に対応させています。

原作本文や特定作家の文体を再現するものではありません。ここでは、リムル風の主人公が、Rust の所有権、借用、`Result`、I/O、ネットワーク、テスト、並行処理、WAL、境界チェックを、ひとつずつ国づくりの課題として解いていく教材用のオリジナルシナリオにしています。

## 読み方

各ファイルは、既存の `levels/` と 1 対 1 で対応します。

| Novel | 対応 Level | 物語上の事件 | Rust の学習テーマ |
| --- | --- | --- | --- |
| [00_ownership_reincarnation.md](00_ownership_reincarnation.md) | Level 0 | 洞窟でスライムとして目覚める | 所有、借用、失敗、共有の設計思想 |
| [01_hello_ownership.md](01_hello_ownership.md) | Level 1 | 最初のスキルを発動する | `println!`、`String`、`&str`、所有権の移動 |
| [02_result_and_io.md](02_result_and_io.md) | Level 2 | 洞窟の記録を読む | `std::fs`、`std::io`、`Result`、`?` |
| [03_naming_the_store.md](03_naming_the_store.md) | Level 3 | 仲間に名を与え、台帳を作る | `struct`、`enum`、`HashMap`、モジュール分割 |
| [04_jura_search.md](04_jura_search.md) | Level 4 | ジュラの森を探索する | `Path`、`PathBuf`、再帰探索、検索 CLI |
| [05_tempest_protocol.md](05_tempest_protocol.md) | Level 5 | テンペスト通信網を作る | `TcpListener`、`TcpStream`、`BufReader`、プロトコル |
| [06_orc_disaster_tests.md](06_orc_disaster_tests.md) | Level 6 | 災厄後に「動いた」を疑う | テスト、失敗分類、品質評価 |
| [07_maou_thread_pool.md](07_maou_thread_pool.md) | Level 7 | 魔王として組織を並行運用する | `Arc`、`Mutex`、`RwLock`、スレッドプール |
| [08_walpurgis_wal.md](08_walpurgis_wal.md) | Level 8 | 開国祭と運用設計 | 設定、環境変数、ログ、WAL、復旧 |
| [09_ultimate_std_design.md](09_ultimate_std_design.md) | Level 9 | 究極能力として設計判断を得る | バイナリ処理、スマートポインタ、`unsafe` |
| [10_final_project_tempest_kvs.md](10_final_project_tempest_kvs.md) | 最終課題 | 魔国連邦テンペストの中核システムを築く | std-only マルチスレッド KVS |

## 進め方

1. この `novels/` で各 Level の「なぜそれを学ぶのか」を物語として掴む。
2. 対応する `levels/level_xx_*/README.md` に移動して、実際の例を読む。
3. `examples/` を実行し、出力とエラーを観察する。
4. `exercises.md` に取り組み、章末の進級チェックに自分の言葉で答える。

## 物語の約束

主人公は洞窟で目覚めたスライムです。目も耳も手足もなく、あるのは内なる声と、厳格な Rust コンパイラだけです。

```text
所有者を決める。
借りる範囲を決める。
失敗を隠さない。
共有するなら守る。
状態は復元できるように残す。
最後に、設計判断を説明する。
```

大賢者は、序盤ではコンパイラ、標準ライブラリ docs、百科事典のようにふるまいます。中盤以降は智慧之王へ進化し、設計レビュー、失敗分析、運用判断まで踏み込む参謀になります。

Rust も同じです。曖昧な所有権、握りつぶしたエラー、巨大な `main.rs`、広すぎるロック、復旧できない永続化。どれも、国づくりでは後で災厄になります。

この小説版では、それらをテンペスト建国の事件として扱います。

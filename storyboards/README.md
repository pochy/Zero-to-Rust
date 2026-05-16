# Rust on Mars: Storyboard Tutorial

このフォルダは、[TUTORIAL.md](../docs/guide/TUTORIAL.md) の内容を、火星サバイバル風のストーリーとして読み進めるためのチュートリアルです。

[THE_MARTIN.md](../docs/reference/THE_MARTIN.md) で整理されている「死ぬ理由をひとつずつ潰す」「在庫確認から始める」「通信を取り戻す」「故障後に復旧する」「最後はギリギリの設計判断に命を預ける」という構造を、Rust 学習に対応させています。

原作本文や特定作家の文体を再現するものではありません。ここでは、火星に取り残されたエンジニアが、Rust の所有権、借用、`Result`、I/O、ネットワーク、テスト、並行処理、WAL、境界チェックを、ひとつずつ生存問題として解いていく教材用のオリジナルシナリオにしています。

## 読み方

各ファイルは、既存の `levels/` と 1 対 1 で対応します。

| Storyboard | 対応 Level | 生存上の問題 | Rust の学習テーマ |
| --- | --- | --- | --- |
| [00_sol_000_inventory.md](00_sol_000_inventory.md) | Level 0 | 死ぬ理由を数える | 所有、借用、失敗、共有の設計思想 |
| [01_sol_006_ownership.md](01_sol_006_ownership.md) | Level 1 | 最初のログを動かす | `String` と `&str`、所有権の移動 |
| [02_sol_012_airlock_io.md](02_sol_012_airlock_io.md) | Level 2 | 壊れたエアロックのログを読む | `Result`、`?`、ファイル I/O |
| [03_sol_025_hab_store.md](03_sol_025_hab_store.md) | Level 3 | HAB の在庫台帳を作る | `HashMap`、`struct`、API 境界 |
| [04_sol_041_search_grid.md](04_sol_041_search_grid.md) | Level 4 | 砂に埋もれた部品を探す | 検索 CLI、`Path`、部分失敗 |
| [05_sol_068_protocol_contact.md](05_sol_068_protocol_contact.md) | Level 5 | 地球との通信プロトコルを作る | TCP KVS、コマンド、レスポンス |
| [06_sol_097_failure_tests.md](06_sol_097_failure_tests.md) | Level 6 | 「動いた」を信用しない | テスト、失敗分類、仕様確認 |
| [07_sol_141_thread_pool_rover.md](07_sol_141_thread_pool_rover.md) | Level 7 | ローバー改造を並行作業にする | `Arc`、`Mutex`、スレッドプール |
| [08_sol_201_wal_recovery.md](08_sol_201_wal_recovery.md) | Level 8 | HAB 破損後に状態を復元する | WAL、設定、ログ、運用 |
| [09_sol_401_launch_window.md](09_sol_401_launch_window.md) | Level 9 | 最終打ち上げ前の境界チェック | バイナリ処理、スマートポインタ、最終判断 |
| [10_final_project_rescue_plan.md](10_final_project_rescue_plan.md) | 最終課題 | 救出作戦の全体設計 | std-only KVS サーバー完成計画 |

## 進め方

1. この `storyboards/` を読んで、各 Level の「なぜそれを学ぶのか」を物語として掴む。
2. 対応する `levels/level_xx_*/README.md` に移動して、実際のコマンドを動かす。
3. `exercises.md` に戻り、物語内の任務を自分のコードで完了する。
4. 進級チェックに、自分の言葉で答える。

## 物語の約束

マークは火星でひとりです。救助はすぐには来ません。

彼が持っているのは、HAB、壊れかけたローバー、標準ライブラリ、そして「問題を分解して解く」という習慣だけです。

```text
死ぬ理由を列挙する。
ひとつ選ぶ。
測定する。
コードにする。
壊す。
直す。
ログに残す。
次へ進む。
```

Rust も同じです。曖昧な所有権、握りつぶしたエラー、巨大な `main.rs`、長すぎるロック、復旧できない状態。どれも、後で命取りになります。

このストーリーボードでは、それらを火星の事故として扱います。

## 増補版との対応

増補版では、ストーリーボード本文は既存 Level の導入として残し、深掘りは `appendices/` と `projects/` で扱います。

| 火星での問題 | 深掘り先 |
| --- | --- |
| 物資の所有者が曖昧 | `appendices/01_ownership_lifetimes.md` |
| 修理手順を抽象化しすぎる | `appendices/02_traits_generics.md` |
| ログ探索や報告が読みにくい | `appendices/03_iterators_patterns_macros.md` |
| 「動いた」を信用しすぎる | `appendices/04_error_testing_quality.md` |
| 補給品を自作するか外部に任せるか | `appendices/05_cargo_ecosystem.md` |
| 複数作業をどう並行するか | `appendices/06_async_concurrency.md` |
| 危険な近道を使うべきか | `appendices/07_unsafe_ffi_performance.md` |
| 救出計画を実装へ落とす | `projects/kvs_std`、`projects/kvs_ecosystem` |

火星編は「なぜそれを学ぶのか」をつかむ入口です。最終的な説明責任は、本編 README、補講、Cargo project で確認してください。

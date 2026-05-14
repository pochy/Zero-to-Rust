# Zero to Rust Index

この索引は、目的別に読む場所を探すための入口です。

最初から順に学ぶ場合は [START_HERE.md](START_HERE.md) を使ってください。このファイルは、学習途中で「この話はどこにあるか」を探すために使います。

## はじめに読む

| 目的 | 読む場所 |
| --- | --- |
| 今日なにから始めるか知りたい | [START_HERE.md](START_HERE.md) |
| 全体の思想と設計背景を知りたい | [TUTORIAL.md](TUTORIAL.md) |
| 学習順を確認したい | [LEARNING_PATH.md](LEARNING_PATH.md) |
| 用語を確認したい | [glossary.md](glossary.md) |
| Level ごとの進級条件を見たい | [CHECKPOINTS.md](CHECKPOINTS.md) |
| 学習記録を書きたい | [STUDY_JOURNAL.md](STUDY_JOURNAL.md) |
| 完走後の発展学習を選びたい | [ADVANCED_TRACK.md](ADVANCED_TRACK.md) |
| Rust で Computer Science を学びたい | [COMPUTER_SCIENCE_TRACK.md](COMPUTER_SCIENCE_TRACK.md) |
| CS の 3 か月プランを見たい | [computer_science/THREE_MONTH_PLAN.md](computer_science/THREE_MONTH_PLAN.md) |
| CS 外部教材の使い分けを知りたい | [computer_science/RESOURCE_GUIDE.md](computer_science/RESOURCE_GUIDE.md) |
| 今後注目される Rust 領域を知りたい | [FUTURE_RUST_DOMAINS.md](FUTURE_RUST_DOMAINS.md) |
| Python と Rust の速度・メモリ差を体感したい | [PERFORMANCE_LAB.md](PERFORMANCE_LAB.md) |

## Rust の中心概念

| 概念 | 本編 | 補講 |
| --- | --- | --- |
| 所有権、借用、ライフタイム | [Level 0](levels/level_00_philosophy/README.md), [Level 1](levels/level_01_intro/README.md) | [appendices/01_ownership_lifetimes.md](appendices/01_ownership_lifetimes.md) |
| `String` と `&str` | [Level 1](levels/level_01_intro/README.md) | [appendices/01_ownership_lifetimes.md](appendices/01_ownership_lifetimes.md) |
| `Result`、`Option`、`?` | [Level 2](levels/level_02_basics/README.md) | [appendices/04_error_testing_quality.md](appendices/04_error_testing_quality.md) |
| `struct`、`enum`、責任境界 | [Level 3](levels/level_03_design/README.md), [Level 5](levels/level_05_application_workflow/README.md) | [appendices/02_traits_generics.md](appendices/02_traits_generics.md) |
| iterator、closure、pattern | [Level 4](levels/level_04_improvement/README.md) | [appendices/03_iterators_patterns_macros.md](appendices/03_iterators_patterns_macros.md) |
| テスト、品質、失敗分類 | [Level 6](levels/level_06_evaluation/README.md) | [appendices/04_error_testing_quality.md](appendices/04_error_testing_quality.md) |
| スレッド、`Arc`、`Mutex` | [Level 7](levels/level_07_integration/README.md) | [appendices/06_async_concurrency.md](appendices/06_async_concurrency.md) |
| WAL、復旧、運用 | [Level 8](levels/level_08_production/README.md) | [appendices/05_cargo_ecosystem.md](appendices/05_cargo_ecosystem.md) |
| `unsafe`、FFI、性能 | [Level 9](levels/level_09_professional/README.md) | [appendices/07_unsafe_ffi_performance.md](appendices/07_unsafe_ffi_performance.md) |

## 演習と解答

| 目的 | 読む場所 |
| --- | --- |
| Level 0-4 の回答例を見る | [solutions/levels_00_04.md](solutions/levels_00_04.md) |
| Level 5-9 の回答例を見る | [solutions/levels_05_09.md](solutions/levels_05_09.md) |
| 最終課題の回答例を見る | [solutions/final_project.md](solutions/final_project.md) |
| 理解確認の回答例を見る | [solutions/assessment_answers.md](solutions/assessment_answers.md) |
| 教える側の観点を見る | [TEACHER_GUIDE.md](TEACHER_GUIDE.md) |

## Projects

| 目的 | 読む場所 |
| --- | --- |
| project の読む順番を知る | [projects/PROJECT_WALKTHROUGH.md](projects/PROJECT_WALKTHROUGH.md) |
| std-only KVS を読む | [projects/kvs_std/README.md](projects/kvs_std/README.md) |
| ecosystem 版と比較する | [projects/kvs_ecosystem/README.md](projects/kvs_ecosystem/README.md) |
| 最終サーバーを動かす | [projects/final_kvs_server/README.md](projects/final_kvs_server/README.md) |
| 最終サーバーの設計を読む | [projects/final_kvs_server/DESIGN.md](projects/final_kvs_server/DESIGN.md) |
| 最終サーバーの障害確認をする | [projects/final_kvs_server/RUNBOOK.md](projects/final_kvs_server/RUNBOOK.md) |
| 最終サーバーを拡張する | [projects/final_kvs_server/EXERCISES.md](projects/final_kvs_server/EXERCISES.md) |

## Computer Science Track

| 目的 | 読む場所 |
| --- | --- |
| CS Track の全体像を見る | [COMPUTER_SCIENCE_TRACK.md](COMPUTER_SCIENCE_TRACK.md) |
| CS 教材本体の入口へ進む | [computer_science/README.md](computer_science/README.md) |
| 3 か月プランを見る | [computer_science/THREE_MONTH_PLAN.md](computer_science/THREE_MONTH_PLAN.md) |
| CS 用語を確認する | [computer_science/glossary.md](computer_science/glossary.md) |
| 外部教材を選ぶ | [computer_science/RESOURCE_GUIDE.md](computer_science/RESOURCE_GUIDE.md) |
| CS Level ごとの進級条件を見る | [computer_science/CHECKPOINTS.md](computer_science/CHECKPOINTS.md) |
| CS 演習の回答例を見る | [computer_science/SOLUTIONS.md](computer_science/SOLUTIONS.md) |
| CS 0: Orientation | [computer_science/levels/cs_00_orientation/README.md](computer_science/levels/cs_00_orientation/README.md) |
| CS 1: Data Structures | [computer_science/levels/cs_01_data_structures/README.md](computer_science/levels/cs_01_data_structures/README.md) |
| CS 2: Algorithms | [computer_science/levels/cs_02_algorithms/README.md](computer_science/levels/cs_02_algorithms/README.md) |
| CS 3: Computer Systems | [computer_science/levels/cs_03_computer_systems/README.md](computer_science/levels/cs_03_computer_systems/README.md) |
| CS 4: OS, CLI, I/O | [computer_science/levels/cs_04_os_cli_io/README.md](computer_science/levels/cs_04_os_cli_io/README.md) |
| CS 5: Networking And Web | [computer_science/levels/cs_05_networking_web/README.md](computer_science/levels/cs_05_networking_web/README.md) |
| CS 6: Databases | [computer_science/levels/cs_06_databases/README.md](computer_science/levels/cs_06_databases/README.md) |
| CS 7: Operating Systems | [computer_science/levels/cs_07_operating_systems/README.md](computer_science/levels/cs_07_operating_systems/README.md) |
| CS 8: Languages And Compilers | [computer_science/levels/cs_08_languages_compilers/README.md](computer_science/levels/cs_08_languages_compilers/README.md) |
| CS 9: Capstone | [computer_science/levels/cs_09_capstone/README.md](computer_science/levels/cs_09_capstone/README.md) |

## 品質確認

| 目的 | コマンドまたは文書 |
| --- | --- |
| Markdown link を確認する | `python3 tools/check_links.py` |
| Rust の formatting を確認する | `cargo fmt --all --check` |
| workspace の test を実行する | `cargo test --workspace` |
| clippy で lint を見る | `cargo clippy --workspace --all-targets` |
| レビュー観点を見る | [REVIEW_CHECKLIST.md](REVIEW_CHECKLIST.md) |

## 最終確認

| 目的 | 読む場所 |
| --- | --- |
| 最終課題仕様を見る | [FINAL_PROJECT_SPEC.md](FINAL_PROJECT_SPEC.md) |
| 完走後の口頭試問をする | [ASSESSMENT.md](ASSESSMENT.md) |
| professional map を読む | [appendices/08_professional_rust_map.md](appendices/08_professional_rust_map.md) |
| std-only から production ecosystem へ移る判断を読む | [appendices/09_from_std_to_production_ecosystem.md](appendices/09_from_std_to_production_ecosystem.md) |
| 完走後の Advanced Track を選ぶ | [ADVANCED_TRACK.md](ADVANCED_TRACK.md) |
| compiler / language internals へ進む | [ADVANCED_TRACK.md](ADVANCED_TRACK.md) |
| Rust で CS を学ぶ | [COMPUTER_SCIENCE_TRACK.md](COMPUTER_SCIENCE_TRACK.md) |
| 今後注目される分野と Rust の相性を見る | [FUTURE_RUST_DOMAINS.md](FUTURE_RUST_DOMAINS.md) |
| Python vs Rust performance lab を実行する | [PERFORMANCE_LAB.md](PERFORMANCE_LAB.md) |

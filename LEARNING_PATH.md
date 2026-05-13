# Learning Path

このファイルは、Zero to Rust を最後まで進めるための実用的な順路です。

## Phase 1: Rust の芯を作る

```text
START_HERE.md
STUDY_JOURNAL.md
CHECKPOINTS.md
levels/level_00_philosophy
levels/level_01_intro
levels/level_02_basics
appendices/01_ownership_lifetimes.md
```

完了条件:

```text
String と &str の違いを説明できる
Result と unwrap の違いを説明できる
clone を使う理由を書ける
CHECKPOINTS.md で Level 0-2 を B 以上にできる
```

## Phase 2: 小さいアプリを設計する

```text
levels/level_03_design
levels/level_04_improvement
levels/level_05_application_workflow
appendices/02_traits_generics.md
appendices/03_iterators_patterns_macros.md
```

完了条件:

```text
Store が何を所有するか説明できる
Command と Response を enum にする理由を説明できる
検索、表示、I/O を分ける理由を説明できる
STUDY_JOURNAL.md に「変更される軸」と「まだ抽象化しない理由」を書ける
```

## Phase 3: 品質と並行処理へ進む

```text
levels/level_06_evaluation
levels/level_07_integration
appendices/04_error_testing_quality.md
appendices/06_async_concurrency.md
```

完了条件:

```text
正常系と異常系のテストを書ける
Arc と Mutex の役割を分けて説明できる
channel で所有権を渡す選択肢を説明できる
CHECKPOINTS.md で Level 6-7 を B 以上にできる
```

## Phase 4: 本番設計と最終課題

```text
levels/level_08_production
levels/level_09_professional
appendices/05_cargo_ecosystem.md
appendices/07_unsafe_ffi_performance.md
appendices/08_professional_rust_map.md
FINAL_PROJECT_SPEC.md
projects/final_kvs_server
```

完了条件:

```text
WAL 書き込み失敗時の方針を説明できる
std-only の価値と限界を説明できる
unsafe を避ける理由と使う条件を説明できる
runbook に障害時の検知方法と復旧方法を書ける
```

## Phase 5: ecosystem と比較する

```text
projects/kvs_std
projects/kvs_ecosystem
REVIEW_CHECKLIST.md
```

完了条件:

```text
serde、clap、thiserror、anyhow、tracing、tokio に任せる責任を説明できる
外部クレートを採用しない場合の保守責任も説明できる
projects/kvs_std と projects/kvs_ecosystem の差分を責任の移動として説明できる
```

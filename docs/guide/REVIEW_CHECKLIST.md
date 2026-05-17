# Rust Review Checklist

Rust コードをレビューするときのチェックリストです。最終課題、業務コード、演習の自己レビューに使います。

## Ownership

```text
String を受け取る関数は所有する理由があるか
&str や &Path で足りる場所に所有型を要求していないか
clone は所有権分離のためか、エラー回避のためか
構造体が参照を持つ場合、寿命の制約を説明できるか
```

## Types

```text
状態の種類は enum で表されているか
存在しないことは Option、失敗は Result で表しているか
public API と内部型が分かれているか
trait は変更軸が見えてから導入されているか
```

## Errors

```text
panic する場所に根拠があるか
I/O、入力、状態、運用の失敗が分類されているか
String エラーで分類不能になっていないか
ユーザー表示と内部エラー型を混同していないか
```

## Concurrency

```text
Arc<Mutex<App>> で全体を包んでいないか
共有状態は最小限か
ロック中に I/O や await をしていないか
終了処理でスレッドを放置していないか
channel で所有権を渡す方が単純ではないか
```

## Persistence And Operations

```text
WAL に書く操作と書かない操作が明確か
WAL 書き込み失敗時に Store を更新しない方針が守られているか
復旧時に壊れた行をどう扱うか決まっているか
health、metrics、ログ、設定の責任が分かれているか
```

## Quality Gate

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
python3 tools/check_links.py
```

この 4 つは最低限です。`unsafe`、FFI、parser、バイナリ処理が増える場合は Miri、fuzzing、property test も検討します。

## Learning Gate

教材としてレビューする場合は、コード品質だけでは不十分です。

```text
START_HERE.md から迷わず最初の実行まで進めるか
各 Level の README、examples、exercises、CHECKPOINTS が対応しているか
STUDY_JOURNAL.md に書くべき判断が各 Level で発生しているか
appendices への参照が、詰まった時点で自然に見つかるか
solutions が先回りした暗記ではなく、比較材料として機能しているか
```

## Solution Review

演習回答を確認するときは [solutions](../../solutions/README.md) と [TEACHER_GUIDE.md](TEACHER_GUIDE.md) を使います。

```text
模範回答と一致しているかではなく、責任分担を説明できているかを見る。
異なる設計でも、所有、失敗、共有、復旧の説明が一貫していれば採用可能。
```

## 次に読む

- 前へ: [docs/guide/FINAL_PROJECT_SPEC.md](FINAL_PROJECT_SPEC.md)
- 次へ: [projects/PROJECT_WALKTHROUGH.md](../../projects/PROJECT_WALKTHROUGH.md)
- 関連: [docs/INDEX.md](../INDEX.md), [solutions/final_project.md](../../solutions/final_project.md)

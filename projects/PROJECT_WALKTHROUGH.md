# Projects Walkthrough

この文書は、Level 0-9 を終えたあとに `projects/` を読む順番を示します。

プロジェクトの目的は、完成コードを眺めることではありません。本編で学んだ判断が、Cargo project の境界にどう現れるかを確認することです。

```text
Level 0-3:
所有権、借用、Store が何を所有するか。

Level 4-5:
wire text、Command、Response、Store の分離。

Level 6:
parse error、state result、unit test。

Level 7:
共有状態、Arc、Mutex、lock 範囲。

Level 8:
WAL、TTL、metrics、admin endpoint、runbook。

Level 9:
std-only の限界と ecosystem への移行判断。
```

## 1. kvs_std を読む

最初に [kvs_std](kvs_std/README.md) を読みます。

この project は、最終サーバーへ進む前の中間地点です。TCP や admin HTTP はありません。代わりに、Rust の中心判断を小さい Cargo crate に閉じ込めています。

読む順番:

```text
projects/kvs_std/README.md
projects/kvs_std/src/lib.rs
projects/kvs_std/src/main.rs
```

`src/lib.rs` では、次の順に読んでください。

```text
Command:
入力を型にしたもの。wire text から切り離された操作。

Response:
表示前の結果。CLI 出力とは分ける。

ParseError:
入力が正しくない理由。Store の失敗とは分ける。

Store:
HashMap と TTL を所有する境界。

parse_command:
&str を借り、Command を所有型として返す境界。

tests:
設計判断が壊れていないか確認する場所。
```

確認する問い:

```text
なぜ parse_command は &str を受け取るのか
なぜ Command::Set は String を所有するのか
なぜ GET missing は Response::NotFound であり ParseError ではないのか
なぜ WAL は GET を記録しないのか
```

## 2. kvs_ecosystem を読む

次に [kvs_ecosystem](kvs_ecosystem/README.md) を読みます。

この project は、`kvs_std` と同じ題材を ecosystem crate へ移した比較材料です。見るべき点は、crate の使い方そのものではなく、責任がどこへ移ったかです。

読む順番:

```text
projects/kvs_ecosystem/README.md
projects/kvs_ecosystem/Cargo.toml
projects/kvs_ecosystem/src/lib.rs
projects/kvs_ecosystem/src/main.rs
```

比較する責任:

```text
serde / serde_json:
JSON の parse / serialize を任せる。

clap:
CLI 引数と help を任せる。

thiserror:
library error の表示と分類を任せる。

anyhow:
binary の上位で文脈つき error を扱う。

tracing:
構造化ログの入口を任せる。

tokio:
async runtime の形を導入する。
```

確認する問い:

```text
std-only 版で自作していた処理はどれか
crate に任せたことで読みやすくなった箇所はどこか
crate に任せても残る設計責任は何か
この小さい例で tokio を使う必要は本当にあるか
```

## 3. final_kvs_server を読む

最後に [final_kvs_server](final_kvs_server/README.md) を読みます。

この project は、本編 Level 5-9 を統合した最終例です。コードを上から順に読むより、責任境界ごとに読む方が理解しやすくなります。

読む順番:

```text
projects/final_kvs_server/README.md
projects/final_kvs_server/DESIGN.md
projects/final_kvs_server/src/lib.rs
projects/final_kvs_server/src/main.rs
projects/final_kvs_server/RUNBOOK.md
projects/final_kvs_server/EXERCISES.md
```

`src/lib.rs` では、次の順に読んでください。

```text
1. Command / Response / ParseError
2. Store / Entry
3. Metrics
4. AppState
5. parse_command
6. WAL append / restore
7. run_tcp_server
8. run_admin_server
9. tests
```

この順番にすると、TCP や HTTP より先に、中心の責任境界を理解できます。

## 4. 本編 Level との対応

| Project topic | 対応する Level | 見るべき判断 |
| --- | --- | --- |
| `Command` / `Response` | Level 5 | 文字列を早めに型へ変換する |
| `Store` | Level 3 | 構造体が状態を所有する |
| `ParseError` | Level 2, 6 | 入力エラーと状態結果を分ける |
| TTL | Level 4, 8 | 期限切れを失敗ではなく状態として扱う |
| WAL | Level 8 | 状態変更だけを記録し、復旧で再生する |
| `Arc<Mutex<AppState>>` | Level 7 | 共有状態を 1 か所に閉じる |
| admin HTTP | Level 8 | health、metrics、keys を運用入口にする |
| ecosystem 比較 | Level 9 | 自作する責任と任せる責任を分ける |

## 5. 学習記録に書くこと

[STUDY_JOURNAL.md](../docs/guide/STUDY_JOURNAL.md) に、次を書いてください。

```text
kvs_std:
自分で持っている責任。

kvs_ecosystem:
crate に任せた責任。

final_kvs_server:
本番投入前に不足している責任。

自分なら次に変える設計:
理由つきで 1 つ選ぶ。
```

## 6. 完了条件

projects を完了したと言えるのは、次をコード参照つきで説明できるときです。

```text
Store は何を所有するか
parse_command はなぜ &str を受け取るか
Command はなぜ String を所有するか
GET missing はなぜエラーではないか
WAL に書く操作と書かない操作は何か
AppState を Mutex で守る利点と限界は何か
serde、clap、thiserror、anyhow、tracing、tokio に任せた責任は何か
```

## 次に読む

- 前へ: [docs/guide/REVIEW_CHECKLIST.md](../docs/guide/REVIEW_CHECKLIST.md)
- 次へ: [projects/kvs_std/README.md](kvs_std/README.md)
- 関連: [docs/guide/FINAL_PROJECT_SPEC.md](../docs/guide/FINAL_PROJECT_SPEC.md)

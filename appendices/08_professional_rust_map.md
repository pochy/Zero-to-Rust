# 08: Professional Rust Map

## 目的

このファイルは、最終課題や実務コードを書く前の総合チェックリストです。Rust の知識を「知っている単語」ではなく「設計時に使う判断」に変換します。

## 設計前チェック

```text
データ:
どの型が何を所有するか。

API:
どの引数は借用で十分か。

失敗:
どの失敗は Result、どの前提違反は panic か。

状態:
どの状態は共有し、どの状態はメッセージで渡すか。

抽象:
trait を入れる変更軸は何か。

永続化:
どの順序で WAL や DB に書くか。

運用:
どのログ、metrics、health check が必要か。

依存:
std で学ぶ部分と crate に任せる部分はどこか。
```

## API レビュー

```text
String を受け取るなら所有する理由があるか
&str で足りる場所に String を要求していないか
Option と Result を混同していないか
public にする型と隠す型を分けているか
trait object が必要な境界か
generic によってエラーメッセージが読みにくくなっていないか
```

## 並行処理レビュー

```text
Arc<Mutex<App>> で全体を包んでいないか
ロック中に I/O や await をしていないか
終了処理で worker を join しているか
channel で所有権を移す方が単純ではないか
Send / Sync の制約を理解しているか
```

## エラーとテストのレビュー

```text
parse error、I/O error、state error、operation error を分けているか
ユーザー表示と内部分類を分けているか
正常系だけでなく異常系をテストしているか
境界値、空入力、不正 UTF-8、壊れた WAL を試しているか
cargo test / fmt / clippy を通しているか
```

## ecosystem 採用レビュー

```text
serde:
wire format や永続形式が必要か。

clap:
CLI の help、validation、subcommand が必要か。

tokio:
大量接続や async crate が必要か。

tracing:
本番でログを検索、相関、集約したいか。

thiserror:
library の公開エラーを保守したいか。

anyhow:
application 上位で文脈つきに失敗を返したいか。
```

## 完了条件

Rust を「全て理解した」と言える状態は、次を満たす状態です。

```text
所有、借用、ライフタイムを API で説明できる
型と enum で状態を表現できる
Result と panic の境界を説明できる
trait と generics を必要な場所だけ導入できる
Iterator と closure を所有権と結びつけて読める
Cargo workspace と crate 公開境界を説明できる
thread と async の選択理由を説明できる
unsafe を避ける理由、使う条件を説明できる
外部クレートの採用理由と保守責任を説明できる
```

# kvs_std

標準ライブラリだけで作る KVS の完成演習です。

このプロジェクトは、`levels/` の小さい例を Cargo project として統合します。目的は高機能サーバーを作ることではなく、所有、借用、失敗、TTL、WAL、テストの責任を 1 つの crate に落とし込むことです。

## 実行

```bash
cargo run -p kvs_std -- SET name Rust
cargo run -p kvs_std -- GET missing
```

この CLI は 1 回の起動で 1 コマンドだけ処理します。起動間で状態は保持しません。状態を長く保持するサーバー化は、最終課題で行います。

## テスト

```bash
cargo test -p kvs_std
```

## 見るべき点

```text
Command は wire text を型に変換したもの。
Store は key/value と TTL を所有する。
Response は表示前の結果で、CLI 表示とは分ける。
WAL は状態変更だけを記録する。
```

## コードを読む順番

```text
Command:
入力を型に変換した結果。

Response:
Store の結果。表示文字列とは分ける。

ParseError:
入力が正しくない理由。

Store:
HashMap、value、TTL を所有する境界。

parse_command:
&str を借りて、Command を所有型として返す境界。

tests:
所有、TTL、WAL の判断が壊れていないか確認する場所。
```

より詳しい読み方は [../PROJECT_WALKTHROUGH.md](../PROJECT_WALKTHROUGH.md) を参照してください。

## std-only の限界

標準ライブラリだけでも、所有権、TTL、WAL、テストは学べます。一方で、実務 CLI、JSON、構造化ログ、非同期 TCP、堅牢 HTTP は外部クレートを検討する領域です。次の `kvs_ecosystem` は、その採用判断を学ぶための対比です。

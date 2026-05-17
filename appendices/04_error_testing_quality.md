# 04: エラー、テスト、品質

## 目的

Rust はメモリ安全性を強く守りますが、仕様の正しさ、運用上の安全性、ユーザー体験までは自動で保証しません。`Result`、テスト、静的解析、レビューを組み合わせます。

## `Result` と `panic!`

```text
Result:
呼び出し側が回復、表示、分類できる失敗。

panic:
プログラムの前提が壊れた、またはテストで即座に失敗させたい場面。
```

I/O、ネットワーク、入力、設定、WAL は自然に失敗します。ここで `unwrap()` を使うと、復旧や説明の道が消えます。

## エラー型を設計する

学習初期:

```rust
fn parse(input: &str) -> Result<Command, String> {
    todo!()
}
```

実務寄り:

```rust
#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    Empty,
    UnknownCommand(String),
    MissingKey,
}
```

エラーを enum にすると、テストと表示を分けられます。文字列は最後にユーザーへ見せる層で作ります。

## `From` でエラーをつなぐ

```rust
enum AppError {
    Io(std::io::Error),
    Parse(ParseError),
}
```

`From` を実装すると `?` で自然に変換できます。実務では `thiserror` や `anyhow` を採用する判断もあります。

## テストの種類

```text
unit test:
関数や型の小さい振る舞いを確認する。

integration test:
外側から crate を使い、複数部品を確認する。

property test:
大量の入力に対して性質を確認する。

fuzz test:
壊れた入力で panic や未定義動作を探す。
```

## 品質ゲート

最終課題では、少なくとも次を通します。

```bash
cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets
```

さらに unsafe や低レイヤー処理では Miri、fuzzing、sanitizer を検討します。

## レビュー観点

```text
Err を String 化しすぎて分類不能になっていないか
panic する場所に根拠があるか
正常な None と失敗の Err を混同していないか
テスト名が仕様を表しているか
境界値と不正入力を試しているか
```

## 進級チェック

```text
Result と panic の使い分けを説明できるか
String エラーから enum エラーへ移るタイミングを説明できるか
単体テストと統合テストの役割を説明できるか
clippy や fmt を品質ゲートとして扱えるか
```

## 公式 docs

- https://doc.rust-lang.org/book/ch09-00-error-handling.html
- https://doc.rust-lang.org/book/ch11-00-testing.html
- https://doc.rust-lang.org/clippy/

## 次に読む

- 前へ: [appendices/03_iterators_patterns_macros.md](03_iterators_patterns_macros.md)
- 次へ: [appendices/05_cargo_ecosystem.md](05_cargo_ecosystem.md)
- 関連: [appendices/README.md](README.md), [docs/guide/LEARNING_PATH.md](../docs/guide/LEARNING_PATH.md)

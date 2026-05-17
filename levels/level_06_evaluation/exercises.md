# Level 6 Exercises

## 1. 小さく変更する

`parser_tests.rs` に `EXISTS key` のパースとテストを追加してください。

正常系と異常系の両方を書きます。

## 2. 出力を比較する

意図的に `parse_get_command` の期待値を間違え、テスト失敗時の出力を読んでください。

その後、期待値を戻してテストが通ることを確認します。

## 3. 設計判断を書く

`parse_command` のエラー型を `String` にする設計と、独自 `enum ParseError` にする設計を比較してください。

`enum ParseError` は、失敗の種類を `MissingKey` や `UnknownCommand` のような名前で分ける設計です。Level 6 では、まず `String` のままで十分か、種類を分ける必要が出てきたかを判断します。

今の段階ではどちらを選ぶか、将来どのタイミングで変えるかを書いてください。

## 提出物

```text
1. EXISTS のテスト
2. テスト失敗時に読んだメッセージの要約
3. String エラーと enum エラーの判断
```

## 進級チェック

```text
失敗する入力を先に考えてテストを書けるか？
失敗を分類し、どの層で扱うべきか説明できるか？
```

## 学習記録

[CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md) の Level 6 を A/B/C で自己評価してください。

[STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md) には、次を書きます。

```text
正常系として確認したこと
異常系として確認したこと
テストを書くために分けた責任
```

## 追加演習: 品質ゲートを書く

最終課題へ進む前に実行する品質ゲートを設計してください。

```bash
cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets
```

それぞれが何を守り、何を守らないかを書いてください。

## 次に読む

- 前へ: [levels/level_06_evaluation/README.md](README.md)
- 次へ: [levels/level_07_integration/README.md](../level_07_integration/README.md)
- 関連: [docs/guide/CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md), [docs/guide/STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md), [solutions/levels_05_09.md](../../solutions/levels_05_09.md)

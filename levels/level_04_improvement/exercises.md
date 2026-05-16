# Level 4 Exercises

## 1. 小さく変更する

`mini_grep.rs` を変更し、大文字小文字を区別しない検索モードを追加してください。

標準ライブラリだけで進めるため、まずは `to_lowercase()` を使って構いません。

## 2. 出力を比較する

次の 3 つを比較してください。

```bash
/tmp/zero_to_rust_grep Rust levels/level_04_improvement/data/search.txt
/tmp/zero_to_rust_grep rust levels/level_04_improvement/data/search.txt
/tmp/zero_to_rust_grep missing levels/level_04_improvement/data/search.txt
```

一致件数、エラーの有無、終了コードについて観察してください。

## 3. 設計判断を書く

`search_file` の中で直接 `println!` する設計と、`Vec<MatchLine>` を返す設計を比較してください。

テスト、将来の UI、エラー処理の観点で判断を書きます。

## 提出物

```text
1. 大文字小文字を区別しない検索の実装方針
2. 3 種類の検索結果の比較
3. 検索結果をデータとして返す理由
```

## 進級チェック

```text
正常な 0 件と失敗を区別できるか？
I/O、検索、表示を分けて説明できるか？
```

## 学習記録

[CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md) の Level 4 を A/B/C で自己評価してください。

[STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md) には、次を書きます。

```text
検索処理が所有する値
Iterator で流すだけの値
表示責任を検索責任から分ける理由
```

## 追加演習: Iterator で読み直す

`mini_grep.rs` の検索処理を、`for` ループ版と iterator chain 版で比較してください。

```text
読みやすさ
所有権と借用の見え方
エラーを返す場所
テストしやすさ
```

短く書けることより、処理の責任が読めるかを優先して判断します。

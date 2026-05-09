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

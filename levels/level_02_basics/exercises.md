# Level 2 Exercises

## 1. 小さく変更する

`mini_cat.rs` を変更し、引数がないときに次のメッセージを表示して終了するようにしてください。

```text
usage: mini_cat <path>
```

## 2. 出力を比較する

存在するファイル、存在しないファイル、ディレクトリを指定した場合の動きを比較してください。

```bash
/tmp/zero_to_rust_cat levels/level_02_basics/data/message.txt
/tmp/zero_to_rust_cat levels/level_02_basics/data/missing.txt
/tmp/zero_to_rust_cat levels/level_02_basics
```

## 3. 設計判断を書く

次の 2 つの設計を比較し、どちらを選ぶか書いてください。

```rust
fn load(path: &str) -> Result<String, std::io::Error>
fn load(path: &str) -> Result<String, String>
```

## 提出物

```text
1. 引数なしの場合の実行結果
2. 3 種類の入力で観察したエラーの違い
3. io::Error を保つか String に変換するかの判断
```

## 進級チェック

```text
失敗しうる処理を Result で表す理由を説明できるか？
? と unwrap の違いを、実行時の振る舞いで説明できるか？
```

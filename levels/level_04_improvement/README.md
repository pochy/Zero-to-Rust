# Level 4: I/O と検索処理の改善

## この Level でできるようになること

`std::fs`、`std::path::Path`、行単位処理を使い、`grep` 風の検索 CLI を作れるようになります。

この Level の中心は、素朴な実装の限界を見つけ、責務を分離して改善することです。

## まず知るべき言葉

- `Path`: ファイルパスを OS に合わせて扱う借用型。
- `PathBuf`: 所有するファイルパス。
- 行単位処理: テキストを `lines()` で 1 行ずつ見る処理。
- 検索結果: 表示文字列ではなく、データとして保持できる結果。
- バイナリファイル: UTF-8 テキストとして読めない可能性があるファイル。
- 部分失敗: あるファイルだけ失敗しても、全体処理は続ける設計。

## なぜこれを学ぶのか

Level 2 の `cat` は、1 つのファイルを読むだけでした。実用的な CLI では、複数ファイル、ディレクトリ、権限エラー、UTF-8 エラーを扱う必要があります。

悪い設計では、探索、読み込み、検索、表示、エラー処理を 1 つの関数に詰め込みます。

```text
引数を読む
ファイルを読む
検索する
println! する
失敗したら panic する
```

良い設計では、少なくとも次を分けます。

```text
入力を解釈する
ファイルを読む
検索結果を作る
結果を表示する
失敗を分類する
```

## 手順 1: 検索 CLI を実行する

```bash
rustc --edition=2021 levels/level_04_improvement/examples/mini_grep.rs -o /tmp/zero_to_rust_grep
/tmp/zero_to_rust_grep Rust levels/level_04_improvement/data/search.txt
```

期待する出力:

```text
levels/level_04_improvement/data/search.txt:1: Rust makes ownership explicit.
levels/level_04_improvement/data/search.txt:3: Rust code should make failure visible.
```

見るべき点は、検索処理が `MatchLine` を作り、表示処理が後で整形していることです。

## 手順 2: 見つからないパターンを試す

```bash
/tmp/zero_to_rust_grep missing levels/level_04_improvement/data/search.txt
```

何も表示されません。これはエラーではありません。

```text
検索結果が 0 件:
正常な結果。

ファイルが読めない:
I/O エラー。

引数が足りない:
使い方のエラー。
```

この区別が、後の API やサーバー設計で重要になります。

## 手順 3: 表示と検索を分ける

`mini_grep.rs` では、検索結果を次の構造体で表します。

```rust
struct MatchLine {
    path: PathBuf,
    line_number: usize,
    line: String,
}
```

`println!` を検索処理の中に直接書くと、テストや再利用が難しくなります。検索結果をデータとして返すと、CLI 表示、JSON 風表示、テストの比較などに使えます。

## よくあるつまずき

```text
Q. パスは String で扱えばよいですか？
A. OS ごとの違いがあるため、ファイルパスには Path や PathBuf を使います。
```

```text
Q. 見つからない場合はエラーですか？
A. いいえ。検索結果 0 件は正常です。ファイルが読めないこととは分けて扱います。
```

```text
Q. すべて read_to_string でよいですか？
A. 小さなテキストでは十分です。巨大ファイルやバイナリ混在では、BufRead など別の設計を検討します。
```

## 次の Level に進む条件

```text
検索結果 0 件と I/O エラーを区別できる
Path と String の使い分けを説明できる
検索処理と表示処理を分ける理由を説明できる
素朴な read_to_string の限界を説明できる
```

## 公式 docs で確認する箇所

- std::fs: https://doc.rust-lang.org/std/fs/
- std::path: https://doc.rust-lang.org/std/path/
- std::io::BufRead: https://doc.rust-lang.org/std/io/trait.BufRead.html

## Rust らしさをさらに深掘りする

検索 CLI は、Iterator、借用、部分失敗、表示分離をまとめて学べる題材です。`Vec<MatchLine>` を返す設計は、検索結果を表示から独立させ、テスト可能なデータに変えます。

追加で読む箇所:

- [Iterator、pattern、macro](../../appendices/03_iterators_patterns_macros.md)
- [エラー、テスト、品質](../../appendices/04_error_testing_quality.md)

次の問いを追加で考えてください。

```text
read_to_string はどの規模や入力で限界になるか
Path と PathBuf の違いは所有権でどう説明できるか
検索結果 0 件を Result::Err にしない理由は何か
```

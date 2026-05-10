# Sol 041: 砂の中の検索グリッド

対応教材: [Level 4: I/O と検索処理の改善](../levels/level_04_improvement/README.md)

## 状況

四十一日目。

通信アンテナの部品が砂に埋もれている。HAB のログには、最後に見た位置、部品番号、風向き、ローバーの走行記録が残っている。

問題は、ログが多すぎることだ。

「人類は火星に来た。そして俺は `grep` を作る」

マークは検索 CLI を実行した。

```bash
rustc --edition=2021 levels/level_04_improvement/examples/mini_grep.rs -o /tmp/zero_to_rust_grep
/tmp/zero_to_rust_grep Rust levels/level_04_improvement/data/search.txt
```

期待する出力:

```text
levels/level_04_improvement/data/search.txt:1: Rust makes ownership explicit.
levels/level_04_improvement/data/search.txt:3: Rust code should make failure visible.
```

## ログ

**LOG 041.1**

マーク:

「検索は簡単だ。ファイルを読んで、行を見て、含まれていたら表示する。そう思ったなら、砂嵐の予報を一行で済ませるタイプだ」

検索処理には境界がある。

```text
引数解析
探索対象の決定
ファイル読み込み
検索
表示
失敗処理
```

マーク:

「これを全部混ぜると、検索結果をテストできない。表示を変えるだけで検索が壊れる。火星では、表示形式の変更で酸素供給が止まる設計を『やめろ』と呼ぶ」

彼は `MatchLine` を眺める。

```rust
struct MatchLine {
    path: PathBuf,
    line_number: usize,
    line: String,
}
```

マーク:

「検索結果は文字列ではない。データだ。パス、行番号、行の内容。この形で持てば、表示もテストも後で変えられる」

## 会話

管制:

「一致しない場合はエラーか？」

マーク:

「違う。見つからなかっただけだ。火星でアンテナを探して見つからないのは悲しいが、ファイルが読めないのとは違う」

管制:

「権限がないファイルがあったら？」

マーク:

「そのファイルだけ失敗として扱う。全体探索を止めるかどうかは設計判断だ。砂地の一マスが危険だからといって、全火星の探索をやめる必要はない」

管制:

「パスは `String` でよいか？」

マーク:

「OS に喧嘩を売りたいならよい。普通は `Path` と `PathBuf` を使う」

## Rust 任務

1. `mini_grep.rs` を実行する。
2. `Rust`、`rust`、`missing` で出力を比較する。
3. 大文字小文字を区別しない検索を追加する。
4. `println!` を検索処理の中に置く設計と、`Vec<MatchLine>` を返す設計を比較する。

演習:

- [Level 4 exercises](../levels/level_04_improvement/exercises.md)

## マークの独り言

「今日の罠は、正常な 0 件と失敗を混同することだ」

```text
検索結果 0 件:
正常。そこにはない。

ファイルが読めない:
失敗。原因を記録する。

引数が足りない:
使い方のエラー。利用者に返す。
```

マーク:

「これを区別しないシステムは、火星では役に立たない。地球でもたぶん迷惑だ」

## 進級チェック

次の問いに答えられたら、Sol 068 へ進む。

```text
検索、表示、I/O を分ける理由を説明できるか。
正常な 0 件と I/O エラーを区別できるか。
`Path` と `PathBuf` を使う理由を説明できるか。
部分失敗を全体失敗にするかどうかの判断軸を持てるか。
```

マークのメモ:

「アンテナ部品はまだ見つからない。だが、見つからないことと、探せないことの違いは分かった」


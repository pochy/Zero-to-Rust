# Sol 012: エアロックログを読む

対応教材: [Level 2: 型、制御構文、エラー処理](../levels/level_02_basics/README.md)

## 状況

十二日目。

エアロックの圧力ログに異常がある。マークはログファイルを読む必要がある。だが、火星のファイルシステムも、地球のファイルシステムも、開発者の都合では動かない。

ファイルは存在しないかもしれない。権限がないかもしれない。壊れているかもしれない。

「つまり、`unwrap()` を使うと俺が爆発する」

彼は `mini_cat` を開いた。

```bash
rustc --edition=2021 levels/level_02_basics/examples/mini_cat.rs -o /tmp/zero_to_rust_cat
/tmp/zero_to_rust_cat levels/level_02_basics/data/message.txt
```

期待する出力:

```text
Rust is a language for explicit responsibility.
Errors are part of the design.
```

## ログ

**LOG 012.1**

マーク:

「ファイルを読む。簡単に聞こえる。火星で『簡単』という単語を使った人間は、たいてい次の十分で後悔する」

コードはこうだ。

```rust
fn main() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    print!("{}", content);
    Ok(())
}
```

マーク:

「`read_to_string` は失敗する。ここが重要だ。失敗するものは、失敗すると書く。これが `Result` だ」

彼は存在しないファイルを指定した。

```bash
/tmp/zero_to_rust_cat levels/level_02_basics/data/missing.txt
```

エラーが出る。プログラムは panic ではなく、失敗として終了する。

マーク:

「美しい。いや、美しいと言うほどではない。だが、HAB の警告灯よりはだいぶ親切だ」

## 会話

管制:

「`unwrap()` の方が短いのでは？」

マーク:

「短い。火星服の酸素ホースを短く切るくらい短い」

管制:

「それはまずい」

マーク:

「そういうことだ。I/O は失敗する。失敗を型に入れる。`?` は『ここでは直せないから上に返す』という合図だ」

管制:

「では、すべてのエラーを `String` に変換してよいか？」

マーク:

「だめだ。圧力低下も、電源断も、ファイルなしも、全部『なんか壊れた』にすると復旧できない。`io::Error` が持っている情報は捨てるな」

## Rust 任務

1. `mini_cat.rs` を実行する。
2. 存在するファイル、存在しないファイル、ディレクトリを渡して動きを比較する。
3. `?` を `unwrap()` に変えた場合の振る舞いを観察する。
4. `Result<String, io::Error>` と `Result<String, String>` の違いを説明する。

演習:

- [Level 2 exercises](../levels/level_02_basics/exercises.md)

## マークの独り言

「火星で重要なのは、失敗しないことではない。失敗が起きたときに、どこで何が起きたか分かることだ」

```text
ファイルがない:
復旧できる。パスを直す。

権限がない:
環境を直す。

UTF-8 ではない:
読み方を変える。

全部 String:
祈る。
```

マーク:

「祈りは設計ではない。少なくとも、Rust コンパイラはそう思っている」

## 進級チェック

次の問いに答えられたら、Sol 025 へ進む。

```text
I/O が失敗する理由を 3 つ挙げられるか。
`?` が失敗をどこへ渡すか説明できるか。
`unwrap()` を使ってよい条件を説明できるか。
エラー型を String に潰すリスクを説明できるか。
```

マークのメモ:

「今日の教訓。ファイルは開けないことがある。エアロックも開かないことがある。どちらも、理由をログに残せ」


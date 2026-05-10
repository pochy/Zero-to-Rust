# 06: 災厄後のテストと智慧之王

対応教材: [Level 6: 品質改善・評価編](../levels/level_06_evaluation/README.md)

## 物語パート

テンペストの通信網は動いた。

少なくとも、昨日まではそう思っていた。

ある朝、倉庫の台帳に異常が出た。`SET food dried_meat` は通る。`GET food` も返る。だが、空白を含む値、存在しないキー、不正な命令を投げると、通信塔が黙り込むことがあった。

「動いたんじゃなかったのか」

『告。動いたという観測は、仕様を満たす証明ではありません』

「大賢者、手厳しい」

『告。進化条件を満たしました』

頭の奥で、冷たい音がした。

『解。大賢者は智慧之王へ進化しました。以後、解析、設計レビュー、失敗分類、標準ライブラリ参照を統合して支援します』

「え、今？」

『肯定。品質改善段階では、単なる知識提示では不十分です。失敗を分類し、再現し、検証する必要があります』

その日、テンペストでは災厄対応会議が開かれた。

リグルドが記録を読み上げる。

「不正な命令で応答がありません」

ゴブタが手を上げる。

「`SET name` って途中まで送ったら、なんか変になったっす」

倉庫番が続ける。

「存在しないキーを消したとき、成功なのか失敗なのか分かりません」

俺は深く息を吐いた。スライムだから息はないが、気分としては吐いた。

「テストを書こう」

## 会話・独白パート

智慧之王が、淡々とコードを示す。

```rust
#[derive(Debug, PartialEq)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Quit,
}

fn parse_command(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();

    match parts.as_slice() {
        ["SET", key, value] => Ok(Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        }),
        ["GET", key] => Ok(Command::Get {
            key: key.to_string(),
        }),
        ["DEL", key] => Ok(Command::Delete {
            key: key.to_string(),
        }),
        ["QUIT"] => Ok(Command::Quit),
        _ => Err(format!("invalid command: {}", input.trim())),
    }
}
```

「`PartialEq` は？」

『解。テストで値同士を比較するために必要です』

「`splitn(3, ' ')` は、値に空白が入る場合のためか」

『肯定。`SET key value with spaces` を、最大 3 要素に分割します』

ゴブタが首をひねる。

「テストって、疑うってことっすか？」

「そうだな。だけど仲間を疑うんじゃない。未来の事故を疑う」

『提案。まず正常系、次に不正入力、境界値、状態エラーを分類してください』

「災厄対策みたいだな」

『肯定。品質改善は災厄の分類から始まります』

## 智慧之王による解説

『告。Level 6 の目的は、「動いた」を検証可能な仕様に変えることです』

テストは、コードが期待通りに動くことを確認するだけではありません。仕様の境界を明確にする作業です。

失敗は分類します。

```text
入力エラー: 不正なコマンド
I/O エラー: 接続断、ファイル読み込み失敗
状態エラー: 存在しないキー
並行処理エラー: ロック競合
設計エラー: 巨大な main.rs
運用エラー: ログ不足
```

『解。分類できない失敗は、改善できません』

`#[test]` を付けた関数は、`cargo test` で実行されるテストになります。`assert_eq!` は、左右の値が等しいことを確認します。

```rust
#[test]
fn parse_set_command() {
    let command = parse_command("SET name rust").unwrap();

    assert_eq!(
        command,
        Command::Set {
            key: "name".to_string(),
            value: "rust".to_string()
        }
    );
}
```

ここでの `unwrap()` は、テストの前提として成功を期待しているため許容できます。失敗した場合はテストが落ち、問題が明確になります。

## Rust 任務

読むもの:

- [parser_tests.rs](../levels/level_06_evaluation/examples/parser_tests.rs)
- [Level 6 exercises](../levels/level_06_evaluation/exercises.md)

考えること:

```text
正常な SET を parse できるか。
値に空白がある SET を parse できるか。
不正コマンドが Err になるか。
存在しないキーはエラーなのか、NOT_FOUND という正常応答なのか。
panic すべき失敗と Result で返す失敗を分けられるか。
```

## 初出用語・関数の説明

- `#[test]`: テスト関数であることを示す属性。
- `cargo test`: Rust のテストをコンパイルして実行する Cargo コマンド。
- `assert_eq!`: 2 つの値が等しいことを確認するマクロ。
- `PartialEq`: 値同士の等価比較を可能にする trait。
- `Debug`: テスト失敗時などに値を表示しやすくする trait。
- `splitn`: 文字列を指定回数まで分割するメソッド。
- `trim`: 文字列の前後の空白や改行を取り除くメソッド。
- `match`: 値の形に応じて処理を分岐する構文。
- 単体テスト: 小さな関数やモジュール単位のテスト。
- 統合テスト: 複数の部品を組み合わせた動作を確認するテスト。
- 失敗分類: エラーを原因や責務ごとに分け、対応方針を決めること。

## 進級チェック

次の問いに答えられたら、次章へ進む。

```text
「動いた」と「テストで確認した」の違いを説明できるか。
入力エラー、I/O エラー、状態エラーを分類できるか。
テスト内の unwrap が許される場面を説明できるか。
panic と Result の使い分けを説明できるか。
```

会議の最後、俺は通信塔のログを見た。

昨日まで「動いた」と呼んでいたものは、まだ粗い試作品だった。

『告。認識を更新しました。次は、複数の処理を同時に扱う段階です』

「国が大きくなってきたってことか」

『解。はい。単独処理では限界です』

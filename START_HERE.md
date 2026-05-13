# START HERE

このファイルは、今日なにから始めればよいかを示す入口です。Rust が完全に初めてでも、順番に進めれば Level 9、補講、Cargo project の実務演習までつながるように構成しています。

## 今日やること

最初の日は、次の 4 つだけで十分です。

```text
1. Rust のコマンドが使えるか確認する
2. Level 0 で Rust の設計思想を読む
3. Level 1 の最小プログラムを実行する
4. 「所有する」と「借りる」の違いを自分の言葉で書く
```

書く場所に迷う場合は [STUDY_JOURNAL.md](STUDY_JOURNAL.md) のテンプレートを使ってください。

## 環境確認

```bash
rustc --version
cargo --version
```

どちらもバージョンが表示されれば、このチュートリアルを進められます。例は標準ライブラリだけで動くようにしているため、外部クレートの追加は不要です。

## 最初の実行

```bash
rustc --edition=2021 levels/level_01_intro/examples/hello_ownership.rs -o /tmp/zero_to_rust_hello
/tmp/zero_to_rust_hello
```

期待する出力:

```text
borrowed: Rust
owned: Rust
```

見るべき点は、同じ文字列を表示していても、関数の受け取り方が違うことです。`borrow_name(&name)` は借りるだけです。`take_name(name)` は `String` の所有権を受け取ります。

## 学習の流れ

各 Level では、次の順番を守ってください。

```text
具体的に動かす
基本用語を押さえる
メンタルモデルを作る
小さな例を実行する
出力を観察する
演習で少し変える
進級チェックで説明する
公式 docs で確認する
appendices で必要な深掘りを読む
CHECKPOINTS.md で進級判断する
STUDY_JOURNAL.md に迷った点を書く
```

Rust は「なんとなく動いた」を許しにくい言語です。その代わり、曖昧な所有、失敗、共有状態を早い段階で見つけてくれます。

## Level 別の進め方

| Level | 何をするか | 次に進む条件 |
| --- | --- | --- |
| 0 | Rust の哲学と所有権キャンバスを書く | 所有、借用、解放、失敗を 1 つの処理で説明できる |
| 1 | Hello Rust と所有権の最小例を動かす | `String` と `&str` の違いを説明できる |
| 2 | `cat` 風 CLI を動かす | `Result` と `?` の役割を説明できる |
| 3 | `HashMap` KVS を作る | 構造体がデータを所有する意味を説明できる |
| 4 | `grep` 風検索を作る | 探索、検索、表示を分ける理由を説明できる |
| 5 | TCP KVS の最小ワークフローを理解する | プロトコルと store を分ける理由を説明できる |
| 6 | テストと失敗分類を行う | 入力エラー、I/O エラー、状態エラーを分類できる |
| 7 | スレッドプールと共有状態を扱う | `Arc<Mutex<T>>` の責務と危険を説明できる |
| 8 | WAL、設定、ログ、復旧を設計する | 障害時の復旧手順を説明できる |
| 9 | バイナリ処理と最終課題に取り組む | std-only と外部クレート利用の判断を説明できる |

## 補講とプロジェクトの進め方

Level 0-9 は本編です。本編を進める途中で、次のように補講を読みます。

| タイミング | 読むもの |
| --- | --- |
| Level 1-3 で所有権が曖昧なとき | `appendices/01_ownership_lifetimes.md` |
| Level 3-5 で抽象化に迷うとき | `appendices/02_traits_generics.md` |
| Level 4-6 で処理の書き方を広げたいとき | `appendices/03_iterators_patterns_macros.md` |
| Level 6 以降で品質を上げたいとき | `appendices/04_error_testing_quality.md` |
| Level 8-9 で実務構成へ進むとき | `appendices/05_cargo_ecosystem.md` 以降 |

Level 9 まで終えたら、次の順番で Cargo project を動かします。

```bash
cargo test -p kvs_std
cargo test -p kvs_ecosystem
cargo test -p final_kvs_server
```

`kvs_std` は標準ライブラリだけで責任を抱える練習です。`kvs_ecosystem` は、同じ責任の一部を成熟した crate に任せる判断を学ぶ練習です。`final_kvs_server` は、TCP、TTL、WAL、admin HTTP、metrics を 1 つに統合する最終成果物です。

最終課題へ入る前に、[FINAL_PROJECT_SPEC.md](FINAL_PROJECT_SPEC.md) と [REVIEW_CHECKLIST.md](REVIEW_CHECKLIST.md) を読んでください。

最後に [ASSESSMENT.md](ASSESSMENT.md) の口頭試問へ答え、Rust の判断を自分の言葉で説明できるか確認します。

演習に取り組んだ後は、[solutions](solutions/README.md) で回答例と比較してください。先に読まず、自分の判断を書いてから読む方が効果的です。

各 Level の終わりでは [CHECKPOINTS.md](CHECKPOINTS.md) で A/B/C の自己評価をしてください。A は次へ進んでよい状態、B は迷った点を記録しながら進める状態、C は同じ Level の例をもう一度変更する状態です。

## 目安時間

| 範囲 | 目安 |
| --- | --- |
| Level 0-1 | 半日から 1 日 |
| Level 2-4 | 3 日から 1 週間 |
| Level 5-6 | 1 週間 |
| Level 7-8 | 1 週間から 2 週間 |
| Level 9 と最終課題 | 1 週間以上 |
| 補講と projects | 2 週間以上 |

急ぐより、各 Level の進級チェックを言語化することを優先してください。

## 1 周目と 2 周目

1 周目ですべてを理解しようとしなくて構いません。1 周目は、次を優先します。

```text
例を動かす
コンパイルエラーを読む
所有、借用、失敗を短く説明する
CHECKPOINTS.md で B 以上なら次へ進む
```

2 周目では、同じ例を「API 設計」として読み直します。

```text
なぜこの関数は String ではなく &str を受け取るのか
なぜこの失敗は panic ではなく Result なのか
なぜこの状態は共有され、別の状態は共有されないのか
なぜここで外部 crate を採用する、または採用しないのか
```

Rust を学ぶ意味は、最初から完璧に書くことではありません。曖昧な責任を、あとから説明できる設計へ直していくことです。

## 公式 docs の使い方

公式 docs は、最初に読む百科事典ではなく、手元の体験を確認する一次情報として使います。

```text
まずローカルで小さく動かす
出力を見る
なぜそうなるか考える
公式 docs で用語と仕様を確認する
```

この順番にすると、docs が暗記対象ではなく、設計判断の根拠になります。

## よくあるトラブル

`rustc: command not found` が出る場合は、Rust がインストールされていません。Rust 公式の rustup を使ってインストールしてください。

`edition 2021` が分からない場合は、いったん気にしなくて構いません。このチュートリアルの例を、幅広いローカル環境で検証しやすくするための指定です。

コンパイルエラーが出た場合は、エラーメッセージを最初から最後まで読んでください。Rust のエラーは、単なる失敗通知ではなく、設計のどこが曖昧かを教える教材です。

`unwrap()` を使いたくなった場合は、まず「この失敗は本当に起きないと言えるか」を書いてください。言えないなら `Result` で返す設計を選びます。

`cargo test -p kvs_ecosystem` で依存クレートの取得に失敗する場合は、ネットワークに接続できる環境で再実行してください。本編の `levels/` は外部クレートなしで進められるため、先に本編を完走できます。

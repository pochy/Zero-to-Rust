# Rust 学習リソース整理

Rust の参考書、Web チュートリアル、YouTube 動画、設計思想・哲学を学ぶための資料を目的別に整理したメモです。

基本方針は、**公式教材で基礎を固める → 手を動かす → 実務寄りの本やプロジェクトへ進む → 設計思想・RFC・unsafe 境界を深掘りする**、という順番です。

## まず最初に選ぶなら

最初の軸はこの 3 つです。

| リソース | 種別 | 向いている人 |
| --- | --- | --- |
| [The Rust Programming Language / The Book][book] | 公式入門書 | Rust を体系的に理解したい人 |
| [Rustlings][rustlings] | 公式系演習 | 手を動かして構文・所有権・型・エラー処理に慣れたい人 |
| [Rust By Example][rust-by-example] | 公式サンプル集 | 実行可能なコード例を見ながら学びたい人 |

The Book は、所有権、借用、ライフタイム、`Result`、`Option`、トレイト、ジェネリクス、並行性など、Rust の思想を正面から学べる公式入門書です。英語版は更新が続いているため、日本語版を読む場合も最終確認は英語版で行うと安心です。

Rustlings は、小さな未完成コードを直しながら Rust に慣れる演習教材です。The Book と並行して進めると、コンパイラのエラーメッセージを読みながら理解を定着させやすくなります。

Rust By Example は、概念や標準ライブラリを実行可能なサンプルで確認したいときに便利です。読む教材というより、逆引き・補強用として使いやすい資料です。

## Web チュートリアル・無料教材

| リソース | 概要 | 使いどころ |
| --- | --- | --- |
| [The Rust Programming Language][book] | Rust 公式の入門書。印刷版は No Starch Press から出版 | 最初に通読する軸 |
| [The Rust Programming Language 日本語版][book-ja] | The Book の日本語版 | 日本語で基礎を固めたいとき |
| [Rustlings][rustlings] | CLI 上で進める公式系演習 | The Book と並行して手を動かす |
| [Rust By Example][rust-by-example] | 公式のコード例集 | 構文・標準ライブラリの補強 |
| [Comprehensive Rust][comprehensive-rust] | Google の無料 Rust 講座 | 短期集中で全体像を俯瞰したいとき |
| Rust 日本語ドキュメント | 日本語の公式系ドキュメント群 | 日本語で公式資料を読みたいとき |
| Tour of Rust / Rust ツアー | ブラウザ上でコードを編集・実行できる入門 | 環境構築前に基本を試したいとき |
| [100 Exercises to Learn Rust][100-exercises] | Luca Palmieri による演習教材 | 実践演習を重ねたいとき |
| JetBrains Academy の Rust コース | オンライン学習コース | IDE や演習環境込みで学びたいとき |
| Rust Exercise 各種 | 練習問題集 | Rustlings 後の反復練習 |
| [rust-lang.org/learn][rust-learn] | Rust 公式学習ページ | 公式リソースの起点 |

Comprehensive Rust は、Google の Android チームが作った無料講座です。基本構文、ジェネリクス、エラー処理、並行性、Android、Chromium、bare-metal まで扱います。The Book より講義資料に近く、全体像を短期間で掴む用途に向いています。

## 参考書・書籍

### 初中級者向け

| 書籍 | 概要 | 向いている人 |
| --- | --- | --- |
| [Rust の練習帳 / Command-Line Rust][command-line-rust] | `cat` や `head` のような CLI ツールを Rust で作る本 | Rust で実用的な小物を作りたい人 |
| 『動かして学ぶ！Rust 入門』 | Zenn の人気コンテンツをベースにしたハンズオン入門 | 実際に動くアプリケーションを作りながら学びたい人 |
| 『実践 Rust プログラミング入門 第 2 版』（自転車本） | 日本語の実践的な Rust 入門書 | 日本語で実践寄りに学びたい人 |
| 『詳解 Rust プログラミング』 | Rust を詳しく扱う日本語書籍 | 仕様や実装を丁寧に確認したい人 |

Command-Line Rust は、テスト、エラー処理、ファイル処理、正規表現など実用的な題材が多く、フロントエンドエンジニアが「Rust で何か作る」入り口としても相性が良いです。

### 深く学ぶ定番書

| 書籍 | 概要 | 向いている人 |
| --- | --- | --- |
| [プログラミング Rust 第 2 版 / Programming Rust, 2nd Edition][programming-rust-ja] | Rust 2021 対応。低レイヤ、メモリ、安全性、並行性まで詳しい定番書 | 他言語経験があり、Rust を深く学びたい人 |
| [Rust in Action][rust-in-action] | DB、OS カーネル、CPU エミュレータなどを作るプロジェクト型の本 | システムプログラミング寄りに学びたい人 |
| [Rust for Rustaceans][rust-for-rustaceans] | 所有権、trait、並行性、unsafe、型レイアウト、coherence、async、`no_std` などを深掘り | 基礎後に Rust らしい設計を学びたい人 |
| [Effective Rust][effective-rust] | idiomatic Rust の考え方を整理した本。日本語版もあり | The Book 後に設計の質を上げたい人 |
| [Rust Atomics and Locks][atomics-locks] | アトミック操作、メモリオーダリング、ロックを深く扱う | 並行・並列処理を本気で学びたい人 |

Programming Rust は、Rust の「言語仕様の決定版」として評価が高い本です。C/C++ 並みの性能と低レベル制御、メモリ安全性、スレッド安全性、並行性といった Rust の特徴を広く扱います。

Rust for Rustaceans は、文法入門ではなく「シニア Rust エンジニアがどう思考しているか」に近い内容です。大きめのコードベース、ライブラリ設計、型、トレイト、非同期、unsafe の考え方を学べます。

Effective Rust は、The Book を読んだあとに「自分の Rust コードが C++ / TypeScript 風 Rust になっていないか」を確認するのに向いています。

### Web バックエンド・実務寄り

| 書籍 | 概要 | 向いている人 |
| --- | --- | --- |
| [Zero To Production In Rust][zero2prod] | ニュースレター API を作りながら、テスト、DB、認証、設定、デプロイを学ぶ | Rust で Web API を作りたい人 |
| 『Rust による Web アプリケーション開発 設計からリリース・運用まで』 | Tokio や Axum などを使った Web 開発、設計、テスト、運用を扱う | 実務でバックエンドを構築したい人 |

Zero To Production In Rust は、`axum` や `actix-web` 方面に進みたい人に向いています。The Book 読了後の次のステップとして評価が高い実践書です。

### 設計思想・コンセプト重視

| 書籍 | 概要 | 向いている人 |
| --- | --- | --- |
| 『コンセプトから理解する Rust』 | 所有権、借用、ライフタイム、型システム、トレイトを「なぜその仕様か」から説明 | Rust 独自の考え方を腑に落としたい人 |
| [Rust for Rustaceans][rust-for-rustaceans] | idiomatic Rust と中上級概念を深掘り | Rust らしい設計思想を知りたい人 |
| [Programming Rust 第 2 版][programming-rust-ja] | メモリモデルや並行性の背景まで扱う | 言語仕様の裏側を知りたい人 |

設計思想を日本語で学ぶ目的なら、『コンセプトから理解する Rust』は特に相性が良いです。他言語との比較を交えながら、Rust の急峻な学習曲線の理由を整理できます。

## YouTube 動画・チャンネル

| チャンネル / シリーズ | 概要 | 向いている人 |
| --- | --- | --- |
| [Let's Get Rusty][lets-get-rusty] | The Book の章ごとの解説、機能まとめ、キャリア・実務寄りの話題 | 初心者から中級者 |
| [Jon Gjengset / Crust of Rust][crust-of-rust] | ライフタイム、trait、iterator、channel、smart pointer、`serde` などを深掘り | 中級者から上級者 |
| [No Boilerplate][no-boilerplate] | 短めの動画で Rust の思想や魅力を紹介 | Rust の考え方を掴みたい人 |
| [Rust 公式チャンネル / RustConf][rustconf-2025] | RustConf や公式発表 | コミュニティや実践事例を追いたい人 |
| [Rust Tutorial Full Course][rust-full-course] | 基本文法、データ構造、所有権、エラー処理、並行処理を 1 本で概観 | 全体を一気に俯瞰したい人 |
| fasterthanlime | 低レベル、ツールチェーン、async、所有権などをストーリー性のある記事・動画で解説 | ディープな背景を知りたい人 |
| Tensor Programming | Rust 関連の解説動画 | 補助教材を増やしたい人 |
| freeCodeCamp / Derek Banas のフルコース動画 | 長尺の入門動画 | 動画で一気に概要を掴みたい人 |
| Steve Klabnik「The History of Rust」 | Rust 1.0 までの変化や設計判断の歴史を解説 | Rust の歴史を時間軸で理解したい人 |
| The Untold Story Of Rust | 個人プロジェクトからコミュニティ駆動言語への移行を扱う動画 | Rust の背景を物語として知りたい人 |

Let's Get Rusty は初心者向けに見やすく、The Book と組み合わせる動画教材としておすすめです。

Crust of Rust は、関数の使い方を覚える動画ではなく、「なぜこの型になるのか」「なぜこのライフタイムが必要なのか」「標準ライブラリはどう設計されているのか」を学ぶ動画です。初心者がいきなり見ると難しいですが、Rust の理解を一段上げる教材として非常に有用です。

## 設計思想・哲学・背景を学ぶ資料

文法よりも、Rust が「なぜこの仕様になったのか」「コンパイラはどう世界を捉えているのか」を知りたい場合は、以下の資料が向いています。

### 公式・一次情報

| リソース | 学べること |
| --- | --- |
| [The Book: Introduction][book-intro] | Rust が何を解決しようとしている言語か |
| The Book: Ownership / References and Borrowing / Lifetimes | なぜ GC なしで安全にしたいのか、なぜ借用を厳密に扱うのか |
| The Book: Struct / Enum / Pattern Matching | なぜデータ構造を型で表現するのか |
| The Book: Traits | なぜ継承より trait なのか |
| The Book: Error Handling | なぜ例外より `Result` なのか |
| The Book: Fearless Concurrency | なぜスレッド安全性を型で保証したいのか |
| [Rust 公式ブログ: Abstraction without overhead: traits in Rust][traits-blog] | GC なしの安全性、データ競合なしの並行性、ゼロコスト抽象化、trait の設計思想 |
| [Rust RFC Book][rfcs] / rust-lang/rfcs GitHub リポジトリ | 機能追加・仕様変更の動機、代替案、欠点、設計判断 |
| [Rust API Guidelines][api-guidelines] | Rust らしい公開 API、型設計、利用者体験。日本語訳も参照候補 |
| [Rust Design Patterns][design-patterns] | idiom、design pattern、anti-pattern |
| [The Rustonomicon][nomicon] | unsafe Rust、安全性の境界、未定義動作。日本語訳も参照候補 |
| [Polonius][polonius] / Niko Matsakis の Borrow Checker 関連資料 | 所有権・借用チェッカーの再設計や理論的背景 |
| [withoutboats: Ownership][withoutboats-ownership] | 所有権を型システムの拡張として捉える視点 |

Rust は、**C/C++ 級の低レベル制御を保ちながら、メモリ安全性・スレッド安全性・高水準な抽象化を実現する**方向を目指す言語です。

特に重要なのは、所有権と借用が単なる構文ではなく、Rust 設計の中心だという点です。所有権を「メモリ管理機能」としてだけ見るより、**型システムを使って値が何回・どこで・どう使われるかを制約する仕組み**として見ると理解が深まります。

The Rustonomicon を読むと、Rust が「unsafe を完全になくす言語」ではなく、**unsafe を局所化して、safe な世界を広く保つ言語**であることが分かります。Rust の安全性は魔法ではなく、危険な部分を `unsafe` として明示し、その上に安全な抽象化を作る設計です。

### 読むと面白い RFC

| RFC | 学べる思想 |
| --- | --- |
| [RFC 0241 Deref conversions][rfc-0241] | 所有権・借用をなぜ見える形にするのか |
| RFC 2094 Non-Lexical Lifetimes | 借用チェッカーをなぜ柔軟にしたのか |
| RFC 0445 Extension Trait Conventions | trait による拡張設計 |
| RFC 0255 Object Safety | trait object の制約と設計 |
| RFC 243 Trait-based Exception Handling | 失敗した案も含め、なぜ Rust が例外ではなく `Result` 文化なのかを考える材料 |
| Rust RFC "North Star" | Rust の言語進化の方向性 |

RFC には、仕様だけでなく「なぜ必要か」「他にどんな代替案があったか」「どんな欠点があるか」が書かれます。Rust の思想を追うには、公式ドキュメント以上に生々しい資料です。

### API 設計で見る Rust らしさ

Rust API Guidelines では、以下のような観点が整理されています。

| 観点 | Rust らしい考え方 |
| --- | --- |
| 型安全性 | `bool` や `Option` だけで曖昧にせず、意味のある型を作る |
| 予測可能性 | API 利用者が驚かない挙動にする |
| 将来性 | 将来の変更で壊れにくい API にする |
| エラー設計 | `Result`、専用エラー型、ドキュメントを重視する |
| trait 設計 | どこまで抽象化するか、object safe にするかを考える |

TypeScript でいうと、単に関数を書くのではなく、公開 API・型設計・利用者体験をどう設計するかに近い資料です。

### 歴史・背景・コミュニティ

| リソース / テーマ | 概要 |
| --- | --- |
| Mozilla と Servo、C++ へのアンチテーゼ | 次世代ブラウザエンジン Servo の文脈で、巨大な C++ 製品の安全性・並行性問題に向き合った背景 |
| The Early History of Rust | Graydon Hoare 氏のブログやスピーチを探すとよい検索ワード |
| Graydon Hoare 氏の古いプレゼン資料 | `venge.net/graydon/talks/` など |
| MIT Technology Review「How Rust went from a side project...」 | エレベーターのソフトウェアクラッシュをきっかけとする有名なエピソードを含む記事 |
| Rust Foundation の 10 周年寄稿 | Graydon Hoare 氏による振り返り |
| ZDNet Japan「エレベーター故障から始まったシステムプログラミング言語の歴史」 | Rust 誕生の背景を日本語で読める記事 |
| Technology Review 日本語版「世界で最も愛されるプログラミング言語 Rust 誕生秘話」 | Graydon の動機、Mozilla の関与、コミュニティ駆動への移行を日本語で読める記事 |
| 日本語 Wikipedia「Rust (プログラミング言語)」 | 開発史、Graydon Hoare の役割、命名由来などの概要 |
| Rust Internals Forum | 言語仕様の設計者やコア開発者の議論 |
| Aaron Turon「Rust's language ergonomics initiative」 | 2017 年の ergonomics initiative。使いやすさを重視する言語設計の方向性 |
| Rust 公式ブログの古いエントリ | Roadmap 2017 など、当時の設計方針 |
| Reddit の議論スレッド | “What is the overall design philosophy of Rust” など、コミュニティの解釈 |
| Rust.Tokyo 関連スライド / 登大遊氏などのプレゼン資料 | 日本語で設計判断の背景を知る補助資料 |
| Zenn / Qiita の記事 | 「Rust 設計思想」「Rust 所有権 なぜ」「Rust 哲学」などで検索 |

Rust の主な哲学として、以下を意識すると資料が読みやすくなります。

| キーワード | 意味 |
| --- | --- |
| Safety without GC | GC なしでメモリ安全性を実現する |
| Performance | C/C++ 級の性能と低レベル制御を保つ |
| Productivity | 高水準な抽象化と開発体験を重視する |
| Zero-cost abstractions | 抽象化しても実行時コストを増やさない |
| Fearless concurrency | 並行処理の安全性を型システムで支える |
| Empirical iteration | 実践で検証しながら言語を進化させる |
| Ergonomics | 制約を人間が扱いやすくする |

## Rust を哲学的に理解するための問い

| 問い | 対応する Rust の思想 |
| --- | --- |
| なぜ GC を使わずに安全にしたいのか | 所有権・RAII・ゼロコスト |
| なぜ `&` と `&mut` を分けるのか | aliasing XOR mutation |
| なぜ null ではなく `Option` なのか | 不正状態を型で表す |
| なぜ例外ではなく `Result` なのか | 失敗を型で明示する |
| なぜ継承ではなく trait なのか | 合成・抽象化・静的ディスパッチ |
| なぜ `unsafe` があるのか | 危険を局所化して安全な抽象を作る |
| なぜコンパイルが厳しいのか | 実行時ではなくコンパイル時に壊す |
| なぜ RFC 文化が強いのか | 言語進化を設計判断として記録する |

## 学習ルート例

### 王道ルート

1. [The Book][book] の 1〜10 章あたりを読む
2. [Rustlings][rustlings] を並行して進める
3. [Rust By Example][rust-by-example] で分からない構文を補強する
4. 小さい CLI を作る
5. [Rust の練習帳 / Command-Line Rust][command-line-rust] に進む
6. Web API を作りたいなら [Zero To Production][zero2prod] に進む
7. Rust らしい設計を学ぶなら [Rust for Rustaceans][rust-for-rustaceans] / [Effective Rust][effective-rust] を読む
8. 深掘りしたくなったら [Crust of Rust][crust-of-rust] を見る

### 設計思想重視ルート

1. [The Book の Introduction][book-intro] を読む
2. Ownership / Borrowing / Lifetimes を読む
3. [Rust 公式ブログ “Abstraction without overhead: traits in Rust”][traits-blog] を読む
4. [Rust API Guidelines][api-guidelines] を読む
5. [Rust Design Patterns][design-patterns] の Idioms / Anti-patterns を読む
6. RFC 0241、RFC 2094 などを読む
7. [Rust for Rustaceans][rust-for-rustaceans] を読む
8. [Rustonomicon][nomicon] を必要な範囲で読む
9. [Crust of Rust][crust-of-rust] を見る

普通の入門ルートが「文法 → 関数 → 構造体 → enum → trait」だとすると、思想重視ルートは「安全性 → 所有権 → 借用 → 型による設計 → trait による抽象化 → unsafe の局所化 → RFC による言語進化」という順番です。

### 日本語リソース中心ルート

1. ZDNet Japan / Technology Review 日本語版の記事で歴史・背景を読む
2. [The Book 日本語版][book-ja] の所有権章を「なぜこの仕組みか？」という視点で読む
3. 『コンセプトから理解する Rust』を通読する
4. 『プログラミング Rust 第 2 版』や中上級者向け書籍へ進む

## 目的別の最初の一冊・教材

| 目的 | おすすめ |
| --- | --- |
| 無料で始める | The Book + Rustlings |
| 日本語でじっくり学ぶ | プログラミング Rust 第 2 版 |
| 手を動かして学ぶ | Rust の練習帳 / Command-Line Rust |
| Web バックエンドを作る | Zero To Production In Rust |
| Rust の設計思想を学ぶ | コンセプトから理解する Rust / Rust for Rustaceans |
| 並行処理を深く学ぶ | Rust Atomics and Locks |
| unsafe と安全性の境界を知る | The Rustonomicon |

[book]: https://doc.rust-lang.org/book/ "The Rust Programming Language"
[book-ja]: https://doc.rust-jp.rs/book-ja/ "The Rust Programming Language 日本語版"
[book-intro]: https://doc.rust-lang.org/book/ch00-00-introduction.html "Introduction - The Rust Programming Language"
[rustlings]: https://github.com/rust-lang/rustlings/ "rust-lang/rustlings"
[rust-by-example]: https://doc.rust-lang.org/rust-by-example/ "Rust By Example"
[comprehensive-rust]: https://github.com/google/comprehensive-rust "google/comprehensive-rust"
[rust-learn]: https://www.rust-lang.org/learn/ "Learn Rust"
[100-exercises]: https://rust-exercises.com/100-exercises/ "100 Exercises to Learn Rust"
[command-line-rust]: https://www.oreilly.com/library/view/command-line-rust/9781098109424/ "Command-Line Rust"
[programming-rust-ja]: https://www.oreilly.co.jp/books/9784873119786/ "プログラミングRust 第2版"
[programming-rust-en]: https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ "Programming Rust, 2nd Edition"
[rust-in-action]: https://www.rustinaction.com/ "Rust in Action"
[zero2prod]: https://www.zero2prod.com/ "Zero To Production In Rust"
[rust-for-rustaceans]: https://nostarch.com/rust-rustaceans "Rust for Rustaceans"
[effective-rust]: https://www.oreilly.co.jp/books/9784814400942/ "Effective Rust"
[atomics-locks]: https://mara.nl/atomics/ "Rust Atomics and Locks"
[lets-get-rusty]: https://www.youtube.com/@letsgetrusty "Let's Get Rusty"
[crust-of-rust]: https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa "Crust of Rust"
[no-boilerplate]: https://www.youtube.com/c/NoBoilerplate "No Boilerplate"
[rustconf-2025]: https://www.youtube.com/playlist?list=PL2b0df3jKKiRFEuVNk76ufXagOgEJ9sBZ "RustConf 2025"
[rust-full-course]: https://www.youtube.com/watch?v=ygL_xcavzQ4 "Rust Tutorial Full Course"
[traits-blog]: https://blog.rust-lang.org/2015/05/11/traits/ "Abstraction without overhead: traits in Rust"
[rfcs]: https://rust-lang.github.io/rfcs/ "The Rust RFC Book"
[rfc-0241]: https://rust-lang.github.io/rfcs/0241-deref-conversions.html "RFC 0241 Deref conversions"
[api-guidelines]: https://rust-lang.github.io/api-guidelines/about.html "Rust API Guidelines"
[design-patterns]: https://rust-unofficial.github.io/patterns/ "Rust Design Patterns"
[nomicon]: https://doc.rust-lang.org/nomicon/ "The Rustonomicon"
[polonius]: https://rust-lang.github.io/polonius/ "Polonius"
[withoutboats-ownership]: https://without.boats/blog/ownership/ "Ownership"

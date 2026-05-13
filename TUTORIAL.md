# Rust 標準ライブラリ中心チュートリアル

このファイルは、Zero to Rust の設計意図と全体像を説明する文書です。実際に学習を始めるときは [START_HERE.md](START_HERE.md)、進行表は [LEARNING_PATH.md](LEARNING_PATH.md)、各 Level 後の自己判定は [CHECKPOINTS.md](CHECKPOINTS.md)、判断の記録は [STUDY_JOURNAL.md](STUDY_JOURNAL.md) を使ってください。

このチュートリアルは、いただいた「単なる使い方ではなく、その技術で何を設計し、何を判断できるようになるべきか」を中心にする構成テンプレートをベースにしています。
テーマは **Rust を外部クレートに頼らず、`std` 中心で深く学ぶ** ことに絞ります。

## この文書の使い方

`TUTORIAL.md` は、教材全体の設計思想をまとめた長い背景文書です。毎日の学習で最初から最後まで通読する必要はありません。

実際の学習では、次を正本として使います。

| 目的 | 正本 |
| --- | --- |
| 今日なにをするか | [START_HERE.md](START_HERE.md) |
| 目的別に場所を探す | [INDEX.md](INDEX.md) |
| Level ごとの学習 | `levels/level_*/README.md` と `levels/level_*/exercises.md` |
| 進級判断 | [CHECKPOINTS.md](CHECKPOINTS.md) |
| 学習記録 | [STUDY_JOURNAL.md](STUDY_JOURNAL.md) |
| Rust 全体の補講 | [appendices/README.md](appendices/README.md) |
| Cargo project の読み方 | [projects/PROJECT_WALKTHROUGH.md](projects/PROJECT_WALKTHROUGH.md) |
| 最終課題 | [FINAL_PROJECT_SPEC.md](FINAL_PROJECT_SPEC.md) |

この文書は、次のタイミングで読むと役に立ちます。

```text
Level 0 の前:
Rust をなぜ学ぶのか、何を判断できるようになるべきかを読む。

Level 3-5 の途中:
小さい CLI からアプリケーション境界へ進む理由を確認する。

Level 8-9 の前:
運用、WAL、並行処理、std-only の限界を俯瞰する。

完走後:
自分が説明できるようになった判断を振り返る。
```

## 重複の扱い

この文書には、現在の `levels/`、`appendices/`、`projects/` と重なる説明があります。これは削除せず、設計背景として残しています。

学習中に内容が重複して見える場合は、次のように扱ってください。

```text
TUTORIAL.md:
なぜその Level があるのか、設計思想を確認する場所。

levels/:
実際に手を動かす場所。

appendices/:
詰まった概念を深掘りする場所。

projects/:
実務に近い crate 構成で責任境界を読む場所。
```

矛盾して見える場合は、より具体的なファイルを優先します。たとえば、Level の手順は `levels/level_*/README.md`、最終課題の仕様は [FINAL_PROJECT_SPEC.md](FINAL_PROJECT_SPEC.md)、実装の読み方は [projects/PROJECT_WALKTHROUGH.md](projects/PROJECT_WALKTHROUGH.md) を優先してください。

---

## 全体方針

このチュートリアルの目的は、Rustの文法を覚えることではありません。

最終的な目的は、以下を自分で判断できるようになることです。

```text
このデータは誰が所有するのか
↓
誰が一時的に借りるのか
↓
いつ解放されるのか
↓
スレッド間で共有してよいのか
↓
失敗したときにどう回復するのか
↓
長く保守できる構造になっているか
```

Rustの中心概念である所有権は、メモリ管理をコンパイラが検査するための規則であり、Rustはガベージコレクションでも手動解放でもなく、所有権システムによってメモリ安全性を実現します。([Rust ドキュメント][1])

また、このチュートリアルでは外部クレートを原則使いません。`serde`、`tokio`、`clap`、`anyhow`、`regex`、`rayon` などを便利に使う前に、まず `std` だけで次の力を鍛えます。

* `String` / `&str` / `Vec<T>` / `HashMap<K, V>` の使い分け
* `Option<T>` / `Result<T, E>` による失敗表現
* `Box<T>` / `Rc<T>` / `Arc<T>` / `Mutex<T>` / `RwLock<T>` の設計判断
* `std::fs` / `std::io` / `std::net` / `std::process` / `std::thread` の実践
* 所有権、借用、ライフタイムを避けずに設計する力

Rust標準ライブラリは、`Vec<T>`、`Option<T>`、I/O、マルチスレッドなどを含む、Rustソフトウェアの基礎となる共有抽象を提供しています。([Rust ドキュメント][2])

---

# 0. まず学ぶべき設計思想・哲学

## 0-1. Rustの本質

### 悪い理解

> Rustは速くて安全なプログラミング言語である。

これは間違いではありませんが、浅い理解です。

### 良い理解

> Rustは、「誰がデータを所有し、誰が一時的に借り、いつ破棄されるか」を型システムで表現することで、メモリ安全性・並行安全性・低レイヤー制御を両立しようとする言語である。

Rustは、単にC/C++の代替ではありません。

Rustは、以下のような問いをコードに強制的に埋め込ませる言語です。

```rust
fn read_config(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```

この短い関数にも、Rustらしい設計思想が入っています。

* `path: &str`
  呼び出し元の文字列を借りるだけで、所有しない。

* `Result<String, std::io::Error>`
  成功すれば読み込んだ文字列の所有権を返す。失敗すればエラーを返す。

* `String`
  ファイル内容はこの関数内で生成されるため、呼び出し元に所有権を渡す。

Rustでは「なんとなく参照」「なんとなくコピー」ではなく、データの責任範囲を明示します。

---

## 0-2. なぜRustが必要なのか

```text
従来の課題
↓
C/C++では高速で低レイヤー制御ができるが、use-after-free、二重解放、データ競合などを人間が防ぐ必要があった。

この技術が提供する考え方
↓
所有権、借用、ライフタイム、型システムによって、危険なメモリ操作や並行アクセスをコンパイル時に検出する。

解決できること
↓
高速性、メモリ安全性、並行安全性、低レイヤー制御を同時に狙える。

新しく発生する設計上の注意点
↓
データの所有者、参照の寿命、共有状態、エラー型、モジュール境界を曖昧にしたコードはコンパイルできない。
```

Rustは「書きにくい言語」ではなく、「曖昧な設計を早い段階で拒否する言語」と捉えると理解しやすくなります。

---

## 0-3. Rustで最も重要な設計判断

Rustで重要なのは、APIや構文を暗記することではなく、以下を判断できることです。

| 判断     | 悪い考え方                     | 良い考え方                                  |
| ------ | ------------------------- | -------------------------------------- |
| 所有権    | コンパイルが通らないから `clone()` する | 本当に所有権を複製すべきか考える                       |
| 借用     | とりあえず `&` を付ける            | 呼び出し側が所有し続けるべきか考える                     |
| ライフタイム | 難しいから避ける                  | 参照がどこまで有効かを設計として表す                     |
| エラー    | `unwrap()` で済ませる          | 失敗を呼び出し元に返すか、ここで処理するか決める               |
| 共有状態   | グローバル変数的に持つ               | `Arc<Mutex<T>>` や `RwLock<T>` の責務を限定する |
| 並行処理   | スレッドを増やせば速い               | ロック範囲、デッドロック、終了処理を設計する                 |
| 抽象化    | 最初からtraitだらけにする           | 変更される軸だけ抽象化する                          |

---

# チュートリアル全体構成

このチュートリアルでは、最終的に次のような成果物を作れる状態を目指します。

```text
標準ライブラリだけで作る
マルチスレッド対応 KVS / HTTP 管理サーバー / 永続化 / テスト / ログ / 運用設計
```

外部クレートなしでも、Rustの本質的な力は十分に学べます。

---

# Level 1: 入門編 — Rustに触る

## 目的

Rustの開発環境、最小プログラム、所有権の直感を理解します。

現在のCargoでは、`cargo new` によって生成されるマニフェストはデフォルトでRust 2024 editionを使うと公式ドキュメントに記載されています。([Rust ドキュメント][3])

## 学ぶこと

* `cargo new`
* `cargo run`
* `main.rs`
* `let`
* `mut`
* `String`
* `Vec<T>`
* `Option<T>`
* `Result<T, E>`
* 所有権の最初の感覚

## 実装内容

### Hello Rust

```bash
cargo new rust_std_tutorial
cd rust_std_tutorial
cargo run
```

```rust
fn main() {
    println!("Hello, Rust!");
}
```

### 所有権の最小例

```rust
fn main() {
    let name = String::from("Rust");
    print_name(name);

    // ここで name はもう使えない
    // println!("{}", name);
}

fn print_name(value: String) {
    println!("{}", value);
}
```

`String` はヒープ上にデータを持つ所有型です。`String` の公式ドキュメントでも、UTF-8でエンコードされた伸長可能な文字列であり、内容を所有する型として説明されています。([Rust ドキュメント][4])

## 設計思想

最初に覚えるべきことは、「値を渡す」と「値を貸す」は違うということです。

```rust
fn take(s: String) {
    println!("{}", s);
}

fn borrow(s: &str) {
    println!("{}", s);
}
```

* `String` を受け取る関数は、所有権を受け取る。
* `&str` を受け取る関数は、文字列を借りる。

## よくある落とし穴

```rust
fn main() {
    let s = String::from("hello");
    let t = s;
    println!("{}", s); // エラー
}
```

初心者は「代入しただけ」と思いがちですが、Rustでは所有権が移動します。

## 到達目標

* `cargo new` / `cargo run` でプロジェクトを作れる
* `String` と `&str` の違いを説明できる
* 所有権の移動によって値が使えなくなることを理解できる

---

# Level 2: 基礎実装編 — 型、制御構文、エラー処理

## 目的

Rustの日常的な基本構文を使い、小さなCLIプログラムを作れるようにします。

## 学ぶこと

* `struct`
* `enum`
* `match`
* `impl`
* `Result`
* `Option`
* `std::env`
* `std::fs`
* `std::io`
* `?` 演算子

`std::io` は入力・出力に関する基本機能を提供し、中心には `Read` と `Write` trait があります。([Rust ドキュメント][5])

## 実装内容

### 簡易 `cat` コマンド

```rust
use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let path = env::args()
        .nth(1)
        .expect("usage: cargo run -- <path>");

    let content = fs::read_to_string(path)?;
    print!("{}", content);

    Ok(())
}
```

## 設計思想

Rustでは、失敗する可能性がある処理は `Result` で表現します。

悪い例：

```rust
let content = std::fs::read_to_string(path).unwrap();
```

良い例：

```rust
let content = std::fs::read_to_string(path)?;
```

`unwrap()` は学習初期には便利ですが、実務コードでは「この失敗は起きない」という根拠がある場合に限定します。

## よくある落とし穴

### エラーを文字列で雑に扱う

```rust
fn load(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}
```

これは一見簡単ですが、エラーの種類が失われます。

より良い設計：

```rust
use std::io;

fn load(path: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}
```

## 到達目標

* 小さなCLIを作れる
* `Result` と `?` を使える
* エラーを握りつぶさずに呼び出し元へ返せる

---

# Level 3: データ設計・構造設計編

## 目的

Rustでデータ構造を設計し、責務ごとにモジュールを分けられるようにします。

## 学ぶこと

* `struct` による状態表現
* `enum` による状態・コマンド・エラー表現
* `HashMap`
* モジュール分割
* `String` と `&str` のAPI設計
* テストしやすい関数分割

Rustの `std::collections` は、一般的なデータ構造の効率的な実装を提供しています。([Rust ドキュメント][6])

## 実装内容

### インメモリKVSのコア部分

```rust
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}
```

## 設計思想

ここでは、まず `String` を所有させます。

初心者にとって、いきなり `&str` を構造体に持たせる設計は危険です。

悪い例：

```rust
struct Store<'a> {
    data: HashMap<&'a str, &'a str>,
}
```

これはライフタイム設計が難しく、学習初期には本質から外れやすいです。

良い例：

```rust
struct Store {
    data: HashMap<String, String>,
}
```

まずは所有する。その後、APIの引数だけ `&str` にして、不要なコピーを減らします。

## ディレクトリ構成

```text
rust_std_tutorial/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── store.rs
    ├── command.rs
    └── error.rs
```

## よくある落とし穴

### すべてを `main.rs` に書く

小さいうちは動きますが、テストしづらくなります。

悪い構成：

```text
main.rs にパース、保存、表示、エラー処理を全部書く
```

良い構成：

```text
main.rs      起動と入出力
store.rs     データ操作
command.rs   コマンド解析
error.rs     エラー定義
```

## 到達目標

* `HashMap` を使った状態管理ができる
* 所有するデータ構造を設計できる
* コアロジックを `main` から分離できる

---

# Level 4: 実践機能編 — CLI検索・置換ツールを作る

## 目的

ファイルI/O、文字列処理、再帰処理、エラー処理を組み合わせて実用的なCLIを作ります。

## 学ぶこと

* `std::fs`
* `std::path::Path`
* `std::path::PathBuf`
* 再帰的ディレクトリ探索
* 行単位処理
* ANSIエスケープによる簡易カラー表示
* 書き込み時の安全性

`std::fs` はローカルファイルシステムを操作する基本機能を提供します。([Rust ドキュメント][7])

## 実装内容

### 簡易grep

```rust
use std::fs;
use std::io;
use std::path::Path;

fn search_file(path: &Path, pattern: &str) -> Result<(), io::Error> {
    let content = fs::read_to_string(path)?;

    for (index, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            println!("{}:{}: {}", path.display(), index + 1, line);
        }
    }

    Ok(())
}
```

### 再帰探索の疑似コード

```text
search_dir(path)
    if path is file
        search_file(path)
    if path is directory
        for child in directory
            search_dir(child)
```

## 設計思想

この段階では、次の分離が重要です。

```text
引数解析
↓
探索対象の決定
↓
ファイル読み込み
↓
検索
↓
表示
↓
必要なら置換
```

悪い設計は、検索処理の中で `println!`、ファイル書き込み、引数解析をすべて混ぜることです。

良い設計では、検索結果をデータとして返します。

```rust
#[derive(Debug)]
struct MatchLine {
    path: std::path::PathBuf,
    line_number: usize,
    line: String,
}
```

## よくある落とし穴

* バイナリファイルをUTF-8として読んで失敗する
* 権限がないファイルで全体処理が止まる
* 置換時に元ファイルを壊す
* パスを単なる `String` として扱う
* 再帰探索でシンボリックリンクを考慮しない

## 到達目標

* ファイル処理と文字列処理を組み合わせられる
* `Path` / `PathBuf` を使える
* 失敗したファイルだけスキップし、全体処理を継続できる

---

# Level 5: アプリケーション編 — TCP KVSサーバーを作る

## 目的

`std::net` を使って、TCP経由でコマンドを受け取るインメモリKVSを作ります。

`std::net` では、`TcpListener` / `TcpStream` によるTCP通信、`UdpSocket` によるUDP通信などが提供されています。([Rust ドキュメント][8])

## 学ぶこと

* `TcpListener`
* `TcpStream`
* `BufReader`
* `Write`
* 独自プロトコル
* コマンドパース
* `Arc<Mutex<T>>`
* 複数クライアント対応

## 実装内容

### プロトコル仕様

```text
SET key value
GET key
DEL key
EXISTS key
QUIT
```

レスポンス：

```text
OK
VALUE value
NOT_FOUND
ERROR message
BYE
```

### TCPサーバーの最小例

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(stream.try_clone()?);

        let mut line = String::new();
        reader.read_line(&mut line)?;

        stream.write_all(b"OK\n")?;
    }

    Ok(())
}
```

## 設計思想

ネットワークアプリでは、処理を最低でも3層に分けます。

```text
TCP入出力層
↓
プロトコル解析層
↓
KVS操作層
```

悪い例：

```text
TCP接続を受ける関数の中で
文字列をsplitし
HashMapを直接操作し
レスポンス文字列も組み立てる
```

良い例：

```text
handle_client()
  -> parse_command()
  -> store.execute(command)
  -> response.to_wire()
```

## よくある落とし穴

* `read_to_string` で接続が閉じるまで待ってしまう
* 1クライアントが遅いと全体が止まる
* コマンドの不正入力でpanicする
* `Mutex` のロック範囲が広すぎる
* レスポンスの末尾改行を忘れる
* プロトコル仕様が曖昧でテストできない

## 到達目標

* TCPサーバーを起動できる
* 独自プロトコルを定義できる
* ネットワーク層とロジック層を分離できる

---

# Level 6: 品質改善・評価編

## 目的

作ったものを「動いた」で終わらせず、テスト、性能、失敗、保守性の観点で改善します。

Rustでは `cargo test` によってテストをコンパイル・実行でき、単体テストや統合テストを整理して実行できます。([Rust ドキュメント][9])

## 学ぶこと

* 単体テスト
* 統合テスト
* エラー型の設計
* ログ設計
* ベンチマークの考え方
* パニックとエラーの区別
* 負荷テストの設計
* 失敗分類

## 実装内容

### コマンドパーサーのテスト

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
```

## 設計思想

品質改善は、感覚ではなく分類から始めます。

```text
問題が起きる
↓
原因を分類する
↓
測定する
↓
改善案を出す
↓
再評価する
```

### 失敗分類

| 分類      | 例             | 対応              |
| ------- | ------------- | --------------- |
| 入力エラー   | 不正なコマンド       | `ERROR` を返す     |
| I/Oエラー  | 接続断、ファイル読込失敗  | `Result` で伝播    |
| 状態エラー   | 存在しないキー       | `NOT_FOUND` を返す |
| 並行処理エラー | ロック競合         | ロック範囲を短くする      |
| 設計エラー   | 巨大な `main.rs` | モジュール分割         |
| 運用エラー   | ログ不足          | イベントを記録する       |

## よくある落とし穴

* `unwrap()` がテストでは通るが本番で落ちる
* パース処理の境界値をテストしていない
* エラー型を全部 `String` にして後から分類できない
* ロック中にI/Oしてしまう
* 性能問題を測定せずに最適化する

## 到達目標

* コアロジックをテストできる
* 失敗を分類できる
* `panic!` と `Result` の使い分けができる
* 測定してから改善する習慣を持てる

---

# Level 7: アーキテクチャ編 — 中規模・チーム開発に耐える設計

## 目的

学習用コードを、複数人で保守できる構成に発展させます。

## 学ぶこと

* レイヤー分割
* traitによる抽象化
* dependency management
* `Arc`
* `Mutex`
* `RwLock`
* スレッドプール
* graceful shutdown
* コードレビュー観点

`std::sync` には、複数スレッド間でデータの寿命を延ばす `Arc` や、排他制御のための同期プリミティブが含まれます。([Rust ドキュメント][10])
`Mutex` は共有データを保護し、ロック中にだけデータへアクセスできるようにする仕組みです。([Rust ドキュメント][11])
`RwLock` は複数の読み取り、または単一の書き込みを許可するロックです。([Rust ドキュメント][12])

## 実装内容

### 推奨構成

```text
src/
├── main.rs
├── server/
│   ├── mod.rs
│   ├── tcp.rs
│   ├── client.rs
│   └── thread_pool.rs
├── protocol/
│   ├── mod.rs
│   ├── command.rs
│   └── response.rs
├── store/
│   ├── mod.rs
│   ├── memory.rs
│   └── ttl.rs
├── persist/
│   ├── mod.rs
│   └── wal.rs
└── error.rs
```

### スレッドプールの概念

```text
main thread
  |
  | accepts TCP connections
  v
job queue
  |
  +--> worker 1
  +--> worker 2
  +--> worker 3
  +--> worker 4
```

### スレッドプールの骨格

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let receiver = Arc::clone(&receiver);

            let handle = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            });

            workers.push(handle);
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), mpsc::SendError<Job>>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f))
    }
}
```

## 設計思想

アーキテクチャは「きれいに見せるため」ではありません。

目的は、変更に耐えることです。

```text
プロトコルを変えてもStoreは壊れない
Storeを永続化してもTCP層は変わらない
スレッド数を変えてもコマンド処理は変わらない
テスト時にネットワークを使わなくてよい
```

## よくある落とし穴

* traitを早く入れすぎてコードが複雑になる
* `Arc<Mutex<Store>>` をどこにでも渡してしまう
* ロック中にクライアントへレスポンスを書いてしまう
* スレッド終了処理を考えていない
* `JoinHandle` を保持せず、ワーカー管理ができない
* パニックしたジョブが全体設計を壊す

## 到達目標

* レイヤーごとに責務を分けられる
* スレッドプールを自作できる
* 共有状態の扱いを設計できる
* 中規模コードのレビュー観点を持てる

---

# Level 8: 運用・本番編

## 目的

標準ライブラリだけで作ったシステムを、本番運用を意識した形に近づけます。

## 学ぶこと

* 設定ファイル
* 環境変数
* ログ
* 永続化
* WAL
* graceful shutdown
* バックアップ
* 権限
* 障害復旧
* スケーリング限界
* セキュリティ上の限界

## 実装内容

### 設定

```rust
use std::env;
use std::net::SocketAddr;

#[derive(Debug)]
struct Config {
    addr: SocketAddr,
    worker_count: usize,
    data_path: String,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        let addr = env::var("APP_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:4000".to_string())
            .parse()
            .map_err(|e| format!("invalid APP_ADDR: {}", e))?;

        let worker_count = env::var("APP_WORKERS")
            .unwrap_or_else(|_| "4".to_string())
            .parse()
            .map_err(|e| format!("invalid APP_WORKERS: {}", e))?;

        let data_path = env::var("APP_DATA")
            .unwrap_or_else(|_| "data.wal".to_string());

        Ok(Self {
            addr,
            worker_count,
            data_path,
        })
    }
}
```

### WALの考え方

```text
SET name rust
DEL old_key
SET lang rust
```

起動時：

```text
WALを先頭から読む
↓
各コマンドをStoreに再適用する
↓
最後の状態を復元する
```

## 設計思想

本番運用では、機能よりも次の観点が重要です。

```text
再現性
可観測性
復旧性
安全性
拡張性
コスト
```

### 標準ライブラリのみの限界

| 領域   | stdだけでできること         | stdだけでは厳しいこと            |
| ---- | ------------------- | ----------------------- |
| TCP  | 独自サーバー              | 高性能asyncランタイム           |
| HTTP | 簡易実装                | 完全なHTTP仕様対応             |
| TLS  | 不可に近い               | 実務では外部ライブラリかリバースプロキシが必要 |
| JSON | 自作可能                | 高品質パーサーは大変              |
| ログ   | `println!` / ファイル出力 | 構造化ログ基盤                 |
| 監視   | 簡易メトリクス             | Prometheus連携など          |
| CLI  | `std::env::args`    | 複雑なサブコマンド               |

## よくある落とし穴

* `127.0.0.1` と `0.0.0.0` の違いを理解しない
* ログに機密情報を出す
* 永続化前にレスポンスを返す
* WAL破損時の復旧方針がない
* Ctrl+C終了時の処理を考えていない
* TLSなしでインターネットに直接公開する
* バックアップとリストアを一度も試していない

## 到達目標

* 環境変数で設定を切り替えられる
* WALで状態復元できる
* ログと復旧手順を設計できる
* std-only構成の限界を説明できる

---

# Level 9: プロフェッショナル編 — 高度な設計・最適化・応用

## 目的

Rustの低レイヤー制御、並行処理、バイナリ処理、抽象化を深く理解し、設計判断できるようにします。

## 学ぶこと

* バイナリプロトコル
* DNSリゾルバ
* SHA-256
* JSONパーサー
* Lispインタープリタ
* ハフマン圧縮
* ワークスティーリング
* `Box`
* `Rc`
* `RefCell`
* `Arc`
* `Mutex`
* `Condvar`
* `mpsc`
* `unsafe` の考え方

`Condvar` は、条件が満たされるまでスレッドをCPUを消費せずに待機させる同期プリミティブです。([Rust ドキュメント][13])
また `std::sync::mpsc` はチャンネル通信を提供し、送信側と受信側を分離した並行処理設計に使えます。([Rust ドキュメント][14])

## 実装候補

### 1. DNSリゾルバ

学ぶこと：

* UDP
* バイト列
* エンディアン
* DNSヘッダー
* 可変長パケット
* 圧縮ドメイン名

設計ポイント：

```text
[u8] を直接触る
↓
境界チェックを必ず行う
↓
PacketReaderを作る
↓
構造体へ安全に変換する
```

---

### 2. Lispインタープリタ

学ぶこと：

* 再帰的データ構造
* `enum`
* `Box`
* `Rc<RefCell<T>>`
* 環境
* 評価器

設計ポイント：

```rust
enum Expr {
    Number(i64),
    Symbol(String),
    List(Vec<Expr>),
}
```

悪い設計：

```text
全部Stringとして扱い、評価時に毎回判定する
```

良い設計：

```text
パース時に構文木へ変換し、評価器は型付き構造を処理する
```

---

### 3. SHA-256実装

学ぶこと：

* ビット演算
* rotate
* padding
* big endian
* 固定長配列
* ファイル入力

設計ポイント：

```text
入力を読む
↓
パディングする
↓
512bitブロックに分割する
↓
メッセージスケジュールを作る
↓
圧縮関数を適用する
↓
ハッシュ値を出力する
```

---

### 4. ハフマン圧縮

学ぶこと：

* `BinaryHeap`
* 木構造
* ビット単位I/O
* ファイルヘッダー
* 可逆性テスト

設計ポイント：

```text
頻度表
↓
ハフマン木
↓
符号表
↓
ビット列出力
↓
ヘッダー保存
↓
復元
```

---

### 5. ワークスティーリング・スケジューラ

学ぶこと：

* `Arc`
* `Mutex`
* `Condvar`
* `mpsc`
* パニック隔離
* タスクキュー
* ワーカー管理

設計ポイント：

```text
各ワーカーが自分のキューを持つ
↓
自分のキューが空なら他ワーカーから盗む
↓
全体停止時はCondvarで通知する
```

## 設計思想

プロフェッショナルなRust開発者は、単に難しい機能を知っている人ではありません。

状況に応じて、次を判断できる人です。

```text
ここは所有させるべきか
ここは借用で十分か
ここはcloneしてよいか
ここはArcが必要か
ここはMutexでよいか
ここはRwLockが有効か
ここはstdだけでよいか
ここは外部クレートを使うべきか
ここはunsafeを避けるべきか
ここはunsafeを安全なAPIで包むべきか
```

## よくある落とし穴

* 難しい機能を使うことが目的になる
* `unsafe` を高速化の魔法だと思う
* `Rc<RefCell<T>>` を乱用して設計を曖昧にする
* `Arc<Mutex<T>>` をグローバル状態の代わりにする
* ベンチマークなしで最適化する
* `clone()` を絶対悪と考える
* ライフタイム注釈を増やせば解決すると思う

## 到達目標

* バイナリプロトコルを安全に処理できる
* 並行処理の設計判断ができる
* スマートポインタを使い分けられる
* std-onlyの限界と価値を説明できる
* 外部クレートを使うべき場面も判断できる

---

# 最終課題

## Production-grade 最終課題

## 標準ライブラリだけで作るマルチスレッドKVSサーバー

最終課題では、以下を作ります。

```text
TCPベースのインメモリKVS
+
TTL
+
WAL永続化
+
スレッドプール
+
簡易HTTP管理画面
+
テスト
+
ログ
+
復旧手順
+
運用ドキュメント
```

---

## アプリケーション内容

クライアントはTCPで接続し、次のようなコマンドを送ります。

```text
SET user:1 Alice
GET user:1
DEL user:1
EXPIRE user:1 60
TTL user:1
QUIT
```

サーバーはメモリ上の `HashMap` にデータを保存し、必要に応じてWALに操作ログを書き込みます。

HTTP管理画面では、ブラウザから以下を確認できます。

```text
GET /health
GET /metrics
GET /keys
```

---

## 必須要件

### 基本機能

* `SET`
* `GET`
* `DEL`
* `EXISTS`
* `EXPIRE`
* `TTL`
* `QUIT`

### 実務的な設計

```text
server層      TCP/HTTP接続
protocol層    コマンドとレスポンス
store層       データ管理
persist層     WAL
config層      設定
error層       エラー型
```

### エラー処理

* 不正コマンド
* 存在しないキー
* I/Oエラー
* WAL書き込み失敗
* クライアント切断
* ロック取得失敗
* TTL期限切れ

### テスト

* コマンドパースの単体テスト
* Store操作の単体テスト
* TTLの単体テスト
* WAL復元の単体テスト
* TCP通信の統合テスト
* 複数クライアント同時接続テスト

### 評価指標

| 指標  | 内容              |
| --- | --------------- |
| 正確性 | SETした値がGETできる   |
| 可用性 | 複数クライアント接続で落ちない |
| 復旧性 | WALから状態復元できる    |
| 性能  | 1秒あたりの処理コマンド数   |
| 安定性 | 不正入力でpanicしない   |
| 保守性 | モジュール単位でテストできる  |

### ログ

標準ライブラリだけなら、まずは次の形式で十分です。

```text
timestamp level event client_addr detail
```

例：

```text
2026-05-09T10:00:00Z INFO client_connected 127.0.0.1:53000
2026-05-09T10:00:01Z INFO command SET key=user:1
2026-05-09T10:00:02Z ERROR wal_write_failed reason=permission_denied
```

### セキュリティ

std-onlyの範囲では、次を実装します。

* bind addressを設定可能にする
* 最大コマンド長を制限する
* 最大キー長を制限する
* 最大値サイズを制限する
* 管理HTTPをlocalhost限定にする
* ログに値本体を出さない
* 不正入力でpanicしない

TLSや本格的な認証は、標準ライブラリだけでは実務品質にするのが難しいため、本番想定ではリバースプロキシや外部コンポーネントの利用を前提にします。

### デプロイ

```text
cargo build --release
↓
環境変数を設定
↓
専用ユーザーで起動
↓
WAL保存先を指定
↓
ログ保存先を指定
↓
health checkで確認
```

### ドキュメント

* 起動方法
* コマンド仕様
* エラー仕様
* 設定一覧
* 復旧手順
* バックアップ手順
* 既知の制限
* 将来拡張案

---

## 完成イメージ

```text
ユーザーがTCPで接続
↓
SET user:1 Alice を送信
↓
プロトコル層がCommand::Setへ変換
↓
StoreがHashMapを更新
↓
WALへ追記
↓
OKを返す
↓
ログに記録
↓
/metricsで処理件数を確認
↓
障害時はWALから復元
```

---

# 1ヶ月の推奨進行プラン

## Week 1: Rustの基礎とCLI

作るもの：

* `cat`風CLI
* `grep`風CLI
* KVSのローカルStore

重点：

* 所有権
* 借用
* `Result`
* `HashMap`
* テスト

---

## Week 2: TCP KVS

作るもの：

* 単一スレッドTCPサーバー
* コマンドパーサー
* レスポンス生成

重点：

* `TcpListener`
* `TcpStream`
* `BufReader`
* プロトコル設計
* 不正入力処理

---

## Week 3: 並行処理とTTL

作るもの：

* `Arc<Mutex<Store>>`
* 複数クライアント対応
* TTL
* クリーンアップスレッド

重点：

* 共有状態
* ロック範囲
* デッドロック回避
* スレッド終了設計

---

## Week 4: 永続化・HTTP管理・品質改善

作るもの：

* WAL
* 起動時復元
* `/health`
* `/metrics`
* 統合テスト
* 運用ドキュメント

重点：

* 復旧性
* 可観測性
* エラー分類
* 本番運用の限界理解

---

# 推奨カリキュラム表

| 段階      | 内容      | 作るもの                      | 到達目標                            |
| ------- | ------- | ------------------------- | ------------------------------- |
| Level 1 | 入門      | Hello Rust / 所有権の最小例      | Rustの基本実行と所有権の直感をつかむ            |
| Level 2 | 基礎実装    | `cat`風CLI                 | `Result`、`?`、ファイルI/Oを使える        |
| Level 3 | データ設計   | ローカルKVS                   | `HashMap`、`struct`、`enum`で設計できる |
| Level 4 | 実践機能    | grep/sed風CLI              | 再帰処理、パス処理、エラー処理ができる             |
| Level 5 | アプリ化    | TCP KVS                   | ネットワーク層とロジック層を分離できる             |
| Level 6 | 評価      | テスト・失敗分類                  | 品質を測定し改善できる                     |
| Level 7 | アーキテクチャ | スレッドプールKVS                | 中規模構成と並行処理を設計できる                |
| Level 8 | 運用      | WAL / health / metrics    | 復旧性、可観測性、設定管理を考えられる             |
| Level 9 | プロ      | DNS / SHA-256 / Lisp / 圧縮 | バイナリ処理、木構造、並行設計を応用できる           |

---

# Rust学習で特に重要な悪い例・良い例

## 1. `clone()` の使い方

悪い例：

```rust
let a = value.clone();
let b = value.clone();
let c = value.clone();
```

良い例：

```rust
fn print_value(value: &str) {
    println!("{}", value);
}
```

ただし、`clone()` は悪ではありません。

```text
cloneしてでも所有権を分離した方が設計が明確になる
```

という場面もあります。重要なのは、コンパイルエラーを消すためではなく、設計判断としてcloneすることです。

---

## 2. `unwrap()` の使い方

悪い例：

```rust
let file = std::fs::read_to_string("config.txt").unwrap();
```

良い例：

```rust
let file = std::fs::read_to_string("config.txt")?;
```

許される例：

```rust
let addr = "127.0.0.1:4000".parse::<std::net::SocketAddr>().unwrap();
```

これはリテラルが固定であり、失敗しないことを開発者が保証できる場合です。

---

## 3. `Arc<Mutex<T>>` の使い方

悪い例：

```rust
type SharedEverything = Arc<Mutex<App>>;
```

良い例：

```rust
type SharedStore = Arc<Mutex<Store>>;
```

共有する対象を小さくします。

```text
共有状態は少なく
ロック範囲は短く
I/O中にロックしない
```

---

## 4. ライフタイムの考え方

悪い理解：

> ライフタイムはコンパイルエラーを消すための記号。

良い理解：

> ライフタイムは、参照がどのデータに依存しているかを型に表す仕組み。

学習初期は、構造体に参照を持たせるより、まず所有型を持たせます。

```rust
struct Config {
    host: String,
}
```

慣れてきたら、関数引数で借用を使います。

```rust
fn connect(host: &str) {
    println!("connect to {}", host);
}
```

---

# このテーマで一番大事な考え方

Rustで一番大事なのは、次の問いを避けないことです。

```text
このデータの所有者は誰か？
```

この問いから逃げると、Rustは難しく感じます。

しかし、この問いに向き合うと、Rustは設計を助けてくれる言語になります。

```text
所有権はメモリ管理の仕組み
借用は責務を渡さずに使う仕組み
ライフタイムは参照の有効範囲を表す仕組み
Resultは失敗を設計に含める仕組み
Arc/Mutexは共有と排他を明示する仕組み
```

このチュートリアルの最終目標は、Rustで「動くもの」を作ることではありません。

**誰が何を所有し、どこで失敗し、どのように復旧し、どこまで安全に並行実行できるかを設計できるエンジニアになること**です。

---

# 増補版: Rust 全体を理解するための追加ロードマップ

この教材は、Level 0-9 の本編を残したまま、補講と実務プロジェクトを追加します。

```text
levels/
小さく動かし、Rust の哲学と設計判断を体験する。

appendices/
Rust の全体地図を補完する。trait、async、unsafe、Cargo、性能まで扱う。

projects/
Cargo workspace で実務に近い crate を作る。std-only 版と ecosystem 版を比較する。
```

## 本編と補講の対応

| 本編 | 補講 | 深める判断 |
| --- | --- | --- |
| Level 0-3 | `appendices/01_ownership_lifetimes.md` | 所有型、借用、ライフタイム、clone の判断 |
| Level 3-5 | `appendices/02_traits_generics.md` | trait、generics、dyn trait、抽象化の時期 |
| Level 4-6 | `appendices/03_iterators_patterns_macros.md` | iterator、closure、match、macro の読み方 |
| Level 6 | `appendices/04_error_testing_quality.md` | エラー分類、panic、テスト、品質ゲート |
| Level 7-9 | `appendices/06_async_concurrency.md` | thread、channel、async、Send/Sync |
| Level 9 | `appendices/07_unsafe_ffi_performance.md` | unsafe、FFI、no_std、performance |
| 最終課題 | `appendices/08_professional_rust_map.md` | 実務レビューと採用判断 |

## Cargo project の役割

`projects/kvs_std` は、標準ライブラリだけで KVS を crate としてまとめます。

```text
Command:
wire text を型に変換したもの。

Store:
key/value、TTL、状態を所有する。

Response:
表示前の結果。CLI や TCP とは分ける。

WAL:
状態変更だけを記録し、復旧可能性を設計に入れる。
```

`projects/kvs_ecosystem` は、同じ題材で主要 crate を採用します。

```text
serde:
JSON の parse と serialize を任せる。

clap:
CLI 引数と help を任せる。

thiserror:
分類可能な library error を保つ。

anyhow:
binary の上位で文脈つきエラーを扱う。

tracing:
構造化ログを扱う。

tokio:
async runtime を使う構成に入る。
```

ここで重要なのは、std-only を卒業することではありません。

```text
std で理解する。
crate に任せる理由を書く。
任せた責任と残った責任を分ける。
```

この 3 つを説明できる状態が、実務 Rust への入口です。

`projects/final_kvs_server` は、最終課題の統合実装です。

```text
TCP command server:
複数クライアントから SET/GET/DEL/TTL を受ける。

WAL:
状態変更を追記し、起動時に復旧する。

TTL:
期限切れを読み取り時に片付ける。

admin HTTP:
/health、/metrics、/keys を返す。

shared state:
Arc<Mutex<AppState>> で Store と metrics を守る。
```

この project は、完全な本番サーバーではありません。むしろ、std-only で自分が背負う責任を見える化し、どこから `tokio`、`hyper`、`axum`、`tracing`、`clap`、`serde` に任せるべきかを判断する教材です。

最終的な自己評価には `ASSESSMENT.md` を使います。Rust の理解は、暗記量ではなく、所有、失敗、共有、復旧、抽象化、依存採用をコード参照つきで説明できるかで測ります。

## 追加された完了条件

増補版の完了条件は、次を自分の言葉で説明できることです。

```text
所有権、借用、ライフタイムを API の形で説明できる。
trait と generics を、変更軸が見えた場所で導入できる。
Iterator と closure を、所有権の流れとして読める。
Result、panic、独自エラー、テストを失敗分類として扱える。
Cargo workspace、edition、feature、crate 採用判断を説明できる。
thread と async の違い、Send/Sync、lock と await の危険を説明できる。
unsafe を避ける理由、使う場合の safety 条件を説明できる。
FFI、no_std、性能改善を、責任境界として扱える。
```

この状態になれば、Rust の全 API を暗記していなくても、新しい crate やフレームワークに出会ったときに「何を所有し、何を借り、どこで失敗し、どの責任を外部へ任せているのか」を読めます。それが、このチュートリアルでいう「Rust を理解した」状態です。

[1]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html?utm_source=chatgpt.com "What is Ownership? - The Rust Programming Language"
[2]: https://doc.rust-lang.org/std/?utm_source=chatgpt.com "Crate std - Rust Standard Library"
[3]: https://doc.rust-lang.org/cargo/?search=edition&utm_source=chatgpt.com "Introduction - The Cargo Book"
[4]: https://doc.rust-lang.org/std/string/struct.String.html?utm_source=chatgpt.com "String in std"
[5]: https://doc.rust-lang.org/std/io/index.html?utm_source=chatgpt.com "std::io"
[6]: https://doc.rust-lang.org/std/collections/index.html?utm_source=chatgpt.com "std::collections - Rust"
[7]: https://doc.rust-lang.org/std/fs/?utm_source=chatgpt.com "std::fs"
[8]: https://doc.rust-lang.org/std/net/index.html?utm_source=chatgpt.com "std::net"
[9]: https://doc.rust-lang.org/book/ch11-02-running-tests.html?utm_source=chatgpt.com "Controlling How Tests Are Run - The Rust Programming ..."
[10]: https://doc.rust-lang.org/std/sync/index.html?utm_source=chatgpt.com "std::sync - Rust"
[11]: https://doc.rust-lang.org/std/sync/struct.Mutex.html?utm_source=chatgpt.com "Mutex in std::sync"
[12]: https://doc.rust-lang.org/std/sync/struct.RwLock.html?utm_source=chatgpt.com "RwLock in std::sync"
[13]: https://doc.rust-lang.org/std/sync/struct.Condvar.html?utm_source=chatgpt.com "Condvar in std::sync"
[14]: https://doc.rust-lang.org/std/sync/mpsc/?utm_source=chatgpt.com "Module mpsc - std::sync"

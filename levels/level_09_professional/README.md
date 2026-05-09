# Level 9: プロフェッショナル設計と最終課題

## この Level でできるようになること

バイナリ処理、スマートポインタ、並行処理、std-only の限界を踏まえ、最終課題の設計を自分で説明できるようになります。

この Level の中心は、難しい機能を使うことではありません。状況に応じて、どの抽象、どの所有、どの同期、どの外部依存を選ぶか判断することです。

## まず知るべき言葉

- バイナリプロトコル: テキストではなくバイト列の構造で通信する約束。
- エンディアン: 複数バイトの数値をどの順序で読むかという規則。
- 境界チェック: バイト列を読む前に範囲内か確認すること。
- `Box<T>`: ヒープ上の値を 1 つの所有者が持つスマートポインタ。
- `Rc<T>`: 単一スレッド内で複数所有を許す参照カウント。
- `RefCell<T>`: 実行時に借用規則を検査する型。
- `Condvar`: 条件が満たされるまでスレッドを待たせる同期プリミティブ。
- `unsafe`: コンパイラが一部の安全性を検査できない操作を許す領域。

## なぜこれを学ぶのか

Level 8 までで、KVS、TCP、テスト、並行処理、WAL、運用設計を扱いました。Level 9 では、より低レイヤーな処理と高度な設計判断へ進みます。

プロフェッショナルな Rust 開発者は、難しい型を知っている人ではありません。次を説明できる人です。

```text
ここは所有でよいか、借用でよいか
clone すべきか、参照で十分か
Arc<Mutex<T>> でよいか、別の設計が必要か
std だけでよいか、外部クレートを採用すべきか
unsafe が必要か、不要か
```

## 手順 1: バイト列を安全に読む

```bash
rustc --edition=2021 levels/level_09_professional/examples/packet_reader.rs -o /tmp/zero_to_rust_packet
/tmp/zero_to_rust_packet
```

期待する出力:

```text
id = 4660
flags = 1
payload = rust
short packet rejected
```

見るべき点は、`[u8]` を直接読む前に必ず長さを確認していることです。バイナリ処理では、境界チェックが設計の中心になります。

## 手順 2: 最終課題の構成を設計する

最終課題は、標準ライブラリだけで作るマルチスレッド KVS サーバーです。

```text
server層      TCP/HTTP 接続
protocol層    Command と Response
store層       HashMap、TTL、状態管理
persist層     WAL 追記と復元
config層      環境変数と設定値
error層       エラー分類
```

必須機能:

```text
SET
GET
DEL
EXISTS
EXPIRE
TTL
QUIT
```

管理エンドポイント:

```text
GET /health
GET /metrics
GET /keys
```

## 手順 3: プロとしての設計判断を書く

最終課題では、コードを書く前に次を文書化します。

```text
所有権:
Store は何を所有するか。

借用:
どの API は &str で十分か。

失敗:
入力エラー、I/O エラー、状態エラーをどう分けるか。

共有:
どの状態を Arc<Mutex<T>> で包むか。

復旧:
WAL 書き込み失敗時にどう振る舞うか。

運用:
どのログと metrics が必要か。

限界:
std-only で実務投入できない領域はどこか。
```

この文書が、最終課題のレビュー基準になります。

## よくあるつまずき

```text
Q. unsafe を使えば速くなりますか？
A. unsafe は高速化の魔法ではありません。安全な API で包める根拠とテストがない限り避けます。
```

```text
Q. Rc<RefCell<T>> は便利なので多用してよいですか？
A. 借用規則を実行時に移すため、設計の曖昧さを隠すことがあります。単一スレッドのグラフ構造など用途を限定します。
```

```text
Q. std-only で本番 HTTP サーバーを作るべきですか？
A. 学習目的なら価値があります。実務では HTTP、TLS、ログ、設定、監視の品質を考え、外部クレートやプロキシを検討します。
```

## 次の Level に進む条件

Level 9 が最終 Level です。完了条件は次です。

```text
PacketReader の境界チェックを説明できる
最終課題の層構成を自分で設計できる
std-only の価値と限界を説明できる
外部クレートを採用すべき場面を説明できる
```

## 公式 docs で確認する箇所

- std::boxed::Box: https://doc.rust-lang.org/std/boxed/struct.Box.html
- std::rc::Rc: https://doc.rust-lang.org/std/rc/struct.Rc.html
- std::cell::RefCell: https://doc.rust-lang.org/std/cell/struct.RefCell.html
- std::sync::Condvar: https://doc.rust-lang.org/std/sync/struct.Condvar.html
- Unsafe Rust: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

# Level 7: アーキテクチャと並行処理の統合

## この Level でできるようになること

スレッドプール、`Arc`、`Mutex`、責務分割を使い、中規模の KVS サーバーに向けた構成を設計できるようになります。

この Level の中心は、共有状態を小さくし、ロック範囲を短くし、ネットワークとロジックを分けることです。

## まず知るべき言葉

- `std::thread`: OS スレッドを扱う標準 API。
- `mpsc`: multiple producer, single consumer のチャンネル。
- `Arc<T>`: 複数スレッドで所有を共有する参照カウント型。
- `Mutex<T>`: 同時に 1 つのスレッドだけが中身へアクセスできるロック。
- スレッドプール: あらかじめ作ったワーカーへ仕事を渡す仕組み。
- graceful shutdown: 処理中の仕事を壊さずに終了する設計。
- レイヤー: 変更理由ごとに分けた責任のまとまり。

## なぜこれを学ぶのか

単一スレッドの KVS は理解しやすい一方で、1 クライアントが遅いと全体が止まります。複数クライアントに対応するには、並行処理が必要になります。

ただし、スレッドを増やすだけでは設計は良くなりません。

```text
悪い設計:
Arc<Mutex<App>> を全体に渡す
ロック中に TCP 書き込みをする
終了処理を考えない

良い設計:
共有する状態を Store に限定する
ロック中に行う処理を短くする
I/O と状態更新を分ける
```

## 手順 1: スレッドプール例を実行する

```bash
rustc --edition=2021 levels/level_07_integration/examples/thread_pool.rs -o /tmp/zero_to_rust_thread_pool
/tmp/zero_to_rust_thread_pool
```

期待する出力:

```text
job 0 -> 0
job 1 -> 1
job 2 -> 4
job 3 -> 9
all jobs submitted
```

ジョブの実行順は環境によって変わる可能性があります。並行処理では、順序に依存しない設計が必要です。

## 手順 2: 推奨構成を読む

中規模の KVS は、次のように分けます。

```text
server/
  tcp.rs        接続受付と入出力
  client.rs     1 クライアントの処理
  thread_pool.rs
protocol/
  command.rs    文字列から Command へ
  response.rs   Response から wire text へ
store/
  memory.rs     HashMap と TTL
persist/
  wal.rs        追記ログと復元
error.rs        共有エラー型
```

分割の基準はファイル数ではなく、変更理由です。

## 手順 3: ロック範囲を設計する

共有 Store を更新する場合でも、ロック中にやることは最小限にします。

```text
ロックを取る
Store を更新する
結果を値として取り出す
ロックを外す
TCP へレスポンスを書く
```

ロック中にネットワーク I/O をすると、遅いクライアントが他のクライアントを待たせます。

## よくあるつまずき

```text
Q. Arc<Mutex<T>> を使えばスレッド安全ですか？
A. データ競合は防げますが、設計が良いとは限りません。ロック粒度、待ち時間、デッドロックを考える必要があります。
```

```text
Q. RwLock の方が常に速いですか？
A. 読み取りが圧倒的に多い場合は候補になりますが、書き込みが多いと複雑さに見合わないことがあります。
```

```text
Q. trait は早めに入れるべきですか？
A. 変更される軸が見えてからで十分です。早すぎる抽象化はコードを読みにくくします。
```

## 次の Level に進む条件

```text
Arc と Mutex の役割を分けて説明できる
ロック中に I/O しない理由を説明できる
レイヤー分割を変更理由で説明できる
スレッドプールの役割を説明できる
```

## 公式 docs で確認する箇所

- std::thread: https://doc.rust-lang.org/std/thread/
- std::sync: https://doc.rust-lang.org/std/sync/
- std::sync::mpsc: https://doc.rust-lang.org/std/sync/mpsc/

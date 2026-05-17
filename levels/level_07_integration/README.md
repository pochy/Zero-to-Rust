# Level 7: アーキテクチャと並行処理の統合

## この Level でできるようになること

スレッドプール、`Arc`、`Mutex`、責務分割を使い、中規模の KVS サーバーに向けた構成を設計できるようになります。

この Level の中心は、共有状態を小さくし、ロック範囲を短くし、ネットワークとロジックを分けることです。

ここで初めて並行処理の用語がまとまって出ます。順番は、まず「仕事を別スレッドに渡す」、次に「結果を受け取る」、最後に「どうしても共有する状態を守る」です。`Arc<Mutex<T>>` は万能の解決策ではなく、共有が必要なときだけ使う道具です。

## まず知るべき言葉

- `std::thread`: OS スレッドを扱う標準 API。
- `mpsc`: multiple producer, single consumer のチャンネル。
- `Arc<T>`: 複数スレッドで所有を共有する参照カウント型。
- `Mutex<T>`: 同時に 1 つのスレッドだけが中身へアクセスできるロック。
- スレッドプール: あらかじめ作ったワーカーへ仕事を渡す仕組み。
- graceful shutdown: 処理中の仕事を壊さずに終了する設計。
- レイヤー: 変更理由ごとに分けた責任のまとまり。

`Arc` は「複数のスレッドが同じ所有物を指せるようにする」道具です。`Mutex` は「同時に 1 人だけが中身を触れるようにする」道具です。2 つを組み合わせると、複数スレッドから 1 つの状態を安全に変更できます。

## なぜこれを学ぶのか

単一スレッドの KVS は理解しやすい一方で、1 クライアントが遅いと全体が止まります。複数クライアントに対応するには、並行処理が必要になります。

ただし、スレッドを増やすだけでは設計は良くなりません。

```text
悪い設計:
アプリ全体を 1 つの共有ロックに入れる
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
  memory.rs     HashMap と状態管理
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
A. `RwLock` は、複数の読み取りを同時に許し、書き込みは 1 つだけにするロックです。読み取りが圧倒的に多い場合は候補になりますが、書き込みが多いと複雑さに見合わないことがあります。
```

```text
Q. trait は早めに入れるべきですか？
A. `trait` は複数の実装を同じ約束で扱うための道具です。変更される軸が見えてからで十分です。早すぎる抽象化はコードを読みにくくします。
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

## Rust らしさをさらに深掘りする

Rust の並行処理では、共有することより、共有しない設計を先に考えます。`Arc<Mutex<T>>` は便利ですが、所有権を channel で移せるならその方が単純な場合があります。

channel は、複数スレッドで同じデータを直接触る代わりに、メッセージとして値を送る仕組みです。まずは「共有する」以外の選択肢があると理解してください。

追加で読む箇所:

- [async、thread、Send/Sync](../../appendices/06_async_concurrency.md)
- [Professional Rust Map](../../appendices/08_professional_rust_map.md)

次の問いを追加で考えてください。

```text
Job に `Send + 'static` が必要な理由は何か。分からなければ、まず「別スレッドへ渡す仕事には制約がある」と理解すれば十分です。
Arc は共有所有、Mutex は排他制御だと分けて説明できるか
channel で所有権を渡す設計に変えると何が単純になるか
```

## 次に読む

- 前へ: [levels/level_06_evaluation/exercises.md](../level_06_evaluation/exercises.md)
- 次へ: [levels/level_07_integration/exercises.md](exercises.md)
- 関連: [docs/guide/CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md), [docs/guide/STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md)

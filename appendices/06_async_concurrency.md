# 06: async、thread、Send/Sync

## 目的

Rust の並行処理には、OS スレッド、チャンネル、ロック、async runtime があります。どれを選ぶかは性能だけでなく、I/O の性質、ライブラリ ecosystem、運用の分かりやすさで決まります。

## thread と async の違い

```text
thread:
OS が実行単位を管理する。CPU 処理や単純な並行化に分かりやすい。

async:
Future が進行状態を持ち、runtime が待機中の処理を切り替える。大量 I/O に向く。
```

async は自動的に速い魔法ではありません。blocking I/O を async タスク内に入れると runtime を詰まらせます。

## Future はまだ終わっていない値

`async fn` は、呼ぶだけでは本体が最後まで実行されません。`Future` を返し、`.await` された時に進みます。

```rust
async fn read_message() -> String {
    "hello".to_string()
}
```

実行するには runtime が必要です。std には汎用 async runtime は含まれていないため、実務では Tokio などを選びます。

## Send と Sync

```text
Send:
値を別スレッドへ移してよい。

Sync:
&T を複数スレッドから共有してよい。
```

`Arc<T>` は共有所有を可能にしますが、中身を変更できるとは限りません。変更するなら `Mutex<T>` や `RwLock<T>` のような同期が必要です。

## lock と await の危険

async では、ロックを持ったまま `.await` しないことが重要です。

```text
悪い流れ:
lock を取る
ネットワーク I/O を await する
他タスクが lock 待ちで止まる
```

状態更新を短く終え、I/O はロック外で行います。これは Level 7 の「ロック中に TCP 書き込みをしない」と同じ思想です。

## channel の設計

channel は、共有状態を直接ロックせずにメッセージで渡す選択肢です。

```text
Arc<Mutex<T>>:
同じ状態を複数処理で触る。

channel:
所有権をメッセージとして移動する。
```

Rust では「共有する」より「所有権を移す」方が単純になる場面があります。

## 進級チェック

```text
thread と async の違いを説明できるか
Future がいつ実行されるか説明できるか
Send と Sync を共有設計として説明できるか
ロック中に await しない理由を説明できるか
```

## 公式 docs

- https://doc.rust-lang.org/book/ch16-00-concurrency.html
- https://doc.rust-lang.org/book/ch17-00-async-await.html
- https://doc.rust-lang.org/std/marker/trait.Send.html
- https://doc.rust-lang.org/std/marker/trait.Sync.html

## 次に読む

- 前へ: [appendices/05_cargo_ecosystem.md](05_cargo_ecosystem.md)
- 次へ: [appendices/07_unsafe_ffi_performance.md](07_unsafe_ffi_performance.md)
- 関連: [appendices/README.md](README.md), [docs/guide/LEARNING_PATH.md](../docs/guide/LEARNING_PATH.md)

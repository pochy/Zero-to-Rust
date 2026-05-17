# 07: unsafe、FFI、no_std、performance

## 目的

`unsafe` は Rust の安全性を捨てる機能ではありません。コンパイラが確認できない一部の条件を、人間が文書化し、テストし、レビューする領域です。

## unsafe でできること

代表例:

```text
raw pointer の dereference
unsafe 関数の呼び出し
mutable static へのアクセス
unsafe trait の実装
union field へのアクセス
```

safe Rust で書けるなら、まず safe Rust を選びます。`unsafe` は高速化の合図ではなく、保証の移譲です。

## Safety comment

`unsafe` ブロックには、なぜ安全と言えるかを書きます。

```rust
fn first_byte(input: &[u8]) -> Option<u8> {
    if input.is_empty() {
        return None;
    }

    // SAFETY: input is checked to be non-empty, so index 0 is in bounds.
    let value = unsafe { *input.as_ptr() };
    Some(value)
}
```

この例は学習用です。通常は `input.first().copied()` の方が良い設計です。

## FFI

Rust から C ABI へ接続する場合、境界は特に慎重に扱います。

```text
文字列の ownership は誰が持つか
null pointer を受け取る可能性はあるか
buffer length は正しいか
panic を FFI 境界の外へ出していないか
解放関数は対になっているか
```

FFI では Rust の型が守れる範囲が狭くなるため、境界の手前で validation を厚くします。

## no_std

`no_std` は標準ライブラリを使わない環境です。組み込み、OS、カーネル、特殊 runtime で出ます。

```text
std:
OS、ファイル、ネットワーク、スレッドなどを含む。

core:
言語の中核型や trait。

alloc:
heap allocation を使う型。
```

この教材の本編は `std` を使います。`no_std` は Rust の所有権が低レイヤーでも有効なことを理解した後の発展です。

## performance

Rust の性能改善は、まず測定から始めます。

```text
1. 仕様をテストで固定する
2. ベンチマークを取る
3. allocation、clone、lock、I/O、algorithm を見る
4. safe Rust で改善する
5. unsafe が必要なら安全条件を書く
```

`clone` を消す、`String` を `&str` にする、`Vec` の capacity を予約する、lock 範囲を短くする、I/O を buffer する。この順番で改善できることが多いです。

## 進級チェック

```text
unsafe を使う前に書くべき安全条件を説明できるか
FFI 境界で ownership が曖昧になる危険を説明できるか
std / core / alloc の違いを説明できるか
性能改善を測定から始める理由を説明できるか
```

## 公式 docs

- https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html
- https://doc.rust-lang.org/nomicon/
- https://doc.rust-lang.org/reference/items/external-blocks.html

## 次に読む

- 前へ: [appendices/06_async_concurrency.md](06_async_concurrency.md)
- 次へ: [appendices/08_professional_rust_map.md](08_professional_rust_map.md)
- 関連: [appendices/README.md](README.md), [docs/guide/LEARNING_PATH.md](../docs/guide/LEARNING_PATH.md)

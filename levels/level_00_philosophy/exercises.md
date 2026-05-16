# Level 0 Exercises

## 1. 所有権キャンバスを埋める

[examples/ownership_canvas.md](examples/ownership_canvas.md) を読み、`ファイルを読み込んで表示する CLI` を題材にして各問いへ答えてください。

## 2. 出力ではなく責任を比較する

次の 2 つの関数を見て、どちらが所有権を受け取り、どちらが借用しているかを書いてください。

```rust
fn show(value: &str) {
    println!("{}", value);
}

fn consume(value: String) {
    println!("{}", value);
}
```

## 3. 設計判断を書く

`clone()` を使ってよい場面と、使う前に設計を見直すべき場面を 1 つずつ書いてください。

## 提出物

```text
1. 所有権キャンバスの回答
2. show と consume の違いの説明
3. clone と unwrap に関する自分の判断基準
```

## 進級チェック

```text
所有権、借用、Result を「文法」ではなく「責任の表現」として説明できるか？
Rust のコンパイルエラーを設計のヒントとして読めるか？
```

## 学習記録

[CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md) の Level 0 を A/B/C で自己評価してください。

[STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md) には、次を書きます。

```text
Rust を他の言語と同じように書こうとすると、どの責任が曖昧になるか
clone や unwrap を使う前に、自分は何を確認するか
```

## 追加演習: Rust を学ぶ理由を書く

他の言語でも同じ CLI は作れます。それでも Rust で作る意味を、次の観点で 5 行以上書いてください。

```text
所有権:
失敗:
共有状態:
復旧:
将来の保守:
```

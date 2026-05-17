# CS 3 Exercises

## 1. size を見る

`std::mem::size_of` を使って、次の size を表示してください。

```text
i32
usize
String
Vec<u8>
&str
&[u8]
Box<i32>
```

結果を見て、heap 上の中身まで size に含まれるか説明してください。

## 2. String と Vec<u8>

同じ file を byte と text の両方で読んでください。

```text
std::fs::read
std::fs::read_to_string
```

text として読めない file があり得る理由を書いてください。

## 3. clone の cost

大きな `String` を clone する version と、`&str` で借りる version を作り、処理時間や allocation の違いを考察してください。

## 提出物

```text
memory_sizes.rs
bytes_vs_text.rs
clone_notes.md
```

## 進級チェック

```text
String の size と中身の size を区別できるか
byte と UTF-8 text を区別できるか
clone を使う前に cost を説明できるか
```

## 次に読む

- 前へ: [computer_science/levels/cs_03_computer_systems/README.md](README.md)
- 次へ: [computer_science/levels/cs_04_os_cli_io/README.md](../cs_04_os_cli_io/README.md)
- 関連: [computer_science/CHECKPOINTS.md](../../CHECKPOINTS.md), [computer_science/SOLUTIONS.md](../../SOLUTIONS.md), [computer_science/glossary.md](../../glossary.md)

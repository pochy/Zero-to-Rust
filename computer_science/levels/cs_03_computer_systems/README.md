# CS 3: Computer Systems

## この Level でできるようになること

binary、byte、CPU、memory、stack、heap、cache、allocation を、Rust の型と実行時の cost に結びつけて説明できるようになります。

## まず知るべき言葉

```text
bit
byte
integer representation
CPU
register
memory
stack
heap
cache
allocation
```

## なぜこれを学ぶのか

高級言語だけを書いていると、処理は「ただ実行される」ように見えます。しかし実際には、CPU が命令を実行し、memory から data を読み、cache に乗るかどうかで速度が変わります。

CMU 15-213 のような computer systems 教材が強いのは、program がどう実行され、情報がどう保存され、どう通信するかを programmer の視点で扱うからです。

## 手順 1: byte と text を分ける

Rust では `String` と `Vec<u8>` は違います。

```text
String: UTF-8 text
Vec<u8>: byte sequence
&str: borrowed UTF-8 text
&[u8]: borrowed byte sequence
```

file、network、binary format はまず byte です。text として解釈できるとは限りません。

## 手順 2: stack と heap を見る

```text
i32: 値そのものは小さく stack に置きやすい
String: pointer、length、capacity を持ち、中身は heap
Vec<T>: pointer、length、capacity を持ち、中身は heap
Box<T>: heap に置いた T への owner
```

Rust の ownership は、heap allocation の寿命を明確にします。

## 手順 3: cache と連続 memory を考える

`Vec<T>` は連続 memory に置かれるため、順番に読む処理と相性が良いです。linked list は挿入が簡単に見えますが、pointer を辿るため cache に乗りにくいことがあります。

data structure は Big-O だけでなく、memory locality でも評価します。

## TypeScript / Go ならどう見えるか

TypeScript では多くの memory detail が runtime に隠れます。Go は pointer、slice、allocation が見えますが GC があります。Rust は allocation と ownership が型や API に現れやすく、低レイヤの理解と相性が良いです。

## よくあるつまずき

```text
String をただの文字列だと思う
clone の cost を無視する
Big-O だけで data structure を選ぶ
byte と text を混同する
cache locality を無視する
```

## 次の Level に進む条件

```text
String と Vec<u8> の違いを説明できる
stack と heap の違いを説明できる
Vec が順次アクセスに強い理由を説明できる
clone が何を複製するか疑える
```

## 公式 docs で確認する箇所

```text
std::string::String
std::vec::Vec
std::boxed::Box
std::mem
```


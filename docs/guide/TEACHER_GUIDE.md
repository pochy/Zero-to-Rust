# Teacher Guide

このガイドは、Zero to Rust を教える人、レビューする人、または自分で学習計画を組む人のための資料です。

## 教えるときの原則

```text
構文より責任を問う。
正解コードより判断理由を問う。
コンパイルエラーを失敗ではなく設計 feedback として扱う。
clone、unwrap、Arc<Mutex<T>> を禁止語にせず、理由を言わせる。
```

## よく詰まる場所

### Level 1: `String` と `&str`

詰まり方:

```text
String は文字列、&str も文字列、という表面的な理解で止まる。
```

問い:

```text
この関数は文字列を保存しますか。
保存しないなら、なぜ所有する必要がありますか。
```

合格ライン:

```text
読むだけなら &str、保存や返却なら String と説明できる。
```

### Level 2: `Result`

詰まり方:

```text
エラー処理を面倒な例外処理として見る。
```

問い:

```text
この失敗は誰が回復できますか。
String に変換するとどの情報を失いますか。
```

合格ライン:

```text
I/O の失敗を Result として上位へ返す理由を説明できる。
```

### Level 3-5: 責務分離

詰まり方:

```text
main.rs に全部書いた方が短いと感じる。
```

問い:

```text
JSON protocol に変えたとき Store は変わりますか。
TCP をやめたら Command は残りますか。
```

合格ライン:

```text
wire text、Command、Store、Response を分けて説明できる。
```

### Level 7: 並行処理

詰まり方:

```text
Arc<Mutex<T>> を付ければスレッド安全だと思う。
```

問い:

```text
何を共有していますか。
ロック中に何をしていますか。
channel で所有権を移す設計は可能ですか。
```

合格ライン:

```text
Arc は共有所有、Mutex は排他制御と分けて説明できる。
```

### Level 8-9: 本番判断

詰まり方:

```text
動けば完成だと思う。
```

問い:

```text
落ちた後にどのファイルから復旧しますか。
WAL 書き込みに失敗した操作を成功扱いしてよいですか。
std-only で本番 HTTP を抱える責任を説明できますか。
```

合格ライン:

```text
機能、復旧、運用、依存採用を分けて説明できる。
```

## 採点ルーブリック

| 評価 | 状態 |
| --- | --- |
| 1 | コードは動くが、所有者や失敗の扱いを説明できない |
| 2 | 所有と借用を一部説明できるが、clone/unwrap が理由なし |
| 3 | 小さい API の所有境界と Result を説明できる |
| 4 | 責務分離、テスト、並行処理、WAL の判断を説明できる |
| 5 | std-only と ecosystem の境界、運用、レビュー観点まで説明できる |

## レビュー時の質問例

```text
この値の所有者は誰ですか。
この関数は所有する必要がありますか。
この None は正常ですか、それとも Err にすべきですか。
この clone は何の責任を分離していますか。
この unwrap は失敗しない根拠がありますか。
この lock 中に I/O していますか。
この trait はどの変更軸を抽象化していますか。
この crate に任せる責任は何ですか。
```

## 模範回答の使い方

`solutions/` は提出前に読ませないでください。まず自分の回答を書かせ、その後で比較します。

比較時に見る点:

```text
答えが同じかではなく、責任が説明されているか。
短いコードかではなく、変更時に壊れにくいか。
用語を暗記しているかではなく、具体例に適用できるか。
```

## 最終発表の形式

学習者は `projects/final_kvs_server` について、10 分で次を説明します。

```text
1. 全体 architecture
2. Store の所有境界
3. Command / Response の役割
4. WAL 書き込み順序
5. Arc<Mutex<AppState>> の利点と限界
6. std-only の限界
7. ecosystem へ移すなら何を移すか
```

この発表ができれば、Rust の知識が単語ではなく設計判断として身についています。

## 次に読む

- 前へ: [docs/guide/ASSESSMENT.md](ASSESSMENT.md)
- 次へ: [docs/guide/REVIEW_CHECKLIST.md](REVIEW_CHECKLIST.md)
- 関連: [docs/guide/CHECKPOINTS.md](CHECKPOINTS.md), [solutions/README.md](../../solutions/README.md), [docs/INDEX.md](../INDEX.md)

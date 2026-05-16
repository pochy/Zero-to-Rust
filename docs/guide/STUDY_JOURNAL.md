# Rust Study Journal

このファイルは、学習中に自分の判断を記録するためのテンプレートです。

Rust は、理解したつもりのまま進むと急に難しくなります。各 Level の終わりに、コードを写すだけでなく「なぜそうしたか」を短く書いてください。

書いた内容は正解でなくて構いません。後から読み返して、判断が変わった場所が Rust の理解が深まった場所です。

## 毎回書くこと

```text
Level:
日付:
今日動かしたファイル:
今日読んだ appendices:
```

## 1. 所有権

```text
この Level で所有していた値:
一時的に借りていた値:
clone した場所:
clone が必要だった理由:
clone しない別案:
```

## 2. 失敗

```text
起きうる失敗:
Result で返した失敗:
Option で表した「ない」:
panic / unwrap / expect を使った場所:
それが許される理由:
```

## 3. 境界

```text
この Level で分けた責任:
分けなかった責任:
あとで分けるべきかもしれない責任:
public に見せる型:
内部だけで使う型:
```

## 4. Rust らしい判断

```text
Rust だから早い段階で気づけた問題:
他の言語なら実行時まで残りそうな問題:
コンパイラに止められてよかった点:
コンパイラに止められて設計を変えた点:
```

## 5. 次の Level へ進む前の確認

```text
CHECKPOINTS.md の自己評価: A / B / C
説明できるようになったこと:
まだ曖昧なこと:
戻って読む場所:
次に小さく変更すること:
```

## 記入例: Level 1

```text
Level:
1

今日動かしたファイル:
levels/level_01_intro/examples/hello_ownership.rs

この Level で所有していた値:
String::from("Rust") で作った name。

一時的に借りていた値:
borrow_name(&name) に渡した &String / &str 的な参照。

clone した場所:
なし。

起きうる失敗:
この例では I/O 失敗はない。主な失敗は所有権移動後に name を使おうとするコンパイルエラー。

Rust だから早い段階で気づけた問題:
take_name(name) の後で name を使う設計が、コンパイル時に止まった。

CHECKPOINTS.md の自己評価:
B。String と &str は説明できるが、関数引数をいつ &str にするかはまだ迷う。

戻って読む場所:
appendices/01_ownership_lifetimes.md
```

## 記入例: Level 8

```text
Level:
8

今日動かしたファイル:
levels/level_08_production/examples/wal_restore.rs

この Level で所有していた値:
Store が HashMap を所有し、WAL は状態変更の記録を所有する。

起きうる失敗:
WAL への書き込み失敗、復旧時の壊れた行、ファイル読み込み失敗。

Result で返した失敗:
I/O 失敗と、復旧不能な形式エラー。

Option で表した「ない」:
GET でキーがない状態。

Rust らしい判断:
状態変更前に WAL 書き込みを成功させる順序を API 側で守る。失敗を握りつぶさず、Store 更新をしない。

CHECKPOINTS.md の自己評価:
A。ただし、壊れた WAL 行を skip するか停止するかは運用方針次第なので runbook に書く。
```

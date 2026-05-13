# Solutions: Level 0-4

## Level 0

### 所有権キャンバス

回答例:

```text
データはどこで生まれるか:
path と pattern はコマンドライン引数から来る。ファイル内容は load_file が String として作る。検索結果は search が Vec<MatchLine> として作る。

誰が所有するか:
読み込んだファイル内容は呼び出し元が所有し、search へ &str として貸す。検索結果は Vec<MatchLine> が所有し、表示層へ渡す。

誰が借りるだけでよいか:
pattern は検索中に読むだけなので &str。表示関数は MatchLine の参照を読むだけでよい。

どこで失敗するか:
引数不足、ファイルなし、権限なし、UTF-8 エラー。

失敗をどこで扱うか:
load_file は io::Error を返す。main がユーザー向けに表示する。検索結果 0 件は失敗ではない。
```

レビュー観点:

```text
「表示できた」ではなく「誰が所有するか」を書けているか。
Result と Option の違いを混同していないか。
```

### clone と unwrap

回答例:

```text
clone してよい場面:
別スレッドへ小さい設定値を渡し、元の所有者も保持したい場合。

見直すべき場面:
コンパイルエラーを消すためだけに大きな Vec<String> を何度も clone している場合。

unwrap してよい場面:
テストで「ここは Ok であるべき」と仕様を短く書く場合。

避ける場面:
ユーザー入力、I/O、ネットワーク、設定読み込み。
```

## Level 1

### API 設計

回答例:

```rust
fn validate_user_name(name: &str) -> bool
fn save_user_name(name: String)
fn render_user_name(name: &str) -> String
```

理由:

```text
validate_user_name:
読むだけなので &str。所有する必要はない。

save_user_name:
保存先が値を保持するなら String を受け取り、保存先が所有する設計が自然。

render_user_name:
入力は読むだけなので &str。戻り値は新しい表示文字列なので String を返す。
```

レビュー観点:

```text
String を要求する理由が「なんとなく」になっていないか。
戻り値が新しく作られる場合は所有型になることを説明しているか。
```

## Level 2

### エラー分類

回答例:

```text
引数エラー:
path が指定されていない。usage を表示して終了する。

I/O エラー:
ファイルなし、権限なし、ディレクトリ指定。

UTF-8 エラー:
read_to_string は UTF-8 として読めない場合にも失敗する。

ユーザーに表示するメッセージ:
どの path で失敗したか、何をすればよいか。

開発者がログで見たい情報:
io::ErrorKind、OS error、実行時 path。
```

設計判断:

```text
学習初期は io::Error を保つ。標準 API が返す失敗分類を失わないため。
複数種類の失敗を 1 つの CLI で扱い始めたら enum AppError を作る。
```

## Level 3

### trait 導入

回答例:

```text
今すぐ導入しない:
実装が memory store だけなら trait は変更軸を先取りしすぎる。

導入してよい場面:
MemoryStore、WalStore、RemoteStore を同じ API で扱う必要が出たとき。

テストのためだけに導入するか:
時計、ファイルシステム、永続化境界など、差し替えたい依存が明確ならよい。
```

レビュー観点:

```text
trait を「きれいだから」ではなく「変更される軸があるから」導入しているか。
```

## Level 4

### Iterator 版と for 版

回答例:

```text
for ループ:
状態を一行ずつ追いやすい。初学者とエラー処理に向く。

iterator chain:
filter/map/collect でデータ変換の流れが短く書ける。複雑になりすぎると読みにくい。

所有権:
content.lines() は content を借り、各 line は &str。MatchLine に入れる時点で String と PathBuf を所有させる。
```

判断:

```text
この教材では for ループでも十分。Iterator へ変える場合は、短さより責任が読めることを優先する。
```

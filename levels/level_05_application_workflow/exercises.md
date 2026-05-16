# Level 5 Exercises

## 1. 小さく変更する

`tcp_kvs_workflow.rs` に `EXISTS key` コマンドを追加してください。

レスポンスは次のどちらかにします。

```text
TRUE
FALSE
```

## 2. 出力を比較する

次の入力を追加し、それぞれのレスポンスを観察してください。

```text
SET lang Rust
EXISTS lang
DEL lang
EXISTS lang
UNKNOWN x
```

## 3. 設計判断を書く

不正コマンドを見つけたとき、次のどちらにするか判断してください。

```text
parse_command が Err を返す
parse_command が Response::Error を直接返す
```

`Err` は `Result` の失敗側です。ここでは「文字列をコマンドに変換できなかった失敗」を、パース層で止めるか、レスポンス生成まで含めるかを考えます。

パース層と実行層の責任を分けて説明してください。

## 提出物

```text
1. EXISTS コマンドの実装
2. 追加した入力とレスポンス
3. パースエラーをどの層で扱うかの判断
```

## 進級チェック

```text
TCP 層、プロトコル層、Store 層の責任を分けて説明できるか？
文字列を早めに enum へ変換する理由を説明できるか？
```

## 学習記録

[CHECKPOINTS.md](../../docs/guide/CHECKPOINTS.md) の Level 5 を A/B/C で自己評価してください。

[STUDY_JOURNAL.md](../../docs/guide/STUDY_JOURNAL.md) には、次を書きます。

```text
parse、execute、format の責任分担
Command と Response を enum にする理由
wire format を変更しても Store を変えないための境界
```

## 追加演習: wire format を設計する

今の text protocol を JSON protocol に変える場合、どの層を変更すべきか書いてください。

この追加演習は発展です。JSON そのものを実装する必要はありません。文字列の形式が変わっても、Store の責任を変えないことを確認します。

```text
parse_command
Command enum
Store
Response
to_wire
TCP 層
```

`Store` を変更せずに済む設計になっているか確認してください。

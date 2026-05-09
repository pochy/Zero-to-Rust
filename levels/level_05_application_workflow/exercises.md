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

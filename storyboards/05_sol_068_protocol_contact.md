# Sol 068: 最初の通信プロトコル

対応教材: [Level 5: TCP KVS の最小アプリケーションワークフロー](../levels/level_05_application_workflow/README.md)

## 状況

六十八日目。

マークは古い探査機の通信装置を復旧し、HAB の端末につないだ。帯域は細い。遅延はひどい。文字列を送るだけでも、地球側との約束が必要だ。

「通信できるかどうかより、何をどう送るかが問題だ」

彼はプロトコルを決めた。

```text
SET key value
GET key
DEL key
EXISTS key
QUIT
```

返答も決める。

```text
OK
VALUE value
NOT_FOUND
ERROR message
BYE
```

## ログ

**LOG 068.1**

マーク:

「地球と話せるかもしれない。俺が最初に送るべき文字列は何か。『生きてる』か。『助けて』か。いや、まずプロトコル仕様だ」

彼は `tcp_kvs_workflow.rs` を実行する。

```bash
rustc --edition=2021 levels/level_05_application_workflow/examples/tcp_kvs_workflow.rs -o /tmp/zero_to_rust_workflow
/tmp/zero_to_rust_workflow
```

期待する出力:

```text
> SET name Rust
< OK
> GET name
< VALUE Rust
> GET missing
< NOT_FOUND
> DEL name
< OK
> GET name
< NOT_FOUND
```

マーク:

「ここではまだ本物の TCP を使わない。いい判断だ。通信機が不安定なときに、プロトコルのバグと配線のバグを同時に追うのは、火星でやる趣味ではない」

## 会話

管制:

「マーク、聞こえるか」

マーク:

「聞こえる。たぶん。そっちは俺の `SET alive true` を受け取ったか」

管制:

「受け取った。次に何が必要だ」

マーク:

「まず責務分離だ」

管制:

「水ではなく？」

マーク:

「水も必要だ。だが、TCP 接続を受ける関数の中で `split` して `HashMap` を直接触ってレスポンスまで作る設計を送ってきたら、俺は火星から怒る」

管制:

「層を分けろということか」

マーク:

「そうだ」

```text
wire text
Command
Store operation
Response
wire text
```

管制:

「不正コマンドは？」

マーク:

「`parse_command` が `Err` を返す。パース層はパースする。実行層は実行する。エラー表示は上位で決める」

## Rust 任務

1. `tcp_kvs_workflow.rs` を実行する。
2. `Command` と `Response` が `enum` で表現されていることを確認する。
3. `EXISTS key` を追加する。
4. 不正コマンドを `parse_command` で `Err` にする設計と、直接 `Response::Error` にする設計を比較する。

演習:

- [Level 5 exercises](../levels/level_05_application_workflow/exercises.md)

## マークの独り言

「文字列は早めに型に変換する。これは火星でかなり大事だ」

```text
"SET name Rust":
ただの文字列。間違いも混ざる。

Command::Set { key, value }:
意味のある要求。以後の処理で迷わない。
```

マーク:

「人間も同じだ。『なんか壊れた』では直せない。『エアロック3の内側シールが 12% 漏れている』なら直せる」

## 進級チェック

次の問いに答えられたら、Sol 097 へ進む。

```text
TCP 層、プロトコル層、Store 層の責任を分けて説明できるか。
文字列を `Command` enum に変換する理由を説明できるか。
不正入力を panic ではなくエラーとして扱えるか。
レスポンスの wire text を仕様として固定できるか。
```

マークのメモ:

「今日、地球と話す準備ができた。最初に送る言葉は決めている。`GET rescue_plan`」


# final_kvs_server Exercises

## 1. WAL の壊れ方を観察する

WAL に不正な行を追加し、起動時にどう失敗するか確認してください。

```text
BROKEN LINE
```

提出物:

```text
起動時エラーの要約
壊れた行をスキップしない理由
スキップする設計に変える場合のリスク
```

## 2. Metrics を増やす

次の counters を追加してください。

```text
gets
sets
deletes
not_found
```

提出物:

```text
Metrics に追加したフィールド
どの層で counter を増やしたか
metrics の出力例
```

## 3. request size limit を追加する

TCP で長すぎる 1 行を拒否する設計を考えてください。

判断すること:

```text
何 byte まで受け入れるか
超過時に ERROR を返すか接続を閉じるか
WAL には何も書かないことをどう保証するか
```

## 4. graceful shutdown を設計する

今の `run_tcp_server` は無限に accept します。終了処理を設計してください。

候補:

```text
Ctrl-C を受ける crate を使う
admin endpoint に shutdown を追加する
listener を nonblocking にして終了 flag を見る
```

std-only で進める場合と、外部 crate を採用する場合を比較してください。

## 5. ecosystem 版へ移行する

次の責任をどの crate に任せるか書いてください。

```text
TCP/HTTP server
JSON protocol
CLI/config
logging
error definitions
metrics export
```

提出物:

```text
採用する crate
採用理由
std-only で残す責任
移行時に壊してはいけない API
```

## 進級チェック

```text
WAL、TTL、metrics、admin HTTP を責任境界として説明できるか
ロック範囲の限界を説明できるか
std-only から ecosystem へ移る理由を説明できるか
```

## 学習記録

[../../STUDY_JOURNAL.md](../../STUDY_JOURNAL.md) に、次を書いてください。

```text
final_kvs_server で自分が最も納得した設計判断
final_kvs_server で実務投入前に変える設計
kvs_std / kvs_ecosystem / final_kvs_server の違い
次に crate に任せたい責任と、その理由
```

[../PROJECT_WALKTHROUGH.md](../PROJECT_WALKTHROUGH.md) の完了条件にも答えてください。

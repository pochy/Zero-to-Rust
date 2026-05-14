# CS 6 Exercises

## 1. index 設計

URL shortener の table を考えます。

```text
id
short_code
original_url
created_at
user_id
```

どの column に index を貼るべきか、理由を書いてください。

## 2. EXPLAIN を読む

PostgreSQL を使える場合は、簡単な table を作って `EXPLAIN` を実行してください。使えない場合は、仮の plan を読み、scan と index scan の違いを書いてください。

## 3. transaction 境界

次の処理を transaction にすべきか判断してください。

```text
short_code を生成する
DB に insert する
analytics counter を増やす
email を送る
```

## 提出物

```text
index_design.md
transaction_boundary.md
explain_notes.md
```

## 進級チェック

```text
index が write を遅くし得る理由を説明できるか
transaction に入れるべき処理と入れるべきでない処理を分けられるか
WAL と application log を混同していないか
```


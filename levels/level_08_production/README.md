# Level 8: 運用、本番設計、復旧

## この Level でできるようになること

設定、ログ、WAL、復旧、バックアップ、運用上の限界を設計に含められるようになります。

この Level の中心は、機能が動くことではなく、障害後に説明して戻せることです。

## まず知るべき言葉

- 設定: bind address、worker 数、データ保存先など環境ごとに変わる値。
- 環境変数: 実行環境からプログラムへ渡す設定。
- WAL: 操作を追記し、再起動時に再生して状態を復元するログ。
- health check: プロセスが応答できるか確認する入口。
- metrics: 処理件数、エラー数、接続数などの観測値。
- backup: 復旧のためにデータを別の場所へ保存すること。
- restore: backup や WAL から状態を戻すこと。

## なぜこれを学ぶのか

本番に近いシステムでは、次の問いに答えられないと運用できません。

```text
どのアドレスで待ち受けるか
障害時にどのファイルから復旧するか
どのログを見れば原因が分かるか
WAL が壊れたらどうするか
どこまでが std-only の限界か
```

Rust の型で守れる範囲と、運用手順で守る範囲を分けて考えます。

## 手順 1: WAL 復元例を実行する

```bash
rustc --edition=2021 levels/level_08_production/examples/wal_restore.rs -o /tmp/zero_to_rust_wal
/tmp/zero_to_rust_wal
```

期待する出力:

```text
restored name = Rust
restored lang = std
exists old = false
```

見るべき点は、現在の `HashMap` を保存しているのではなく、操作ログを再生して状態を戻していることです。

## 手順 2: 設定を環境変数で変える

本番では、コードにアドレスや保存先を直接書きません。

```text
APP_ADDR=127.0.0.1:4000
APP_WORKERS=4
APP_DATA=/var/lib/zero-to-rust/data.wal
```

設定は、起動時に読み、失敗したら明確なエラーにします。

```text
設定の読み込み失敗:
起動前に止める。

コマンド処理中の入力エラー:
クライアントへ ERROR を返す。

WAL 書き込み失敗:
状態更新の扱いを慎重に決める。
```

## 手順 3: std-only の限界を明記する

標準ライブラリだけでも、TCP、ファイル、スレッド、ロック、簡易 HTTP 風レスポンスは作れます。

一方で、実務品質の TLS、完全な HTTP、構造化ログ、堅牢な CLI、非同期ランタイムは、外部クレートや周辺コンポーネントを検討する領域です。

重要なのは、外部クレートを避け続けることではありません。std で何を理解し、どこから外部クレートを採用すべきか判断できることです。

## よくあるつまずき

```text
Q. WAL に書く前に OK を返してよいですか？
A. 障害時に失われてもよい操作かどうかで決まります。永続化を保証するなら、書き込み成功後に OK を返します。
```

```text
Q. localhost で動いたら公開してよいですか？
A. いいえ。bind address、認証、TLS、入力制限、ログの機密情報を検討する必要があります。
```

```text
Q. println ログで十分ですか？
A. 学習段階では十分です。本番では構造化、ローテーション、収集、機密情報マスクが必要になります。
```

## 次の Level に進む条件

```text
WAL の再生で状態が戻る理由を説明できる
設定、ログ、復旧手順を機能と同じくらい重要だと説明できる
std-only でできることと厳しいことを分けて説明できる
```

## 公式 docs で確認する箇所

- std::env: https://doc.rust-lang.org/std/env/
- std::fs::OpenOptions: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
- std::time: https://doc.rust-lang.org/std/time/

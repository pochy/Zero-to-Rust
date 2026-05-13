# Solutions: Level 5-9

## Level 5

### wire format を JSON に変える場合

回答例:

```text
変更する:
parse_command
Response::to_wire
必要なら Command / Response の serialize 表現

変更しない:
Store の内部データ構造
Store::execute の基本責任
```

理由:

```text
Store は TCP や JSON を知らないべき。Command を受け取り Response を返すだけなら、wire format を text から JSON に変えても store 層は変わらない。
```

レビュー観点:

```text
文字列 parsing の都合が store に漏れていないか。
Response::Error と parse error の責任が混ざっていないか。
```

## Level 6

### 品質ゲート

回答例:

```text
cargo test --workspace:
仕様として固定した振る舞いを守る。メモリ安全性の全てや性能は保証しない。

cargo fmt --check:
差分を読みやすくする。設計の良し悪しは保証しない。

cargo clippy --workspace --all-targets:
よくある書き方の問題を見つける。仕様ミスは見つけられない。

python3 tools/check_links.py:
教材内のローカルリンク切れを見つける。外部 URL の生存は確認しない。
```

## Level 7

### `Arc<Mutex<Store>>` と channel

回答例:

```text
Arc<Mutex<Store>>:
複数スレッドが同じ Store を共有する。単純だが lock 競合が起きる。

channel:
Store 専用スレッドが所有者になる。各クライアントは Command を送るだけ。共有状態を減らせる。
```

比較:

```text
所有権:
channel の方が「Store の所有者は専用スレッド」と明確。

ロック:
Arc<Mutex<Store>> は lock が必要。channel は Store 自体の lock が不要になる場合がある。

終了処理:
channel は送信側を閉じることで終了を表しやすい。

遅いクライアント:
レスポンス送信を Store 所有スレッドでやると詰まる。結果だけ返し、I/O は client 側で行う設計がよい。
```

## Level 8

### runbook

回答例:

```text
プロセスが起動しない:
1. APP_ADDR / ADMIN_ADDR の競合を見る。
2. WAL_PATH の親ディレクトリへ書けるか見る。
3. WAL の壊れた行を確認する。
4. 直近の変更を確認する。
5. 復旧方針を決める。

WAL が壊れている:
1. どの行で parse error になるか見る。
2. バックアップがあるか確認する。
3. スキップするか起動停止するか判断する。
4. 判断を記録する。
5. 再起動して復旧結果を確認する。
```

型で守る範囲:

```text
Command の形、Response の形、Option/Result の区別。
```

運用で守る範囲:

```text
バックアップ、ログ確認、WAL 修復、ポート管理、監視。
```

## Level 9

### std-only と ecosystem

回答例:

```text
serde:
JSON の parse/serialize を任せる。手書き parser のバグを減らす。

clap:
CLI help、引数 validation、env 連携を任せる。

thiserror:
library の分類可能な enum error を簡潔に実装する。

anyhow:
binary 上位で文脈つきに失敗を返す。

tracing:
println ではなく、target、level、span、field を持つログへ進める。

tokio:
大量接続や async HTTP stack と統合する。
```

採用しない判断:

```text
学習段階では std-only のままにして、責任を自分で説明できるようにする。
本番化するとき、HTTP/TLS/認証/metrics は成熟した crate や外部コンポーネントを検討する。
```

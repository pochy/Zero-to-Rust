# Assessment Answer Examples

## 1. Store は何を所有していますか

回答例:

```text
Store は HashMap<String, Entry> を所有しています。key は String、value も Entry 内の String として所有します。Entry は expires_at: Option<SystemTime> も持ち、TTL の有無を Store の状態として表します。
```

良い理由:

```text
呼び出し元の入力文字列や TCP buffer の寿命に依存しないため、Store は長く状態を保持できます。
```

## 2. parse_command はなぜ &str を受け取りますか

回答例:

```text
parse_command は入力を読むだけで保存しないため &str で十分です。Command を作る時点で key/value を String に変換し、後続処理が入力 buffer の寿命に依存しないようにします。
```

## 3. GET missing はなぜ Error ではありませんか

回答例:

```text
存在しない key を読むことは KVS の正常な問い合わせ結果です。失敗理由があるわけではなく、値がないという状態なので NOT_FOUND を返します。
```

補足:

```text
ファイルが読めない、WAL に書けない、command が不正、などは Error です。
```

## 4. WAL 書き込み失敗時に Store を更新しない理由

回答例:

```text
Store を先に更新して WAL 書き込みに失敗すると、メモリ上では成功したように見えても、再起動後にその変更が消えます。復旧性を保証する設計では、WAL 書き込みに失敗した操作を成功扱いにしません。
```

## 5. Arc<Mutex<AppState>> の限界

回答例:

```text
全状態が 1 つの lock に集まるため、WAL 書き込みや /keys の処理中に他の command が待ちます。単純で学習しやすい一方、読み取りの多い負荷や遅い I/O に弱いです。
```

改善候補:

```text
Store と Metrics の lock を分ける。Store 専用スレッドへ channel で Command を送る。tokio/axum へ移す。
```

## 6. admin HTTP を std-only で実装する限界

回答例:

```text
HTTP parser が最小実装なので、method、header、body、keep-alive、timeout、request size limit、TLS、認証を十分に扱えません。学習にはよいですが、本番では axum/hyper や reverse proxy を検討します。
```

## 7. tokio や axum に移すなら、どの責任を移しますか

回答例:

```text
tokio へ非同期 I/O と task scheduling を移します。axum へ admin HTTP の routing、request parsing、response generation を移します。Command/Store/Response/WAL のドメイン設計は、できるだけ crate 内に残します。
```

## 8. unsafe はこの project に必要ですか

回答例:

```text
不要です。TcpListener、HashMap、Mutex、File I/O、String parsing は safe Rust で十分に書けます。unsafe を使って速くする前に、lock 範囲、I/O buffering、protocol、storage 設計を見直すべきです。
```

もし使うなら:

```text
なぜ safe Rust では足りないか、unsafe block の安全条件、テスト、レビュー手順を文書化する必要があります。
```

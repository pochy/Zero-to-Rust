# Rust Mastery Assessment

この評価表は、Zero to Rust を終えたあとに自分の理解を確認するためのものです。

## Level A: 基礎を説明できる

```text
String と &str の違いを説明できる
所有権移動後に値が使えない理由を説明できる
Result と Option の違いを説明できる
unwrap を使う前に根拠を書ける
```

合格条件:

```text
Level 0-2 の進級チェックに自分の言葉で答えられる
```

## Level B: 小さい設計ができる

```text
struct が何を所有するか決められる
enum でコマンドやレスポンスを表現できる
I/O、検索、表示を分けられる
parse error と state result を分けられる
```

合格条件:

```text
Level 3-5 の exercises を自力で変更できる
```

## Level C: 品質と並行処理を説明できる

```text
正常系と異常系のテストを書ける
panic と Result の使い分けを説明できる
Arc と Mutex の役割を分けて説明できる
ロック中に I/O しない理由を説明できる
```

合格条件:

```text
cargo test --workspace と cargo clippy --workspace --all-targets を通せる
```

## Level D: 本番寄りの責任を設計できる

```text
WAL 書き込み順序を説明できる
復旧手順を書ける
metrics と health check の意味を説明できる
std-only の限界を説明できる
```

合格条件:

```text
projects/final_kvs_server を起動し、TCP と admin endpoint を確認できる
```

## Level E: 実務 Rust の判断ができる

```text
trait/generics/dyn trait の使い分けを説明できる
async と thread の違いを説明できる
unsafe を避ける理由と使う条件を説明できる
serde/tokio/clap/tracing/thiserror/anyhow の採用理由を書ける
```

合格条件:

```text
projects/kvs_std と projects/kvs_ecosystem を比較し、どの責任を crate に任せたか説明できる
```

## 最終口頭試問

次の問いに、コード参照つきで答えてください。

```text
1. Store は何を所有していますか。
2. parse_command はなぜ &str を受け取りますか。
3. GET missing はなぜ Error ではありませんか。
4. WAL 書き込み失敗時に Store を更新しない理由は何ですか。
5. Arc<Mutex<AppState>> の限界は何ですか。
6. admin HTTP を std-only で実装する限界は何ですか。
7. tokio や axum に移すなら、どの責任を移しますか。
8. unsafe はこの project に必要ですか。不要ならなぜですか。
```

## Advanced Track へ進む条件

[ADVANCED_TRACK.md](ADVANCED_TRACK.md) へ進む前に、次を 1 つ選んで理由を書いてください。

```text
async Web service へ進む
advanced types / lifetimes へ進む
procedural macro へ進む
unsafe / FFI へ進む
embedded / no_std へ進む
performance profiling へ進む
data systems へ進む
library design へ進む
```

選んだ領域について、次を説明できれば Advanced Track へ進めます。

```text
本編で学んだどの判断を使い回せるか
新しく学ぶ必要がある責任は何か
どの品質ゲートを追加すべきか
```

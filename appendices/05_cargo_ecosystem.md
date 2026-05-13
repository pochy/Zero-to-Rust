# 05: Cargo、workspace、ecosystem

## 目的

Rust の実務開発は Cargo なしでは成立しません。Cargo はビルドツール、依存管理、テスト実行、workspace 管理、公開単位の中心です。

## package、crate、module

```text
package:
Cargo.toml を持つ配布単位。

crate:
コンパイル単位。lib crate や bin crate がある。

module:
crate 内の名前空間。mod、pub、use で整理する。
```

この 3 つを混同すると、ファイル分割や公開 API の判断が曖昧になります。

## workspace

複数 package を一緒に扱う単位です。

```toml
[workspace]
resolver = "3"
members = ["projects/kvs_std", "projects/kvs_ecosystem"]
```

workspace では `cargo test --workspace` のように全 member をまとめて確認できます。学習用には、std-only 版と ecosystem 版を並べるのに向いています。

## edition 2024

新しい Cargo プロジェクトでは 2024 edition が標準です。既存の Level 例は `rustc --edition=2021` で動かしますが、Cargo project では `edition = "2024"` を使います。

edition は「Rust のバージョン」ではなく、互換性を保ちながら構文や一部ルールを切り替える仕組みです。

## feature flags

feature は依存や機能を条件付きで有効にする仕組みです。

```toml
[features]
default = ["json"]
json = ["serde", "serde_json"]
```

ただし、feature は公開 API とビルド組み合わせを増やします。便利だから増やすのではなく、ユーザーが本当に選ぶ必要がある境界に限定します。

## 主要クレートを採用する判断

| クレート | 役割 | 採用判断 |
| --- | --- | --- |
| `serde` | serialize / deserialize | wire format や設定を安定して扱う |
| `clap` | CLI parser | 引数、help、validation を自前実装しない |
| `tokio` | async runtime | 大量接続や async ecosystem に入る |
| `tracing` | structured logging | 本番でログを検索、集約、関連付ける |
| `thiserror` | library error | enum エラーを読みやすく実装する |
| `anyhow` | application error | CLI や bin の上位で文脈付き失敗を扱う |

## 採用しない判断も設計

外部クレートを使わないことは美徳ではありません。逆に、何でも依存することも設計放棄です。

```text
学習目的:
std で仕組みを理解する。

実務目的:
信頼できる crate に任せ、保守責任を分ける。

セキュリティ目的:
TLS、暗号、HTTP parser は成熟した crate や外部コンポーネントを優先する。
```

## 進級チェック

```text
package / crate / module の違いを説明できるか
workspace を使う理由を説明できるか
edition と compiler version の違いを説明できるか
外部クレートを採用する理由と採用しない理由を書けるか
```

## 公式 docs

- https://doc.rust-lang.org/cargo/
- https://doc.rust-lang.org/cargo/reference/workspaces.html
- https://doc.rust-lang.org/edition-guide/editions/creating-a-new-project.html

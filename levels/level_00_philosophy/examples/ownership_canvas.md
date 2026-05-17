# Ownership Canvas

Rust のコードを書く前に、次の問いを埋めてください。正解を急ぐための表ではなく、設計の曖昧さを見つけるための表です。

## 題材

```text
ファイルを読み込み、内容を検索し、一致した行を表示する CLI
```

## 1. データはどこで生まれるか

```text
例:
コマンドライン引数として path と pattern が渡される。
ファイル内容は std::fs::read_to_string で生成される。
検索結果は search 関数の中で作られる。
```

あなたの回答:

```text

```

## 2. 誰が所有するか

```text
例:
読み込んだファイル内容の String は load_file が作り、呼び出し元へ返す。
検索結果の Vec<MatchLine> は search が作り、表示層へ渡す。
```

あなたの回答:

```text

```

## 3. 誰が借りるだけでよいか

```text
例:
検索関数は pattern を &str として借りるだけでよい。
表示関数は MatchLine の参照を読むだけでよい。
```

あなたの回答:

```text

```

## 4. どこで失敗するか

```text
例:
引数が足りない。
ファイルが存在しない。
権限がない。
ファイルが UTF-8 ではない。
```

あなたの回答:

```text

```

## 5. 失敗をどこで扱うか

```text
例:
load_file は io::Error を返す。
main はユーザー向けメッセージを表示する。
検索結果 0 件はエラーではなく正常な結果として扱う。
```

あなたの回答:

```text

```

## 6. 後で並行処理するなら何を共有するか

```text
例:
検索対象ファイル一覧はワーカーへ分配する。
結果集約用 Vec は共有せず、チャンネルで main へ送る。
```

あなたの回答:

```text

```

## 7. 設計判断メモ

```text
値を複製してよい理由:

失敗をここで止めてよい理由:

失敗を呼び出し元へ返すべき理由:

後で並行処理するときに共有しそうな状態:
```

## 次に読む

- 前へ: [levels/level_00_philosophy/README.md](../README.md)
- 次へ: [levels/level_00_philosophy/exercises.md](../exercises.md)
- 関連: [docs/guide/CHECKPOINTS.md](../../../docs/guide/CHECKPOINTS.md), [docs/guide/STUDY_JOURNAL.md](../../../docs/guide/STUDY_JOURNAL.md)

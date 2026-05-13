# Solutions

このフォルダは、演習の模範回答とレビュー観点をまとめます。

模範回答は丸写し用ではありません。Rust では、同じ問題に複数の妥当な設計があります。ここでは、答えそのものよりも、判断理由の書き方を重視します。

## 読み方

```text
1. 先に自分で exercises.md に答える。
2. solutions を読む。
3. 答えが違う場合、どちらの責任分担が明確か比較する。
4. 最後に REVIEW_CHECKLIST.md で自己レビューする。
```

## 一覧

| ファイル | 対応 |
| --- | --- |
| [levels_00_04.md](levels_00_04.md) | Level 0-4 の回答例 |
| [levels_05_09.md](levels_05_09.md) | Level 5-9 の回答例 |
| [final_project.md](final_project.md) | final_kvs_server の回答例 |
| [assessment_answers.md](assessment_answers.md) | ASSESSMENT.md の口頭試問回答例 |

## 良い回答の条件

```text
所有者を明記している
借用でよい理由を説明している
失敗をどこで扱うか書いている
clone / unwrap / Arc<Mutex<T>> を使う理由を書いている
将来の変更時に何が壊れにくいか説明している
```

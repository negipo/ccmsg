---
name: ccmsg-sending
description: 他の Claude Code セッション（リポジトリ）へメッセージを送る。ユーザが「○○ に『××』と送って」「○○ に伝えて」等と言ったときに使う。
---

# ccmsg: 送信

他リポジトリのセッション宛にメッセージを送る。

```bash
ccmsg send --to <宛先ディレクトリ名> --body "<本文>" --project "$CLAUDE_PROJECT_DIR"
```

- 送信元（from）は `--project` の basename を CLI が算出する
- 宛先は相手のプロジェクトルートのディレクトリ名
- 宛先が宛先簿に未登録だとエラーで停止する。その場合、相手に一度 `/ccmsg`（受信確認）を実行して参加してもらえば送れるようになる
- 宛先がわからないときは ccmsg-listing-peers skill で一覧を確認する

## 注意

- 自分自身宛・空メッセージは送信できない
- identity 衝突が検出された場合、誤配信防止のため送信は中止される

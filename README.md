# ccmsg

ローカルマシン上の複数リポジトリで動く Claude Code セッション間で、短いメッセージを送受信する最小限のツール。デーモンもネットワークもフックも持たず、ローカルの SQLite ファイル一つを共有する。

## インストール

```bash
cargo install --path .
ccmsg install   # skill を ~/.claude/skills/ へ配置
```

## 使い方

ユーザは CLI を直接叩かず、Claude への自然言語または `/ccmsg` で操作する。

- `/ccmsg`、「メッセージ来てる?」 — 自分宛の未読を確認（同時に参加）
- 「○○ に『××』と送って」 — メッセージを送信
- 「誰とやり取りできる?」 — 既知の宛先一覧
- 「返事を待って」 — 新着が届くまで待ち受け

## 仕組み

- エージェントの識別子はプロジェクトルートのディレクトリ名（basename）
- 1 リポジトリにつき 1 エージェント、同時 1 セッションを前提とする
- データは `~/.local/share/ccmsg/messages.db`（SQLite, WAL）に置く
- フックは登録しない。`~/.claude/settings.json` を一切触らない

## アンインストール

```bash
ccmsg uninstall
```

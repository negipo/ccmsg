---
name: ccmsg-listing-peers
description: ccmsg でやり取りできる相手（既知の宛先）の一覧を表示する。ユーザが「誰とやり取りできる?」「相手一覧」等と言ったときに使う。
---

# ccmsg: 宛先一覧

ccmsg に参加済みの既知の宛先（ディレクトリ名）を一覧する。

```bash
ccmsg list
```

- 一度でも ccmsg を使った（`/ccmsg` を実行した、または送受信した）リポジトリが宛先簿に載る
- ここに出る名前を `ccmsg send --to <名前>` の宛先に使う

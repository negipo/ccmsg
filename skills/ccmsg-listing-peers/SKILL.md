---
name: ccmsg-listing-peers
description: List the peers (known destinations) you can message with ccmsg. Use when the user says things like "who can I message?" or "list peers".
---

# ccmsg: List Peers

List the known peers (directory names) that have joined ccmsg.

```bash
ccmsg list
```

- A repository appears in the peer directory once it has used ccmsg (ran `/ccmsg`, or sent/received a message).
- Use the names shown here as the `ccmsg send --to <name>` destination.

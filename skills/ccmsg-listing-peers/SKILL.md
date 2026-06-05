---
name: ccmsg-listing-peers
description: List the peers (known destinations) you can message with ccmsg. Use when the user says things like "who can I message?" or "list peers".
---

# ccmsg: List Peers

List the known peers (directory names) that have joined ccmsg.

```bash
ccmsg list
```

- Run `list` with no arguments. Unlike the other subcommands it takes no `--project` (or any
  other) flag; passing one makes it exit with an error.
- A repository appears in the peer directory once it has used ccmsg (ran `/ccmsg`, or sent/received a message).
- Use the names shown here as the `ccmsg send --to <name>` destination.

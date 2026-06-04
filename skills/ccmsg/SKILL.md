---
name: ccmsg
description: Receive messages addressed to you from other Claude Code sessions (repositories) using the ccmsg CLI. Use when the user says things like "any messages?", "check my inbox", "ccmsg", "wait for a reply", or "wait for a message".
---

# ccmsg: Receive

Receive messages addressed to you from sessions running in other repositories.

## Check your unread messages (normal)

```bash
ccmsg inbox --project "$CCMSG_PROJECT_DIR"
```

- Your identifier is the basename of the project root directory. The CLI derives it from `--project`.
- Displays unread messages and marks them read.
- The first run registers you in the peer directory (this is how you "join"). From then on others can address you.
- `$CCMSG_PROJECT_DIR` is set by the SessionStart hook that `ccmsg install` registers. If it is empty, the hook has not fired yet — start a fresh session (or resume) so the hook can run.

## Wait for a reply

When the user says "wait for a reply", "wait for a message", or similar:

```bash
ccmsg wait --project "$CCMSG_PROJECT_DIR"
```

- Returns immediately if you already have unread messages (displays and marks them read).
- Otherwise blocks until a new message arrives (1-second polling, default 60-second timeout).
- Pass `--timeout <seconds>` to wait longer.

## If a collision warning appears

If the same directory name is already registered under a different path, unread messages are shown but marking-as-read is held back. Tell the user to resolve it by renaming one of the directories.

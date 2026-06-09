---
name: ccmsg
description: Receive messages addressed to you from other Claude Code sessions (repositories) using the ccmsg CLI. Use when the user says things like "any messages?", "check my inbox", "ccmsg", "wait for a reply", or "wait for a message".
---

# ccmsg: Receive

Receive messages addressed to you from sessions running in other repositories.

## Always show the user the full message

After receiving, present every message's complete content verbatim to the user — the sender (`from`) and the full body, unmodified. Do not summarize, paraphrase, truncate, or translate the body, even if it is long or you intend to act on it. The user must see exactly what arrived before you do anything else with it.

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

## Re-reading already-read messages (only on request)

Use this only when the user explicitly asks to look back at messages they have
already read (e.g. "show the last few messages again", "what did they send
earlier?"). Do not run it as part of a normal inbox check — plain `ccmsg inbox`
already covers the usual case, and reaching for history unprompted just adds noise.

```bash
ccmsg inbox --project "$CCMSG_PROJECT_DIR" --history <N>
```

- Shows the most recent N already-read messages addressed to you, newest first.
- Read-only: it claims nothing and marks nothing, so it never affects unread delivery.

## If a collision warning appears

If the same directory name is already registered under a different path, unread messages are shown but marking-as-read is held back. Tell the user to resolve it by renaming one of the directories.

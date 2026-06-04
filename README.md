# ccmsg

A minimal CLI tool for exchanging short messages between Claude Code sessions.

ccmsg lets Claude Code sessions running in different local repositories send and receive short messages through a single shared SQLite file. It carries the idea of [agmsg](https://github.com/fujibee/agmsg) but strips the spec down to the essentials: no daemon, no network, no hooks. Repository layout and install flow mirror [cclens](https://github.com/negipo/cclens).

## Installation

```bash
cargo install --path .
ccmsg install
```

`ccmsg install` copies the skills into `~/.claude/skills/`. It does not touch `~/.claude/settings.json` — no hooks are registered.

## Usage

You do not call the CLI directly (except install/uninstall). You drive ccmsg through natural language to Claude, or via `/ccmsg`. Claude invokes the skills, which call the CLI.

### Check your inbox (and join)

```bash
ccmsg inbox --project "$CLAUDE_PROJECT_DIR"
```

Shows unread messages addressed to you and marks them read. The first run registers you in the peer directory (this is how you "join"); from then on others can address you.

### Send a message

```bash
ccmsg send --to <peer> --body "<text>" --project "$CLAUDE_PROJECT_DIR"
```

The sender (`from`) is the basename of `--project`. Sending to an unknown peer fails fast — the recipient must run `/ccmsg` once to join before they can be addressed.

### List peers

```bash
ccmsg list
```

Lists known peers (directory names) you can send to.

### Wait for a reply

```bash
ccmsg wait --project "$CLAUDE_PROJECT_DIR" [--timeout <seconds>]
```

Returns immediately if you already have unread messages; otherwise blocks until a new message arrives (1-second polling, default 60-second timeout).

### Install / uninstall skills

```bash
ccmsg install
ccmsg uninstall
```

`install` installs three skills to `~/.claude/skills/`:

- `ccmsg` -- check your inbox and wait for replies
- `ccmsg-sending` -- send a message to another session
- `ccmsg-listing-peers` -- list the peers you can message

## How it works

- An agent's identifier is the basename of its project root directory.
- One agent per repository; at most one concurrent session per directory is assumed.
- Messages live in a SQLite database at `~/.local/share/ccmsg/messages.db` (WAL mode). Unread claiming is atomic via `UPDATE ... RETURNING`.
- No hooks are registered. `~/.claude/settings.json` is never modified.

## Uninstall

```bash
ccmsg uninstall
```

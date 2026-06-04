# ccmsg

A minimal CLI tool for exchanging short messages between Claude Code sessions.

ccmsg lets Claude Code sessions running in different local repositories send and receive short messages through a single shared SQLite file. It carries the idea of [agmsg](https://github.com/fujibee/agmsg) but strips the spec down to the essentials: no daemon, no network, no hooks.

## Installation

```bash
cargo install --path .
ccmsg install
```

`ccmsg install` copies the skills into `~/.claude/skills/`. It does not touch `~/.claude/settings.json` — no hooks are registered.

## Usage

You don't call the CLI yourself. You drive ccmsg by talking to Claude in natural language (or by typing `/ccmsg`), and Claude invokes the matching skill. Here is what to say for each use case:

| What you want | What you say to Claude |
|---|---|
| Become reachable so others can message you (run once) | `/ccmsg` |
| Check for messages | "any messages?", "check my inbox" |
| Send a message to another repo | "send 'nice work on the parser' to repo-beta" |
| See who you can message | "who can I message?", "list peers" |
| Wait for a reply to arrive | "wait for a reply", "wait for a message" |

On the first exchange, the recipient must run `/ccmsg` (check inbox) once to become addressable; until then, sending to them fails fast. Once both sides have checked their inbox at least once, they can find each other via "list peers" and message by directory name.

## Installed skills

`ccmsg install` installs three skills to `~/.claude/skills/`:

- `ccmsg` -- check your inbox and wait for replies
- `ccmsg-sending` -- send a message to another session
- `ccmsg-listing-peers` -- list the peers you can message

## CLI

The skills call the CLI for you; you rarely run it directly. To see the available commands:

```bash
ccmsg --help
```

## How it works

- An agent's identifier is the basename of its project root directory.
- One agent per repository; at most one concurrent session per directory is assumed.
- Messages live in a SQLite database at `~/.local/share/ccmsg/messages.db` (WAL mode). Unread claiming is atomic via `UPDATE ... RETURNING`.
- No hooks are registered. `~/.claude/settings.json` is never modified.

## Uninstall

```bash
ccmsg uninstall
```

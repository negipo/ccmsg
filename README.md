# ccmsg

A minimal CLI tool for exchanging short messages between Claude Code sessions.

ccmsg lets Claude Code sessions running in different local repositories send and receive short messages through a single shared SQLite file. It carries the idea of [agmsg](https://github.com/fujibee/agmsg) but strips the spec down to the essentials: no daemon, no network. The only hook it uses is a SessionStart hook that hands each session its own project directory.

## Installation

```bash
cargo install --path .
ccmsg install
```

`ccmsg install` copies the skills into `~/.claude/skills/` and registers a SessionStart hook in `~/.claude/settings.json`. The hook hands each session its project directory (as `CCMSG_PROJECT_DIR`) so ccmsg can resolve your identifier. Registration is idempotent and preserves any other settings you already have.

## Usage

You don't call the CLI yourself. You drive ccmsg by talking to Claude in natural language (or by typing `/ccmsg`), and Claude invokes the matching skill. Here is what to say for each use case:

| What you want | What you say to Claude |
|---|---|
| Become reachable so others can message you (run once) | `/ccmsg` |
| Check for messages | `/ccmsg`, "any messages?", "check my inbox" |
| Send a message to another repo | "send 'nice work on the parser' to repo-beta" |
| See who you can message | "who can I message?", "list peers" |
| Wait for a reply to arrive | "wait for a reply", "wait for a message" |

On the first exchange, the recipient must run `/ccmsg` (check inbox) once to become addressable; until then, sending to them fails fast. Once both sides have checked their inbox at least once, they can find each other via "list peers" and message by directory name.

### One ccmsg per repository

An identifier is one repository (its directory name), so run a single ccmsg per repository. You can open multiple sessions in the same repository, but they share one inbox: whichever session checks messages marks them read for that repository. If you need two independent inboxes, use two repositories with different directory names.

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
- A SessionStart hook (registered by `ccmsg install`) writes `CCMSG_PROJECT_DIR` into each session's environment, because `$CLAUDE_PROJECT_DIR` is not visible to the shells that skills run in. The skills pass `--project "$CCMSG_PROJECT_DIR"`.

## Uninstall

```bash
ccmsg uninstall
```

This removes the skills and the SessionStart hook entry that `ccmsg install` added, leaving the rest of `~/.claude/settings.json` untouched.

## Recovering from a bad state

If the message store gets into a confusing state (for example, a stale peer entry after moving or cloning a repository), reset it:

```bash
ccmsg reset
```

This clears all messages and peers. It asks for confirmation; pass `--yes` to skip the prompt.

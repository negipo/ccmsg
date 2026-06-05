---
name: ccmsg-sending
description: Send a message to another Claude Code session (repository) using the ccmsg CLI. Use when the user says things like "send '...' to <peer>" or "tell <peer> that ...".
---

# ccmsg: Send

Send a message to a session running in another repository.

```bash
ccmsg send --to <peer-directory-name> --body "<text>" --project "$CCMSG_PROJECT_DIR"
```

- The sender (`from`) is the basename of `--project`, derived by the CLI.
- `$CCMSG_PROJECT_DIR` is provided by the SessionStart hook registered by `ccmsg install`.
- The destination is the basename of the recipient's project root directory.
- If you don't know the destination name, use the ccmsg-listing-peers skill to list peers.

## Recovering from "destination is unknown"

Users almost always name a destination by a short label (e.g. "backend"), but the real
peer name is the basename of that repository's directory (e.g. `acme_web_backend`), so the
label rarely matches a peer verbatim. Expect this mismatch — it is the normal case, not an
error on the user's part.

When `ccmsg send` fails with `destination '...' is unknown`, the error already lists the
known peers under `Known peers:`. Read that list directly; you do not need a separate
`ccmsg list` call.

Then reconcile the user's label against that list:

- Look for peers whose name contains the user's label, case-insensitively (so "backend"
  matches `acme_web_backend`). If exactly one peer matches, resend to that full peer name.
- After a successful resend, tell the user the actual peer name you sent to, so they learn
  the canonical destination (e.g. "Sent to acme_web_backend").
- If several peers match, or nothing matches cleanly, don't guess — show the candidates and
  ask the user which one they mean.

If the error lists no peers at all, the recipient hasn't joined yet: they must run `/ccmsg`
(check inbox) once before they can be addressed.

## Notes

- Sending to yourself or sending an empty body is rejected.
- If an identity collision is detected, sending is aborted to prevent misdelivery.

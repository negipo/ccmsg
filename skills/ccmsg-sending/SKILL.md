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
- If the destination is not in the peer directory, the command fails fast. In that case, the recipient must run `/ccmsg` (check inbox) once to join before they can be addressed.
- If you don't know the destination name, use the ccmsg-listing-peers skill to list peers.

## Notes

- Sending to yourself or sending an empty body is rejected.
- If an identity collision is detected, sending is aborted to prevent misdelivery.

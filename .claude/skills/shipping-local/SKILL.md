---
name: shipping-local
description: Commit and push ccmsg changes, then reinstall the CLI and skills on this machine so your local environment runs the latest version. Use when the user says things like "ship", "ship it", "release locally", "reinstall", or otherwise wants changes finalized and rolled out to their own setup.
---

# shipping-local

Finalize ccmsg changes and roll them out to the local environment, in this order:

1. Commit: use the `git-committing` skill
2. Push: use the `git-pushing` skill
3. Reinstall the CLI: `cargo install --path .`
4. Reinstall the skills: `ccmsg install`

Steps 3 and 4 rebuild the CLI and skills from the current source, so they only make sense once the build is sound — run them after commit and push. `ccmsg install` is idempotent and leaves the rest of `~/.claude/settings.json` untouched.

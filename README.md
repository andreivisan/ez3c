# EZ3C

**Easy Claude Code Chats** — a TUI utility to manage your Claude Code chats
across projects: browse, read, delete, copy, and move them.

## Motivation

Claude Code currently doesn't let you delete old chats from within the CLI,
nor view chats belonging to a different project. Chat histories accumulate
forever and can grow large. EZ3C gives you a single screen to inspect and
clean them up.

## How Claude Code stores chats

Everything lives under `~/.claude/`:

| Path | Contents |
|---|---|
| `projects/<munged-path>/<session-uuid>.jsonl` | One file per chat. Dir name is the project's absolute path with `/` replaced by `-`. |
| `projects/<munged-path>/memory/` | Per-**project** memory (not per-chat — never moved by EZ3C). |
| `file-history/<session-uuid>/` | File-edit backups, keyed by session UUID. |
| `history.jsonl` | Prompt history; each entry carries `project` and `sessionId`. |

Each `.jsonl` line is a typed JSON record. The first lines are metadata
(`mode`, `file-history-snapshot`, …) and early "user" messages are often meta
noise (`isMeta: true`, slash-command tags) — the chat preview must skip these
and find the first real user prompt.

Two gotchas EZ3C handles deliberately:

- **The dir-name munging is lossy.** `-Users-x-my-app` could be `/Users/x/my-app`
  or `/Users/x/my/app`. The true path is read from the `cwd` field inside the
  chat records, not reconstructed from the dir name.
- **The format is undocumented and version-dependent.** EZ3C treats records as
  opaque JSON, touches only the fields it must (`sessionId`, `cwd`), and fails
  safe on anything unexpected.

## Features

1. **List all chats grouped by project**, sorted by last activity. Each entry
   shows the first real user prompt (truncated), last-modified date, message
   count, and file size. Per-project disk usage is shown in the group header.

```text
~/dev/utilities/ez3c                                    3 chats · 1.2 MB
  > Help me build this TUI described in the READ…   Jun 12 · 48 msgs · 820 KB
    Refactor the parser module to use serde…        Jun 10 · 12 msgs · 310 KB

~/dev/rust/from-zero-to-edge                            7 chats · 4.8 MB
    Explain ownership and borrowing with…           Jun 02 · 95 msgs · 2.1 MB
    ...
```

2. **View a chat** read-only in a scrollable pane (ENTER).

3. **Filter** chats by preview text or project name (`/`).

4. **Delete a chat** (DEL/BACKSPACE/`d`) with a y/n confirmation. Deletion is
   permanent and also removes the chat's `file-history/<uuid>/` backups.

5. **Copy or move a chat** to another project (`c` / `m`): pick the target
   project from a list, ENTER to confirm.
   - *Move* keeps the session UUID and rewrites the embedded `cwd` fields.
   - *Copy* generates a fresh session UUID (two chats must never share one)
     and rewrites `sessionId` in every record.

## Usage

```bash
$ ez3c        # opens the TUI
```

| Key | Action |
|---|---|
| `j` / `k` / arrows | Move down / up |
| `ENTER` | Open chat viewer (in viewer: scroll; `q`/ESC to close) |
| `/` | Filter; ESC clears |
| `d` / DEL / BACKSPACE | Delete chat (asks for confirmation) |
| `c` / `m` | Copy / move chat to another project |
| `?` | Help |
| `q` | Quit |

Subcommands are reserved for future scripting use
(e.g. `ez3c clean --older-than 30d`).

## Status

In development. Not affiliated with Anthropic; this tool manipulates
undocumented local files — keep backups of `~/.claude` if your chats matter.

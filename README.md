# EZ3C

Easy Claude Code Chats is a utility TUI application that allows you to manage 
your Claude Code chats accross projects easily. 

## Motivation

At the moment Claude Code doesn't allow to remove old chats from within the CLI.
Claude Code also doesn't allow to view previous chats from within different projects.
This could be due to context building - needs investigation.

## Features

1. List all chats in the following format

```text
PROJECT 1
1. Chat-1.1 - First 100 chars
2. Chat-1.2 - First 100 chars
...

PROJECT 2
1. Chat-2.1 - First 100 chars
2. Chat-2.2 - First 100 chars
...
```

2. Remove a chat from the list by pressing DEL or BACKSPACE key

3. Copy or Move a chat from one project to another

> NOTE: If Claude Code creates a context then the context has to be moved as well.

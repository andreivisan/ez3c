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

## Usage

```bash
# list all chats
$ ez3c list

# scroll down to the desired chat 
# press DEL or BACKSPACE to delete
# press C to copy or M to move
# user is then presented with a list of projects
# scroll up or down to the desired project (the current project is the project
# which the chat belongs
# press ENTER to copy or move the chat
```

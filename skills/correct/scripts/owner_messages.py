#!/usr/bin/env python3
"""Print the owner's typed messages from one project's Claude Code sessions.

Usage: owner_messages.py [--since YYYY-MM-DD]

Reads ~/.claude/projects/<slug>/*.jsonl, where <slug> is the current
directory's absolute path with every non-alphanumeric character replaced by
"-". Keeps only messages the owner typed (origin kind "human"), including
ones queued while the agent was busy, and slash commands that carry arguments,
oldest first. Each prints as one block: the session ID prefix, the timestamp,
the tail of the agent's previous reply, and the message.
"""

import argparse
import json
import re
import signal
import sys
from datetime import date
from pathlib import Path

COMMAND = re.compile(r"<command-name>(/[^<]*)</command-name>")
ARGS = re.compile(r"<command-args>(.*?)</command-args>", re.S)
CONTEXT = 240  # characters kept from the end of the previous reply
MAX_CHARS = 600  # characters kept from the start of each message


def clip(text, limit, tail=False):
    """Collapse whitespace and keep at most limit characters."""
    text = " ".join(text.split())
    if len(text) <= limit:
        return text
    return "…" + text[-limit:] if tail else text[:limit] + "…"


def text_of(event):
    """An event's message text: a plain string, or its text blocks joined."""
    message = event.get("message")
    content = message.get("content") if isinstance(message, dict) else None
    if isinstance(content, str):
        return content
    if not isinstance(content, list):
        return ""
    return " ".join(
        str(block.get("text", "")) for block in content if isinstance(block, dict) and block.get("type") == "text"
    )


def human(origin):
    """Whether an origin record marks text the owner typed."""
    return isinstance(origin, dict) and origin.get("kind") == "human"


def owner_text(event):
    """The owner's message in an event, or None when the owner did not type it.

    A message typed while the agent is busy is stored as a queued_command
    attachment, not a user event.
    """
    if event.get("isMeta") or event.get("isSidechain"):
        return None
    attachment = event.get("attachment")
    if event.get("type") == "attachment" and isinstance(attachment, dict):
        queued = attachment.get("type") == "queued_command" and human(attachment.get("origin"))
        prompt = attachment.get("prompt")
        return prompt if queued and isinstance(prompt, str) else None
    if event.get("type") != "user":
        return None
    content = text_of(event)
    command = COMMAND.search(content)
    if command:
        args = ARGS.search(content)
        args = args.group(1).strip() if args else ""
        return f"{command.group(1)} {args}" if args else None
    return content if human(event.get("origin")) else None


def messages(path, since):
    """Yield (timestamp, previous reply, message) for one transcript."""
    previous = ""
    with path.open(encoding="utf-8", errors="replace") as lines:
        for line in lines:
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            if not isinstance(event, dict):
                continue
            if event.get("type") == "assistant" and not event.get("isSidechain"):
                text = text_of(event)
                if text.strip():
                    previous = text
                continue
            text = owner_text(event)
            stamp = str(event.get("timestamp", ""))
            if text and text.strip() and stamp[:10] >= since:
                yield stamp, previous, text


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--since", type=date.fromisoformat, help="earliest date to include, YYYY-MM-DD")
    options = parser.parse_args()
    if hasattr(signal, "SIGPIPE"):
        signal.signal(signal.SIGPIPE, signal.SIG_DFL)  # exit quietly when piped to head

    slug = re.sub(r"[^A-Za-z0-9]", "-", str(Path.cwd()))
    folder = Path.home() / ".claude" / "projects" / slug
    if not folder.is_dir():
        sys.exit(f"no transcripts at {folder}")

    since = options.since.isoformat() if options.since else ""
    found = []
    for path in folder.glob("*.jsonl"):
        for stamp, previous, text in messages(path, since):
            found.append((stamp, path.stem[:8], previous, text))
    for stamp, session, previous, text in sorted(found):
        print(f"[{session} {stamp[:16]}]")
        if previous:
            print(f"  agent: {clip(previous, CONTEXT, tail=True)}")
        print(f"  owner: {clip(text, MAX_CHARS)}")
    if not found:
        print(f"no owner messages in {folder}", file=sys.stderr)


if __name__ == "__main__":
    main()

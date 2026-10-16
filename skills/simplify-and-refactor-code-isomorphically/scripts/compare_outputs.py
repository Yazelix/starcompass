#!/usr/bin/env python3
"""Byte-compare two files or directory trees for isomorphic refactor verification."""

from __future__ import annotations

import filecmp
import hashlib
import os
import sys
from pathlib import Path


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def files_under(root: Path) -> dict[str, Path]:
    if root.is_file():
        return {"<file>": root}
    return {
        str(path.relative_to(root)): path
        for path in root.rglob("*")
        if path.is_file()
    }


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: compare_outputs.py <before-file-or-dir> <after-file-or-dir>", file=sys.stderr)
        return 2

    before = Path(sys.argv[1])
    after = Path(sys.argv[2])
    if not before.exists() or not after.exists():
        print("missing input path", file=sys.stderr)
        return 2

    before_files = files_under(before)
    after_files = files_under(after)
    before_keys = set(before_files)
    after_keys = set(after_files)

    missing = sorted(before_keys - after_keys)
    extra = sorted(after_keys - before_keys)
    changed: list[str] = []

    for key in sorted(before_keys & after_keys):
        left = before_files[key]
        right = after_files[key]
        if not filecmp.cmp(left, right, shallow=False):
            changed.append(key)

    if not missing and not extra and not changed:
        print("MATCH")
        for key in sorted(before_keys):
            print(f"{key}\t{digest(before_files[key])}")
        return 0

    print("DIFFER")
    for key in missing:
        print(f"missing_after\t{key}")
    for key in extra:
        print(f"extra_after\t{key}")
    for key in changed:
        print(f"changed\t{key}\t{digest(before_files[key])}\t{digest(after_files[key])}")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())

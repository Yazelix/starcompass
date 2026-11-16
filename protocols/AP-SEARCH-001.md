---
id: AP-SEARCH-001
version: 1
title: Search narrowly
---

Search inside one named folder. Never sweep `/`, a home directory, a cache
root, or a folder of shared agent work trees, and never run `du` on them: one
sweep can stall the whole machine. When a guard blocks a search, narrow it;
never route around the guard. Hosts without a pre-tool hook, such as Codex,
rely on this rule alone.

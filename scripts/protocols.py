#!/usr/bin/env python3
"""Index, render, and verify agent-protocol imports."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from datetime import date
from pathlib import Path
from typing import Any, Sequence


ROOT = Path(__file__).resolve().parents[1]
SOURCE_REPOSITORY = "https://github.com/luccahuguet/agent-protocols"
PROTOCOL_ID = re.compile(r"AP-[A-Z][A-Z0-9-]*-[0-9]{3}")
COMMIT_ID = re.compile(r"[0-9a-f]{40}")
FRONTMATTER_FIELDS = {"id", "version", "title"}
PROFILE_FIELDS = {"schema", "name", "description", "protocols"}
EXCEPTION_REQUIRED = {"protocol", "scope", "reason", "approved_by", "approved_at"}
EXCEPTION_ALLOWED = EXCEPTION_REQUIRED | {"review_condition"}


class ProtocolError(Exception):
    """A user-actionable protocol repository or import error."""


@dataclass(frozen=True)
class Protocol:
    id: str
    version: int
    title: str
    path: str
    sha256: str
    body: str

    def import_record(self) -> dict[str, Any]:
        return {"id": self.id, "version": self.version, "sha256": self.sha256}

    def index_record(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "version": self.version,
            "title": self.title,
            "path": self.path,
            "sha256": self.sha256,
        }


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as error:
        raise ProtocolError(f"missing file: {path}") from error
    except json.JSONDecodeError as error:
        raise ProtocolError(f"invalid JSON in {path}: {error}") from error


def json_text(value: Any) -> str:
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def atomic_write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        dir=path.parent, prefix=f".{path.name}."
    )
    try:
        os.fchmod(descriptor, 0o644)
        with os.fdopen(descriptor, "w", encoding="utf-8", newline="\n") as temporary:
            temporary.write(content)
        os.replace(temporary_name, path)
    except BaseException:
        try:
            os.unlink(temporary_name)
        except FileNotFoundError:
            pass
        raise


def parse_protocol(root: Path, path: Path) -> Protocol:
    raw = path.read_bytes()
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as error:
        raise ProtocolError(f"protocol is not UTF-8: {path}") from error
    if not text.startswith("---\n"):
        raise ProtocolError(f"protocol lacks opening frontmatter delimiter: {path}")
    boundary = text.find("\n---\n", 4)
    if boundary < 0:
        raise ProtocolError(f"protocol lacks closing frontmatter delimiter: {path}")

    metadata: dict[str, str] = {}
    for line in text[4:boundary].splitlines():
        key, separator, value = line.partition(":")
        if not separator or not key.strip() or not value.strip():
            raise ProtocolError(f"invalid frontmatter line in {path}: {line!r}")
        key = key.strip()
        if key in metadata:
            raise ProtocolError(f"duplicate frontmatter field {key!r} in {path}")
        metadata[key] = value.strip()
    if set(metadata) != FRONTMATTER_FIELDS:
        raise ProtocolError(
            f"frontmatter fields in {path} must be {sorted(FRONTMATTER_FIELDS)}"
        )

    protocol_id = metadata["id"]
    if not PROTOCOL_ID.fullmatch(protocol_id):
        raise ProtocolError(f"invalid protocol ID {protocol_id!r} in {path}")
    if path.name != f"{protocol_id}.md":
        raise ProtocolError(f"protocol filename must be {protocol_id}.md: {path}")
    try:
        version = int(metadata["version"])
    except ValueError as error:
        raise ProtocolError(f"protocol version must be an integer in {path}") from error
    if version < 1:
        raise ProtocolError(f"protocol version must be positive in {path}")
    body = text[boundary + 5 :].strip()
    if not body:
        raise ProtocolError(f"protocol body is empty: {path}")
    return Protocol(
        id=protocol_id,
        version=version,
        title=metadata["title"],
        path=path.relative_to(root).as_posix(),
        sha256=digest(raw),
        body=body,
    )


def load_protocols(root: Path) -> dict[str, Protocol]:
    protocol_dir = root / "protocols"
    paths = sorted(protocol_dir.glob("*.md"))
    if not paths:
        raise ProtocolError(f"no protocols found in {protocol_dir}")
    protocols: dict[str, Protocol] = {}
    for path in paths:
        protocol = parse_protocol(root, path)
        if protocol.id in protocols:
            raise ProtocolError(f"duplicate protocol ID: {protocol.id}")
        protocols[protocol.id] = protocol
    return protocols


def expected_index(root: Path) -> dict[str, Any]:
    protocols = load_protocols(root)
    return {
        "schema": 1,
        "protocols": [protocols[key].index_record() for key in sorted(protocols)],
    }


def load_profiles(
    root: Path, protocols: dict[str, Protocol]
) -> dict[str, dict[str, Any]]:
    profile_dir = root / "profiles"
    paths = sorted(profile_dir.glob("*.json"))
    if not paths:
        raise ProtocolError(f"no profiles found in {profile_dir}")
    profiles: dict[str, dict[str, Any]] = {}
    used_protocols: set[str] = set()
    for path in paths:
        profile = read_json(path)
        if not isinstance(profile, dict) or set(profile) != PROFILE_FIELDS:
            raise ProtocolError(
                f"profile fields in {path} must be {sorted(PROFILE_FIELDS)}"
            )
        name = profile["name"]
        if not isinstance(name, str) or path.name != f"{name}.json":
            raise ProtocolError(f"profile name must match filename: {path}")
        if profile["schema"] != 1:
            raise ProtocolError(f"unsupported profile schema in {path}")
        if (
            not isinstance(profile["description"], str)
            or not profile["description"].strip()
        ):
            raise ProtocolError(f"profile description is empty: {path}")
        ids = profile["protocols"]
        if (
            not isinstance(ids, list)
            or not ids
            or not all(isinstance(item, str) for item in ids)
        ):
            raise ProtocolError(
                f"profile protocols must be a non-empty string list: {path}"
            )
        if len(ids) != len(set(ids)):
            raise ProtocolError(f"profile contains duplicate protocol IDs: {path}")
        unknown = sorted(set(ids) - protocols.keys())
        if unknown:
            raise ProtocolError(
                f"profile {name} references unknown protocols: {unknown}"
            )
        for required in ("AP-SCOPE-001", "AP-EXCEPTION-001"):
            if required not in ids:
                raise ProtocolError(f"profile {name} must include {required}")
        if name in profiles:
            raise ProtocolError(f"duplicate profile name: {name}")
        profiles[name] = profile
        used_protocols.update(ids)
    unused = sorted(protocols.keys() - used_protocols)
    if unused:
        raise ProtocolError(f"protocols are not included in any profile: {unused}")
    return profiles


def validate_repository(
    root: Path,
) -> tuple[dict[str, Protocol], dict[str, dict[str, Any]]]:
    protocols = load_protocols(root)
    actual_index = read_json(root / "protocols" / "index.json")
    wanted_index = {
        "schema": 1,
        "protocols": [protocols[key].index_record() for key in sorted(protocols)],
    }
    if actual_index != wanted_index:
        raise ProtocolError(
            "protocols/index.json is stale; run scripts/protocols.py index"
        )
    profiles = load_profiles(root, protocols)
    return protocols, profiles


def validate_version_transition(previous: Any, current: dict[str, Any]) -> None:
    if not isinstance(previous, dict) or previous.get("schema") != 1:
        raise ProtocolError("base revision has an invalid protocol index")
    try:
        before = {item["id"]: item for item in previous["protocols"]}
        after = {item["id"]: item for item in current["protocols"]}
    except (KeyError, TypeError) as error:
        raise ProtocolError("base revision has an invalid protocol index") from error
    removed = sorted(before.keys() - after.keys())
    if removed:
        raise ProtocolError(
            f"protocol IDs are permanent and cannot be removed: {removed}"
        )
    for protocol_id in sorted(before.keys() & after.keys()):
        old = before[protocol_id]
        new = after[protocol_id]
        if new["version"] < old["version"]:
            raise ProtocolError(f"{protocol_id} version regressed")
        if new["sha256"] != old["sha256"] and new["version"] <= old["version"]:
            raise ProtocolError(f"{protocol_id} changed without a version bump")


def validate_exceptions(value: Any, selected: set[str]) -> list[dict[str, str]]:
    if value is None:
        return []
    if not isinstance(value, dict) or set(value) != {"schema", "exceptions"}:
        raise ProtocolError(
            "exceptions document must contain only schema and exceptions"
        )
    if value["schema"] != 1 or not isinstance(value["exceptions"], list):
        raise ProtocolError(
            "exceptions document must use schema 1 and an exceptions list"
        )
    result: list[dict[str, str]] = []
    identities: set[tuple[str, str]] = set()
    for number, item in enumerate(value["exceptions"], 1):
        if not isinstance(item, dict):
            raise ProtocolError(f"exception {number} must be an object")
        fields = set(item)
        if not EXCEPTION_REQUIRED <= fields or not fields <= EXCEPTION_ALLOWED:
            raise ProtocolError(
                f"exception {number} requires {sorted(EXCEPTION_REQUIRED)} and permits review_condition"
            )
        if not all(isinstance(item[key], str) and item[key].strip() for key in fields):
            raise ProtocolError(f"exception {number} fields must be non-empty strings")
        if item["protocol"] not in selected:
            raise ProtocolError(
                f"exception {number} targets a protocol outside the profile"
            )
        try:
            date.fromisoformat(item["approved_at"])
        except ValueError as error:
            raise ProtocolError(
                f"exception {number} approved_at must be YYYY-MM-DD"
            ) from error
        identity = (item["protocol"], item["scope"])
        if identity in identities:
            raise ProtocolError(f"duplicate exception scope for {item['protocol']}")
        identities.add(identity)
        result.append({key: item[key] for key in sorted(fields)})
    return result


def assemble_import(
    root: Path,
    profile_names: Sequence[str],
    overlay: str,
    exceptions_document: Any,
    source_repository: str,
    source_commit: str,
) -> tuple[dict[str, Any], str]:
    protocols, profiles = validate_repository(root)
    if (
        isinstance(profile_names, (str, bytes))
        or not profile_names
        or not all(isinstance(name, str) and name for name in profile_names)
        or len(profile_names) != len(set(profile_names))
    ):
        raise ProtocolError("choose one or more unique profiles")
    if not isinstance(source_repository, str) or not source_repository.strip():
        raise ProtocolError("source repository must be a non-empty string")
    unknown_profiles = sorted(set(profile_names) - profiles.keys())
    if unknown_profiles:
        raise ProtocolError(
            f"unknown profiles {unknown_profiles}; choose from {sorted(profiles)}"
        )
    if not COMMIT_ID.fullmatch(source_commit):
        raise ProtocolError(
            "source commit must be a full lowercase 40-character Git SHA"
        )
    selected_ids: list[str] = []
    for profile_name in profile_names:
        for protocol_id in profiles[profile_name]["protocols"]:
            if protocol_id not in selected_ids:
                selected_ids.append(protocol_id)
    selected = [protocols[protocol_id] for protocol_id in selected_ids]
    exceptions = validate_exceptions(
        exceptions_document, {item.id for item in selected}
    )
    manifest = {
        "schema": 1,
        "source": {"repository": source_repository, "commit": source_commit},
        "profiles": list(profile_names),
        "protocols": [item.import_record() for item in selected],
        "exceptions": exceptions,
        "overlay": {"sha256": digest(overlay.encode("utf-8"))},
    }
    return manifest, render_agents(manifest, selected, overlay)


def markdown_cell(value: str) -> str:
    return value.replace("|", "\\|").replace("\n", " ")


def render_agents(
    manifest: dict[str, Any], protocols: Sequence[Protocol], overlay: str
) -> str:
    source = manifest["source"]
    lines = [
        "# Agent Guidelines",
        "",
        "This file is self-contained. Canonical protocol text was rendered into it;",
        "the source repository is needed only to update or verify the import.",
        "",
        "## Protocol import record",
        "",
        f"- Source: `{source['repository']}`",
        f"- Source commit: `{source['commit']}`",
        f"- Profiles: {', '.join(f'`{name}`' for name in manifest['profiles'])}",
        "- Manifest: `.agent-protocols.json` (schema 1)",
        "",
        "| Protocol | Version | SHA-256 |",
        "| --- | ---: | --- |",
    ]
    for item in manifest["protocols"]:
        lines.append(f"| `{item['id']}` | {item['version']} | `{item['sha256']}` |")
    lines.extend(["", "### Local exceptions", ""])
    if manifest["exceptions"]:
        for item in manifest["exceptions"]:
            review = item.get("review_condition", "none recorded")
            lines.extend(
                [
                    f"- `{item['protocol']}` in {markdown_cell(item['scope'])}: "
                    f"{markdown_cell(item['reason'])}",
                    f"  Approved by {markdown_cell(item['approved_by'])} on "
                    f"{item['approved_at']}; review condition: {markdown_cell(review)}.",
                ]
            )
    else:
        lines.append("No local exceptions.")
    lines.extend(["", "## Canonical protocols", ""])
    for protocol in protocols:
        lines.extend(
            [
                f"### {protocol.id} — {protocol.title}",
                "",
                protocol.body,
                "",
            ]
        )
    lines.extend(
        [
            "## Repository-local rules",
            "",
            overlay.strip() or "No additional local rules.",
            "",
        ]
    )
    return "\n".join(lines)


def validate_manifest(
    root: Path, manifest: Any, overlay: str, source_commit: str | None = None
) -> tuple[dict[str, Any], str]:
    if not isinstance(manifest, dict):
        raise ProtocolError("import manifest must be an object")
    required = {"schema", "source", "profiles", "protocols", "exceptions", "overlay"}
    if set(manifest) != required or manifest.get("schema") != 1:
        raise ProtocolError("import manifest has invalid fields or schema")
    source = manifest["source"]
    if not isinstance(source, dict) or set(source) != {"repository", "commit"}:
        raise ProtocolError("manifest source must contain repository and commit")
    if source_commit is not None and source["commit"] != source_commit:
        raise ProtocolError(
            f"source checkout is {source_commit}, but the manifest pins {source['commit']}"
        )
    exceptions_document = {"schema": 1, "exceptions": manifest["exceptions"]}
    wanted_manifest, agents = assemble_import(
        root,
        manifest["profiles"],
        overlay,
        exceptions_document,
        source["repository"],
        source["commit"],
    )
    if manifest != wanted_manifest:
        raise ProtocolError(
            "import manifest drifted from its profile, protocols, or overlay"
        )
    return wanted_manifest, agents


def check_import(
    root: Path,
    manifest: Any,
    overlay: str,
    agents: str,
    source_commit: str | None = None,
    exceptions_document: Any = None,
) -> None:
    _, wanted_agents = validate_manifest(root, manifest, overlay, source_commit)
    if exceptions_document is not None:
        selected = {item["id"] for item in manifest["protocols"]}
        exceptions = validate_exceptions(exceptions_document, selected)
        if exceptions != manifest["exceptions"]:
            raise ProtocolError(
                "exceptions authoring file drifted from the import manifest"
            )
    if agents != wanted_agents:
        raise ProtocolError("AGENTS.md drifted from its pinned protocol import")


def git(root: Path, *arguments: str) -> str:
    process = subprocess.run(
        ["git", "-C", str(root), *arguments],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if process.returncode:
        raise ProtocolError(process.stderr.strip() or "git command failed")
    return process.stdout.strip()


def index_at_revision(root: Path, revision: str) -> dict[str, Any]:
    raw = git(root, "show", f"{revision}:protocols/index.json")
    try:
        return json.loads(raw)
    except json.JSONDecodeError as error:
        raise ProtocolError(f"{revision} has an invalid protocol index") from error


def source_head(root: Path) -> str:
    head = git(root, "rev-parse", "HEAD")
    if not COMMIT_ID.fullmatch(head):
        raise ProtocolError("source repository does not have a full Git HEAD")
    return head


def require_clean_source(root: Path) -> None:
    status = git(
        root,
        "status",
        "--short",
        "--untracked-files=all",
        "--",
        "protocols",
        "profiles",
        "scripts/protocols.py",
    )
    if status:
        raise ProtocolError(
            "protocol sources are dirty; commit them before rendering or checking"
        )


def command_index(args: argparse.Namespace) -> None:
    validate_version_transition(
        index_at_revision(args.root, args.base), expected_index(args.root)
    )
    index_path = args.root / "protocols" / "index.json"
    atomic_write(index_path, json_text(expected_index(args.root)))
    print(f"wrote {index_path}")


def command_validate(args: argparse.Namespace) -> None:
    protocols, profiles = validate_repository(args.root)
    print(f"valid: {len(protocols)} protocols, {len(profiles)} profiles")


def command_version_check(args: argparse.Namespace) -> None:
    previous = index_at_revision(args.root, args.base)
    validate_version_transition(previous, expected_index(args.root))
    print(f"valid: protocol IDs and versions advance from {args.base}")


def command_render(args: argparse.Namespace) -> None:
    require_clean_source(args.root)
    commit = source_head(args.root)
    overlay = args.overlay.read_text(encoding="utf-8")
    exceptions = read_json(args.exceptions) if args.exceptions else None
    manifest, agents = assemble_import(
        args.root,
        args.profile,
        overlay,
        exceptions,
        args.source_repository,
        commit,
    )
    atomic_write(args.manifest_out, json_text(manifest))
    atomic_write(args.agents_out, agents)
    print(f"wrote {args.manifest_out} and {args.agents_out}")


def command_check(args: argparse.Namespace) -> None:
    require_clean_source(args.root)
    overlay = args.overlay.read_text(encoding="utf-8")
    exceptions = read_json(args.exceptions)
    manifest = read_json(args.manifest)
    try:
        actual_agents = args.agents.read_text(encoding="utf-8")
    except FileNotFoundError as error:
        raise ProtocolError(f"missing file: {args.agents}") from error
    check_import(
        args.root,
        manifest,
        overlay,
        actual_agents,
        source_head(args.root),
        exceptions,
    )
    print(f"valid: {args.agents} matches {args.manifest}")


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--root", type=Path, default=ROOT, help=argparse.SUPPRESS)
    commands = result.add_subparsers(dest="command", required=True)

    index = commands.add_parser("index", help="regenerate protocols/index.json")
    index.add_argument("--base", default="HEAD")
    index.set_defaults(run=command_index)

    validate = commands.add_parser(
        "validate", help="validate protocols, index, and profiles"
    )
    validate.set_defaults(run=command_validate)

    version_check = commands.add_parser(
        "version-check",
        help="require permanent IDs and version bumps relative to a Git revision",
    )
    version_check.add_argument("--base", required=True)
    version_check.set_defaults(run=command_version_check)

    render = commands.add_parser(
        "render", help="render a pinned self-contained AGENTS.md"
    )
    render.add_argument(
        "--profile",
        required=True,
        action="append",
        help="ordered profile to compose; repeat for additional profiles",
    )
    render.add_argument("--overlay", required=True, type=Path)
    render.add_argument("--exceptions", required=True, type=Path)
    render.add_argument("--manifest-out", required=True, type=Path)
    render.add_argument("--agents-out", required=True, type=Path)
    render.add_argument("--source-repository", default=SOURCE_REPOSITORY)
    render.set_defaults(run=command_render)

    check = commands.add_parser(
        "check", help="detect drift in a rendered protocol import"
    )
    check.add_argument("--overlay", required=True, type=Path)
    check.add_argument("--exceptions", required=True, type=Path)
    check.add_argument("--manifest", required=True, type=Path)
    check.add_argument("--agents", required=True, type=Path)
    check.set_defaults(run=command_check)
    return result


def main() -> int:
    args = parser().parse_args()
    try:
        args.run(args)
    except (OSError, ProtocolError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

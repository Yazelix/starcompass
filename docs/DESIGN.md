# Design decisions

## The durable unit is a protocol

Each rule has a permanent ID, an integer version, a title, and canonical
Markdown. The generated index records the file hash. Profiles refer only to IDs,
so wording has one owner and profile composition cannot create hidden copies.

Protocol versions describe compatibility at the rule level. Every canonical file
change requires a bump because automation cannot safely distinguish editorial
wording from a semantic change. The history gate also prevents removal or reuse
of an ID. Git commits identify the exact catalog, profiles, and rendering
implementation used for an import. Consumers record both because neither
replaces the other.

## Profiles compose; projects still own policy

The four profiles are curated ordered sets, not inheritance trees. Repeated
`--profile` arguments form an ordered union and discard duplicate protocol IDs at
their second occurrence. This supports combinations such as a greenfield
terminal engine without inventing a profile for every project type.

Profiles contain shared rules only. Repository-specific commands, branches,
tools, paths, ownership, and release rules belong in the consumer overlay. A
consumer can alter canonical behavior only through an explicit exception.

## Self-contained output beats remote inclusion

Agents must be able to read the complete active policy from the local
`AGENTS.md`, including offline and at historical revisions. Remote includes and
runtime fetches would make behavior depend on availability and mutable external
state. Rendering duplicates text deliberately at the consumption boundary while
retaining one editable canonical source.

The generated import record makes that duplication auditable: repository, exact
commit, profiles, protocol IDs, versions, hashes, and approved exceptions are
visible without opening another file.

## Local overlay and exceptions are different

An overlay adds repository detail without weakening canonical requirements. An
exception narrows, replaces, or suspends one imported protocol. Conflating them
would hide policy changes in ordinary local instructions.

The exception authoring file and generated manifest are both checked. This makes
an unrendered approval change visible instead of letting the manifest silently
remain authoritative.

## Structural integrity is not compliance

The checker is deterministic and deliberately narrow. It can prove that the
catalog, composition, metadata, inputs, and generated output agree. It cannot
prove that an agent read or obeyed instructions, that an approval claim is true,
or that source code follows the protocols.

The audit skill preserves this boundary by reporting mechanical drift and
semantic ambiguity separately. Runtime enforcement, telemetry, and agent-action
attestation are outside the initial scope.

## Dependency decision

The checker uses Python 3's standard library.

- Rust would provide a strong distributable binary but would impose a build and
  toolchain on every early consumer for a small metadata tool.
- POSIX shell would be widely available but brittle for nested JSON, Unicode,
  deterministic rendering, and actionable validation errors.
- YAML or TOML would be pleasant for authors but require either another parser
  dependency or a restricted home-grown dialect.
- Markdown protocol sources plus JSON metadata keep human prose readable and
  machine input unambiguous with no package installation.

The exit cost is intentionally low: schemas are documented, the output is plain
Markdown and JSON, and no consumer runtime depends on the Python implementation.
A future compiled checker can replace it without changing protocol ownership.

## Skill boundary

Skills live here only when their sole responsibility is applying or auditing
these protocols. General writing, UI, review, opinion, and repository-management
skills have independent lifecycles and belong elsewhere. Co-location is not a
claim that skills are canonical protocol text; skills must reference the catalog
and checker rather than copy protocol wording.

## Bootstrap and publication

This repository cannot embed its own current commit into a generated
`AGENTS.md`: committing the result would immediately change the commit it claims
to pin. Its self-contained root instructions are therefore a documented
bootstrap exception to consumer rendering, not a hidden import.

The repository starts private and without hosted CI. This keeps early format
iteration and GitHub Actions cost explicit. Publication, licensing, releases,
and hosted checks require later user decisions.

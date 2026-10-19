//! Import assembly and rendering of `AGENTS.md`, directory rule files, and
//! the optional protocol file.

use super::catalog::{Protocol, validate_repository};
use super::manifest::{
    ExceptionsDocument, ImportRecord, Manifest, Nested, NestedFiles, Overlay, Source,
    validate_exceptions,
};
use super::{Error, Result, digest, markdown_content, valid_commit_id, valid_inline_code};
use std::{collections::BTreeSet, path::Path};

#[allow(clippy::too_many_arguments)]
pub(super) fn assemble_import(
    root: &Path,
    profile_names: &[String],
    overlay: &str,
    exceptions_document: &ExceptionsDocument,
    source_repository: &str,
    source_commit: &str,
    directories: Vec<(String, String)>,
    protocol_file: Option<String>,
) -> Result<(Manifest, String, Option<String>, Vec<NestedFiles>)> {
    let (protocols, profiles) = validate_repository(root)?;
    if profile_names.is_empty()
        || profile_names.iter().any(|name| !valid_inline_code(name))
        || profile_names.iter().collect::<BTreeSet<_>>().len() != profile_names.len()
    {
        return Err(Error::new(
            "choose one or more unique single-line profiles without backticks",
        ));
    }
    if !valid_inline_code(source_repository) {
        return Err(Error::new(
            "source repository must be non-empty single-line text without backticks",
        ));
    }
    let unknown = profile_names
        .iter()
        .filter(|name| !profiles.contains_key(*name))
        .cloned()
        .collect::<Vec<_>>();
    if !unknown.is_empty() {
        return Err(Error::new(format!(
            "unknown profiles {unknown:?}; choose from {:?}",
            profiles.keys().collect::<Vec<_>>()
        )));
    }
    if !valid_commit_id(source_commit) {
        return Err(Error::new(
            "source commit must be a full lowercase 40-character Git SHA",
        ));
    }

    let mut selected = Vec::new();
    let mut seen = BTreeSet::new();
    for name in profile_names {
        for id in &profiles[name].protocols {
            if seen.insert(id.clone()) {
                selected.push(&protocols[id]);
            }
        }
    }
    validate_exceptions(exceptions_document, &seen)?;
    let mut manifest = Manifest {
        schema: 1,
        source: Source {
            repository: source_repository.into(),
            commit: source_commit.into(),
        },
        profiles: profile_names.to_vec(),
        protocols: selected
            .iter()
            .map(|protocol| ImportRecord {
                id: protocol.id.clone(),
                version: protocol.version,
                sha256: protocol.sha256.clone(),
            })
            .collect(),
        exceptions: exceptions_document.exceptions.clone(),
        overlay: Overlay {
            sha256: digest(overlay.as_bytes()),
        },
        nested: Vec::new(),
        protocol_file,
    };
    let files = nest(&mut manifest, directories);
    let agents = render_agents(&manifest, &selected, overlay);
    let protocols = manifest
        .protocol_file
        .is_some()
        .then(|| render_protocol_file(&selected));
    Ok((manifest, agents, protocols, files))
}

/// Records sorted directory overlays in the manifest and renders their files.
fn nest(manifest: &mut Manifest, directories: Vec<(String, String)>) -> Vec<NestedFiles> {
    let mut files = directories
        .into_iter()
        .map(|(path, overlay)| NestedFiles {
            agents: render_nested_agents(&path, &overlay),
            path,
            overlay,
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.path.cmp(&right.path));
    manifest.nested = files
        .iter()
        .map(|file| Nested {
            path: file.path.clone(),
            sha256: digest(file.overlay.as_bytes()),
        })
        .collect();
    files
}

fn markdown_text(value: &str) -> String {
    value
        .replace('|', "\\|")
        .replace("\r\n", " ")
        .replace(['\r', '\n'], " ")
}

pub(super) fn render_import_record(manifest: &Manifest) -> String {
    let mut lines: Vec<String> = vec!["# Agent Guidelines".into(), String::new()];
    match &manifest.protocol_file {
        None => lines.extend([
            "This file is self-contained. Canonical protocol text was rendered into it;".into(),
            "the source repository is needed only to update or verify the import.".into(),
            "Do not edit this generated file directly. Edit `.agent-protocols.local.md`".into(),
            "or `.agent-protocols.exceptions.json`, then render from the pinned source.".into(),
        ]),
        Some(file) => lines.extend([
            format!("Read `{file}` before planning, changing, or reviewing work."),
            "It holds the canonical protocol text, which binds as if written here. The".into(),
            "source repository is needed only to update or verify the import. Do not".into(),
            "edit generated files directly. Edit `.agent-protocols.local.md` or".into(),
            "`.agent-protocols.exceptions.json`, then render from the pinned source.".into(),
        ]),
    }
    lines.extend([
        String::new(),
        "## Protocol import record".into(),
        String::new(),
        format!("- Source: `{}`", manifest.source.repository),
        format!("- Source commit: `{}`", manifest.source.commit),
        format!(
            "- Profiles: {}",
            manifest
                .profiles
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        "- Manifest: `.agent-protocols.json` (schema 1)".into(),
    ]);
    if !manifest.nested.is_empty() {
        lines.push(format!(
            "- Directory rules: {}; read a directory's `AGENTS.md` before changing files under it.",
            manifest
                .nested
                .iter()
                .map(|entry| format!("`{}/`", entry.path))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    lines.extend([
        String::new(),
        "| Protocol | Version | SHA-256 |".into(),
        "| --- | ---: | --- |".into(),
    ]);
    for protocol in &manifest.protocols {
        lines.push(format!(
            "| `{}` | {} | `{}` |",
            protocol.id, protocol.version, protocol.sha256
        ));
    }
    lines.extend([String::new(), "### Local exceptions".into(), String::new()]);
    if manifest.exceptions.is_empty() {
        lines.push("No local exceptions.".into());
    } else {
        for exception in &manifest.exceptions {
            let mut review = markdown_text(
                exception
                    .review_condition
                    .as_deref()
                    .unwrap_or("none recorded"),
            )
            .trim_end()
            .to_owned();
            if !review.ends_with(['.', '!', '?']) {
                review.push('.');
            }
            lines.push(format!(
                "- `{}` in {}: {}",
                exception.protocol,
                markdown_text(&exception.scope),
                markdown_text(&exception.reason)
            ));
            lines.push(format!(
                "  Approved by {} on {}; review condition: {}",
                markdown_text(&exception.approved_by),
                exception.approved_at,
                review
            ));
        }
    }
    lines.push(String::new());
    if manifest.protocol_file.is_none() {
        lines.extend(["## Canonical protocols".into(), String::new()]);
    }
    lines.join("\n") + "\n"
}

pub(super) const PROTOCOL_FILE_HEADER: &str = "# Canonical Agent Protocols\n\n\
    The `AGENTS.md` that names this file imports these protocols and records\n\
    their source, versions, digests, and local exceptions. Do not edit this\n\
    generated file directly; render it from the pinned source.\n";

fn render_protocol_file(protocols: &[&Protocol]) -> String {
    let mut output = PROTOCOL_FILE_HEADER.to_owned();
    for protocol in protocols {
        output.push_str(&format!(
            "\n## {} — {}\n\n{}\n",
            protocol.id, protocol.title, protocol.body
        ));
    }
    output
}

pub(super) fn render_local_rules(overlay: &str) -> String {
    [
        "## Repository-local rules",
        "",
        match markdown_content(overlay) {
            "" => "No additional local rules.",
            local => local,
        },
        "",
    ]
    .join("\n")
}

fn render_agents(manifest: &Manifest, protocols: &[&Protocol], overlay: &str) -> String {
    let mut output = render_import_record(manifest);
    for protocol in protocols
        .iter()
        .filter(|_| manifest.protocol_file.is_none())
    {
        output.push_str(&format!(
            "### {} — {}\n\n{}\n\n",
            protocol.id, protocol.title, protocol.body
        ));
    }
    output.push_str(&render_local_rules(overlay));
    output
}

pub(super) fn render_nested_agents(path: &str, overlay: &str) -> String {
    let local = match markdown_content(overlay) {
        "" => "No additional local rules.",
        local => local,
    };
    format!(
        "# Agent Guidelines for `{path}/`\n\n\
         These directory-local rules add to the repository root `AGENTS.md`, which\n\
         holds the protocol import and the repository-wide rules. Do not edit this\n\
         generated file directly. Edit `.agent-protocols.local.md` in this directory,\n\
         then render from the pinned source.\n\n\
         {local}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::starcompass::catalog::expected_index;
    use crate::starcompass::cli::SOURCE_REPOSITORY;
    use crate::starcompass::json_text;
    use crate::starcompass::test_support::*;
    use std::fs;

    #[test]
    fn repository_and_profiles_render_deterministically() {
        let (protocols, profiles) = validate_repository(&root()).unwrap();
        assert_eq!(protocols.len(), 24);
        assert_eq!(
            json_text(&expected_index(&root()).unwrap()).unwrap(),
            fs::read_to_string(root().join("protocols/index.json")).unwrap()
        );
        assert_eq!(
            profiles.keys().map(String::as_str).collect::<Vec<_>>(),
            [
                "fork",
                "greenfield",
                "orchestrator",
                "release",
                "terminal-engine"
            ]
        );
        for profile in profiles.keys() {
            let names = [profile.clone()];
            let first = assemble_root(
                &root(),
                &names,
                "Local truth.\n",
                &EMPTY_EXCEPTIONS,
                SOURCE_REPOSITORY,
                COMMIT,
            )
            .unwrap();
            let second = assemble_root(
                &root(),
                &names,
                "Local truth.\n",
                &EMPTY_EXCEPTIONS,
                SOURCE_REPOSITORY,
                COMMIT,
            )
            .unwrap();
            assert_eq!(first, second);
            assert!(first.1.contains(&format!("- Profiles: `{profile}`")));
            assert!(first.1.contains("### AP-MINIMAL-001 "));
            assert!(
                first
                    .1
                    .contains("## Repository-local rules\n\nLocal truth.")
            );
            for imported in first.0.protocols {
                assert!(first.1.contains(&format!("### {} ", imported.id)));
            }
        }
        assert_eq!(
            render_local_rules("    code\n"),
            "## Repository-local rules\n\n    code\n"
        );

        for (repository, profiles) in [
            (
                "https://example.test/repo`\n\n## Injected",
                vec!["greenfield".into()],
            ),
            (SOURCE_REPOSITORY, vec!["greenfield`\n\n## Injected".into()]),
        ] {
            assert!(
                assemble_root(
                    &root(),
                    &profiles,
                    "Local truth.\n",
                    &EMPTY_EXCEPTIONS,
                    repository,
                    COMMIT,
                )
                .unwrap_err()
                .to_string()
                .contains("single-line")
            );
        }
    }

    #[test]
    fn profiles_fit_canonical_instruction_budget() {
        const LIMIT: usize = 12_850;

        let (_, profiles) = validate_repository(&root()).unwrap();
        let local_rules = render_local_rules("");
        for profile in profiles.keys() {
            let (_, agents) = assemble_root(
                &root(),
                std::slice::from_ref(profile),
                "",
                &EMPTY_EXCEPTIONS,
                SOURCE_REPOSITORY,
                COMMIT,
            )
            .unwrap();
            let actual = agents.len() - local_rules.len();
            assert!(
                actual <= LIMIT,
                "{profile} canonical/import section is {actual} bytes; limit is {LIMIT}"
            );
        }
        let selected = vec!["greenfield".into(), "terminal-engine".into()];
        let (_, agents) = assemble_root(
            &root(),
            &selected,
            "",
            &EMPTY_EXCEPTIONS,
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        let actual = agents.len() - local_rules.len();
        const COMBINED_LIMIT: usize = 13_900;
        assert!(
            actual <= COMBINED_LIMIT,
            "greenfield + terminal-engine canonical/import section is {actual} bytes; limit is {COMBINED_LIMIT}"
        );
    }

    #[test]
    fn release_requires_delivery_before_promotion() {
        let (manifest, agents) = assemble_root(
            &root(),
            &["release".into()],
            "Local truth.\n",
            &EMPTY_EXCEPTIONS,
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        let ids = manifest
            .protocols
            .iter()
            .map(|protocol| protocol.id.as_str())
            .collect::<Vec<_>>();
        let delivery = ids.iter().position(|id| *id == "AP-DELIVERY-001").unwrap();
        let promotion = ids.iter().position(|id| *id == "AP-PROMOTION-001").unwrap();
        assert!(delivery < promotion);
        assert!(
            agents.find("### AP-DELIVERY-001 ").unwrap()
                < agents.find("### AP-PROMOTION-001 ").unwrap()
        );
    }

    #[test]
    fn exceptions_are_explicit_and_scoped_to_the_profile() {
        let mut document = ExceptionsDocument {
            schema: 1,
            exceptions: vec![exception("AP-GIT-001", "private alpha")],
        };
        document.exceptions[0].reason = "Recorded\r\nexception.".into();
        document.exceptions[0].review_condition = Some("Review later.\r\n".into());
        let (manifest, agents) = assemble_root(
            &root(),
            &["greenfield".into()],
            "Local truth.\n",
            &document,
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        assert_eq!(manifest.exceptions, document.exceptions);
        assert!(agents.contains("Recorded exception."));
        assert!(!agents.contains('\r'));
        assert!(agents.contains("Approved by user on 2026-08-04"));
        assert!(agents.lines().any(|line| {
            line == "  Approved by user on 2026-08-04; review condition: Review later."
        }));

        let mut null_review = serde_json::to_value(&document).unwrap();
        null_review["exceptions"][0]["review_condition"] = serde_json::Value::Null;
        assert!(serde_json::from_value::<ExceptionsDocument>(null_review).is_err());

        let mut year_zero = document.clone();
        year_zero.exceptions[0].approved_at = "0000-01-01".into();
        let selected = BTreeSet::from(["AP-GIT-001".into()]);
        assert!(
            validate_exceptions(&year_zero, &selected)
                .unwrap_err()
                .to_string()
                .contains("YYYY-MM-DD")
        );

        let invalid = ExceptionsDocument {
            schema: 1,
            exceptions: vec![exception("AP-TERMINAL-001", "all")],
        };
        assert!(
            assemble_root(
                &root(),
                &["greenfield".into()],
                "Local truth.\n",
                &invalid,
                SOURCE_REPOSITORY,
                COMMIT,
            )
            .unwrap_err()
            .to_string()
            .contains("outside the profile")
        );
    }

    #[test]
    fn profiles_compose_in_order_without_duplicates() {
        let (manifest, agents) = assemble_root(
            &root(),
            &["greenfield".into(), "terminal-engine".into()],
            "Local truth.\n",
            &EMPTY_EXCEPTIONS,
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        let ids = manifest
            .protocols
            .iter()
            .map(|protocol| &protocol.id)
            .collect::<Vec<_>>();
        assert_eq!(ids.len(), ids.iter().collect::<BTreeSet<_>>().len());
        assert_eq!(manifest.profiles, ["greenfield", "terminal-engine"]);
        assert!(agents.contains("- Profiles: `greenfield`, `terminal-engine`"));
        assert!(ids.iter().any(|id| id.as_str() == "AP-TERMINAL-001"));
    }
}

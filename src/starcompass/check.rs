//! Drift detection for consumer imports, with or without the pinned source.

use super::catalog::valid_protocol_id;
use super::manifest::LOCAL_OVERLAY;
use super::manifest::{
    ExceptionsDocument, Manifest, NestedFiles, nested_path, protocol_file_path, validate_exceptions,
};
use super::render::{
    PROTOCOL_FILE_HEADER, assemble_import, render_import_record, render_local_rules,
    render_nested_agents,
};
use super::{
    Error, Result, digest, read_text, valid_commit_id, valid_inline_code, valid_lower_hex,
};
use std::{collections::BTreeSet, path::Path};

/// Codex's default `project_doc_max_bytes`: it stops adding instruction files
/// on the root-to-directory chain once their combined size reaches this limit.
const INSTRUCTION_CHAIN_LIMIT: usize = 32 * 1024;

/// Reads each manifest directory's files after rejecting paths that are not
/// normalized plain relative names, so a manifest cannot escape the consumer.
pub(super) fn read_nested(base: &Path, manifest: &Manifest) -> Result<Vec<NestedFiles>> {
    manifest
        .nested
        .iter()
        .map(|entry| {
            if nested_path(&entry.path)? != entry.path {
                return Err(Error::new(format!(
                    "nested directory {:?} must be written in normalized form",
                    entry.path
                )));
            }
            let directory = base.join(&entry.path);
            Ok(NestedFiles {
                path: entry.path.clone(),
                overlay: read_text(&directory.join(LOCAL_OVERLAY))?,
                agents: read_text(&directory.join("AGENTS.md"))?,
            })
        })
        .collect()
}

pub(super) fn validate_nested(
    manifest: &Manifest,
    agents: &str,
    files: &[NestedFiles],
) -> Result<()> {
    if manifest
        .nested
        .windows(2)
        .any(|pair| pair[0].path >= pair[1].path)
    {
        return Err(Error::new("nested directories must be sorted and unique"));
    }
    for (entry, file) in manifest.nested.iter().zip(files) {
        if entry.sha256 != digest(file.overlay.as_bytes()) {
            return Err(Error::new(format!(
                "{}/{LOCAL_OVERLAY} overlay drifted from the import manifest",
                file.path
            )));
        }
        if file.agents != render_nested_agents(&file.path, &file.overlay) {
            return Err(Error::new(format!(
                "{}/AGENTS.md drifted from its overlay",
                file.path
            )));
        }
    }
    for file in files {
        let total = agents.len()
            + files
                .iter()
                .filter(|ancestor| {
                    file.path == ancestor.path
                        || file.path.starts_with(&format!("{}/", ancestor.path))
                })
                .map(|ancestor| 2 + ancestor.agents.len())
                .sum::<usize>();
        if total > INSTRUCTION_CHAIN_LIMIT {
            return Err(Error::new(format!(
                "instructions loaded in {}/ total {total} bytes, above the {INSTRUCTION_CHAIN_LIMIT}-byte default Codex project-document limit",
                file.path
            )));
        }
    }
    Ok(())
}

pub(super) fn validate_consumer_import(
    manifest: &Manifest,
    overlay: &str,
    exceptions_document: &ExceptionsDocument,
    agents: &str,
) -> Result<()> {
    if manifest.schema != 1 {
        return Err(Error::new("import manifest must use schema 1"));
    }
    if !valid_inline_code(&manifest.source.repository) {
        return Err(Error::new(
            "manifest source repository must be non-empty single-line text without backticks",
        ));
    }
    if !valid_commit_id(&manifest.source.commit) {
        return Err(Error::new(
            "manifest source commit must be a full lowercase 40-character Git SHA",
        ));
    }
    if manifest.profiles.is_empty()
        || manifest
            .profiles
            .iter()
            .any(|profile| !valid_inline_code(profile))
        || manifest.profiles.iter().collect::<BTreeSet<_>>().len() != manifest.profiles.len()
    {
        return Err(Error::new(
            "manifest profiles must be a non-empty list of unique single-line strings without backticks",
        ));
    }
    if manifest.protocols.is_empty() {
        return Err(Error::new("manifest protocols must be a non-empty list"));
    }
    let mut selected = BTreeSet::new();
    for (index, protocol) in manifest.protocols.iter().enumerate() {
        if !valid_protocol_id(&protocol.id) {
            return Err(Error::new(format!(
                "manifest protocol {} has an invalid ID",
                index + 1
            )));
        }
        if !selected.insert(protocol.id.clone()) {
            return Err(Error::new(format!(
                "manifest contains duplicate protocol ID {}",
                protocol.id
            )));
        }
        if protocol.version == 0 {
            return Err(Error::new(format!(
                "manifest protocol {} must have a positive version",
                protocol.id
            )));
        }
        if !valid_lower_hex(&protocol.sha256, 64) {
            return Err(Error::new(format!(
                "manifest protocol {} must have a lowercase SHA-256 digest",
                protocol.id
            )));
        }
    }
    if manifest.overlay.sha256 != digest(overlay.as_bytes()) {
        return Err(Error::new(
            "import manifest overlay drifted from the overlay file",
        ));
    }
    validate_exceptions(exceptions_document, &selected)?;
    if exceptions_document.exceptions != manifest.exceptions {
        return Err(Error::new(
            "exceptions authoring file drifted from the import manifest",
        ));
    }

    let prefix = render_import_record(manifest);
    if !agents.starts_with(&prefix) {
        return Err(Error::new(
            "AGENTS.md drifted in its protocol import record",
        ));
    }
    let suffix = render_local_rules(overlay);
    if !agents.ends_with(&suffix) {
        return Err(Error::new(
            "AGENTS.md drifted in its repository-local rules",
        ));
    }
    if agents.len() < prefix.len() + suffix.len() {
        return Err(Error::new("AGENTS.md drifted from its import record"));
    }
    let inline = &agents[prefix.len()..agents.len() - suffix.len()];
    match (&manifest.protocol_file, inline.trim().is_empty()) {
        (None, true) => Err(Error::new(
            "AGENTS.md canonical protocol section must be non-empty",
        )),
        (Some(file), false) => Err(Error::new(format!(
            "AGENTS.md must not inline protocol text that the manifest assigns to {file}"
        ))),
        _ => Ok(()),
    }
}

/// Reads the manifest's protocol file after rejecting a path that is not a
/// normalized plain relative name, so a manifest cannot escape the consumer.
pub(super) fn read_protocol_file(base: &Path, manifest: &Manifest) -> Result<Option<String>> {
    let Some(file) = &manifest.protocol_file else {
        return Ok(None);
    };
    if protocol_file_path(file)? != *file {
        return Err(Error::new(format!(
            "protocol file {file:?} must be written in normalized form"
        )));
    }
    let text = read_text(&base.join(file))?;
    if !text.starts_with(PROTOCOL_FILE_HEADER)
        || text[PROTOCOL_FILE_HEADER.len()..].trim().is_empty()
    {
        return Err(Error::new(format!(
            "{file} drifted in its header or has no protocol text"
        )));
    }
    Ok(Some(text))
}

pub(super) fn validate_source_import(
    root: &Path,
    manifest: &Manifest,
    overlay: &str,
    nested: &[NestedFiles],
    agents: &str,
    protocols: Option<&str>,
    source_commit: &str,
) -> Result<()> {
    if manifest.source.commit != source_commit {
        return Err(Error::new(format!(
            "source checkout is {source_commit}, but the manifest pins {}",
            manifest.source.commit
        )));
    }
    let exceptions = ExceptionsDocument {
        schema: 1,
        exceptions: manifest.exceptions.clone(),
    };
    let (wanted, wanted_agents, wanted_protocols, _) = assemble_import(
        root,
        &manifest.profiles,
        overlay,
        &exceptions,
        &manifest.source.repository,
        &manifest.source.commit,
        nested
            .iter()
            .map(|file| (file.path.clone(), file.overlay.clone()))
            .collect(),
        manifest.protocol_file.clone(),
    )?;
    if manifest != &wanted {
        return Err(Error::new(
            "import manifest drifted from its profile, protocols, or overlay",
        ));
    }
    if agents != wanted_agents {
        return Err(Error::new(
            "AGENTS.md drifted from its pinned protocol import",
        ));
    }
    if protocols != wanted_protocols.as_deref() {
        return Err(Error::new(
            "protocol file drifted from its pinned protocol import",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::starcompass::cli::SOURCE_REPOSITORY;
    use crate::starcompass::json_text;
    use crate::starcompass::test_support::*;
    use std::fs;

    #[test]
    fn manifest_check_detects_every_owned_input_drifting() {
        let empty = ExceptionsDocument {
            schema: 1,
            exceptions: Vec::new(),
        };
        let (manifest, agents) = assemble_root(
            &root(),
            &["greenfield".into()],
            "Local truth.\n",
            &empty,
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        validate_consumer_import(&manifest, "Local truth.\n", &empty, &agents).unwrap();
        validate_source_import(
            &root(),
            &manifest,
            "Local truth.\n",
            &[],
            &agents,
            None,
            COMMIT,
        )
        .unwrap();

        assert!(
            validate_consumer_import(
                &manifest,
                "Local truth.\n",
                &empty,
                &(agents.clone() + "manual edit\n"),
            )
            .unwrap_err()
            .to_string()
            .contains("AGENTS.md drifted")
        );
        assert!(
            validate_consumer_import(&manifest, "Changed local truth.\n", &empty, &agents,)
                .unwrap_err()
                .to_string()
                .contains("overlay drifted")
        );
        assert!(
            validate_source_import(
                &root(),
                &manifest,
                "Local truth.\n",
                &[],
                &agents,
                None,
                &"f".repeat(40),
            )
            .unwrap_err()
            .to_string()
            .contains("source checkout")
        );

        let changed = ExceptionsDocument {
            schema: 1,
            exceptions: vec![exception("AP-CI-001", "alpha")],
        };
        assert!(
            validate_consumer_import(&manifest, "Local truth.\n", &changed, &agents,)
                .unwrap_err()
                .to_string()
                .contains("authoring file drifted")
        );

        let marker = "## Canonical protocols\n\n";
        let changed_prose = agents.replacen(marker, &format!("{marker}Manual edit.\n\n"), 1);
        assert_ne!(changed_prose, agents);
        validate_consumer_import(&manifest, "Local truth.\n", &empty, &changed_prose).unwrap();
        assert!(
            validate_source_import(
                &root(),
                &manifest,
                "Local truth.\n",
                &[],
                &changed_prose,
                None,
                COMMIT,
            )
            .unwrap_err()
            .to_string()
            .contains("AGENTS.md drifted")
        );
    }

    #[test]
    fn consumer_manifest_rejects_invalid_self_describing_fields() {
        let exceptions = ExceptionsDocument {
            schema: 1,
            exceptions: Vec::new(),
        };
        let (manifest, agents) = assemble_root(
            &root(),
            &["greenfield".into()],
            "Local truth.\n",
            &exceptions,
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        let rejected = |changed: &Manifest, expected: &str| {
            assert!(
                validate_consumer_import(changed, "Local truth.\n", &exceptions, &agents)
                    .unwrap_err()
                    .to_string()
                    .contains(expected)
            );
        };

        let mut changed = manifest.clone();
        changed.schema = 2;
        rejected(&changed, "schema 1");

        let mut changed = manifest.clone();
        changed.source.repository.clear();
        rejected(&changed, "repository");

        let mut changed = manifest.clone();
        changed.source.repository = "https://example.test/repo`\n\n## Injected".into();
        rejected(&changed, "single-line");

        let mut changed = manifest.clone();
        changed.source.commit.make_ascii_uppercase();
        rejected(&changed, "source commit");

        let mut changed = manifest.clone();
        changed.profiles.push(changed.profiles[0].clone());
        rejected(&changed, "profiles");

        let mut changed = manifest.clone();
        changed.profiles[0] = "greenfield`\n\n## Injected".into();
        rejected(&changed, "single-line");

        let mut changed = manifest.clone();
        changed.protocols[0].id = "not-a-protocol".into();
        rejected(&changed, "invalid ID");

        let mut changed = manifest.clone();
        changed.protocols.push(changed.protocols[0].clone());
        rejected(&changed, "duplicate protocol ID");

        let mut changed = manifest.clone();
        changed.protocols[0].version = 0;
        rejected(&changed, "positive version");

        let mut changed = manifest;
        changed.protocols[0].sha256.make_ascii_uppercase();
        rejected(&changed, "lowercase SHA-256");
    }

    #[test]
    fn nested_rules_render_deterministically_and_reject_drift() {
        let (manifest, agents, files) =
            nested_fixture(&[("venus", "Venus truth.\n"), ("orbit", "Orbit truth.\n")]);
        assert_eq!(
            manifest
                .nested
                .iter()
                .map(|entry| entry.path.as_str())
                .collect::<Vec<_>>(),
            ["orbit", "venus"]
        );
        assert!(agents.contains("- Directory rules: `orbit/`, `venus/`;"));
        assert_eq!(
            files[0].agents,
            render_nested_agents("orbit", "Orbit truth.\n")
        );
        assert!(
            files[0]
                .agents
                .starts_with("# Agent Guidelines for `orbit/`\n")
        );
        assert!(files[0].agents.ends_with("\n\nOrbit truth.\n"));
        validate_consumer_import(&manifest, "Local truth.\n", &EMPTY_EXCEPTIONS, &agents).unwrap();
        validate_nested(&manifest, &agents, &files).unwrap();
        validate_source_import(
            &root(),
            &manifest,
            "Local truth.\n",
            &files,
            &agents,
            None,
            COMMIT,
        )
        .unwrap();

        let (plain, plain_agents, _) = nested_fixture(&[]);
        assert!(!plain_agents.contains("Directory rules"));
        assert!(!json_text(&plain).unwrap().contains("nested"));

        let mut changed = files.clone();
        changed[1].overlay = "Changed truth.\n".into();
        assert_error(
            validate_nested(&manifest, &agents, &changed),
            "overlay drifted",
        );

        let mut changed = files.clone();
        changed[0].agents.push_str("manual edit\n");
        assert_error(
            validate_nested(&manifest, &agents, &changed),
            "orbit/AGENTS.md drifted",
        );

        let mut changed = manifest.clone();
        changed.nested.reverse();
        assert_error(
            validate_nested(&changed, &agents, &files),
            "sorted and unique",
        );

        let large = "x".repeat(INSTRUCTION_CHAIN_LIMIT);
        let (manifest, agents, files) = nested_fixture(&[("orbit", &large)]);
        assert_error(
            validate_nested(&manifest, &agents, &files),
            "above the 32768-byte",
        );
    }

    #[test]
    fn nested_chains_include_every_ancestor_directory() {
        let (manifest, agents, files) = nested_fixture(&[
            ("orbit", &"x".repeat(INSTRUCTION_CHAIN_LIMIT / 3)),
            ("orbit/crates", &"y".repeat(INSTRUCTION_CHAIN_LIMIT / 3)),
            ("venus", &"z".repeat(INSTRUCTION_CHAIN_LIMIT / 3)),
        ]);
        assert_error(
            validate_nested(&manifest, &agents, &files),
            "orbit/crates/ total",
        );
    }

    #[test]
    fn protocol_file_moves_canonical_text_out_of_agents() {
        let (_, inline_agents, _, _) = import_fixture(&[], None);
        let (manifest, agents, protocols, files) =
            import_fixture(&[("orbit", "Orbit truth.\n")], Some("AGENT-PROTOCOLS.md"));
        let protocols = protocols.unwrap();
        assert!(agents.contains("Read `AGENT-PROTOCOLS.md` before planning"));
        assert!(agents.contains("| `AP-SCOPE-001` |"));
        assert!(!agents.contains("### AP-SCOPE-001"));
        assert!(protocols.starts_with(PROTOCOL_FILE_HEADER));
        assert!(protocols.contains("\n## AP-SCOPE-001 — "));
        assert!(agents.len() < inline_agents.len());
        validate_consumer_import(&manifest, "Local truth.\n", &EMPTY_EXCEPTIONS, &agents).unwrap();
        validate_nested(&manifest, &agents, &files).unwrap();
        let source = |agents: &str, protocols: &str| {
            validate_source_import(
                &root(),
                &manifest,
                "Local truth.\n",
                &files,
                agents,
                Some(protocols),
                COMMIT,
            )
        };
        source(&agents, &protocols).unwrap();
        assert_error(
            source(&agents, &(protocols.clone() + "manual edit\n")),
            "protocol file drifted",
        );

        let inlined = agents.replacen(
            "## Repository-local rules",
            "### AP-SCOPE-001 — copied\n\n## Repository-local rules",
            1,
        );
        assert_error(
            validate_consumer_import(&manifest, "Local truth.\n", &EMPTY_EXCEPTIONS, &inlined),
            "must not inline protocol text",
        );

        let directory = TestDirectory::copy_catalog();
        let mut manifest = manifest;
        let read = |manifest: &Manifest| read_protocol_file(&directory.0, manifest);
        assert!(read(&manifest).is_err());
        fs::write(directory.0.join("AGENT-PROTOCOLS.md"), &protocols).unwrap();
        assert_eq!(
            read(&manifest).unwrap().as_deref(),
            Some(protocols.as_str())
        );
        fs::write(directory.0.join("AGENT-PROTOCOLS.md"), "edited\n").unwrap();
        assert_error(read(&manifest), "drifted in its header");
        manifest.protocol_file = Some("../AGENT-PROTOCOLS.md".into());
        assert_error(read(&manifest), "protocol file must be");
        for loaded in ["AGENTS.md", "docs/agents.override.md"] {
            assert_error(protocol_file_path(loaded), "auto-loaded instruction file");
        }
    }
}

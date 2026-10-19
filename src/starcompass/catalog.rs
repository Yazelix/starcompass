//! The source catalog: protocol files, profiles, the generated index, and
//! the version history gate.

use super::{Error, Result, digest, markdown_content, present, read_json, valid_inline_code};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub(super) struct Protocol {
    pub(super) id: String,
    pub(super) version: u64,
    pub(super) title: String,
    path: String,
    pub(super) sha256: String,
    status: Option<ProtocolStatus>,
    pub(super) body: String,
}

impl Protocol {
    fn index_record(&self) -> IndexRecord {
        IndexRecord {
            id: self.id.clone(),
            version: self.version,
            title: self.title.clone(),
            path: self.path.clone(),
            sha256: self.sha256.clone(),
            status: self.status,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum ProtocolStatus {
    Retired,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ProtocolIndex {
    schema: u64,
    protocols: Vec<IndexRecord>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct IndexRecord {
    id: String,
    version: u64,
    title: String,
    path: String,
    sha256: String,
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    status: Option<ProtocolStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Profile {
    schema: u64,
    name: String,
    description: String,
    pub(super) protocols: Vec<String>,
}

fn require_catalog_path(
    path: &Path,
    kind: &str,
    expected: fn(&fs::FileType) -> bool,
) -> Result<()> {
    let file_type = fs::symlink_metadata(path)
        .map_err(|error| {
            Error::new(format!(
                "cannot read catalog source {}: {error}",
                path.display()
            ))
        })?
        .file_type();
    if expected(&file_type) {
        Ok(())
    } else {
        Err(Error::new(format!(
            "catalog source must be a {kind}: {}",
            path.display()
        )))
    }
}

fn parse_protocol(root: &Path, path: &Path) -> Result<Protocol> {
    let raw = fs::read(path)?;
    let text = std::str::from_utf8(&raw)
        .map_err(|_| Error::new(format!("protocol is not UTF-8: {}", path.display())))?;
    if !text.starts_with("---\n") {
        return Err(Error::new(format!(
            "protocol lacks opening frontmatter delimiter: {}",
            path.display()
        )));
    }
    let boundary = text[4..]
        .find("\n---\n")
        .map(|offset| offset + 4)
        .ok_or_else(|| {
            Error::new(format!(
                "protocol lacks closing frontmatter delimiter: {}",
                path.display()
            ))
        })?;

    let mut metadata = BTreeMap::new();
    for line in text[4..boundary].lines() {
        let (key, value) = line.split_once(':').ok_or_else(|| {
            Error::new(format!(
                "invalid frontmatter line in {}: {line:?}",
                path.display()
            ))
        })?;
        let (key, value) = (key.trim(), value.trim());
        if key.is_empty() || value.is_empty() {
            return Err(Error::new(format!(
                "invalid frontmatter line in {}: {line:?}",
                path.display()
            )));
        }
        if metadata.insert(key, value).is_some() {
            return Err(Error::new(format!(
                "duplicate frontmatter field {key:?} in {}",
                path.display()
            )));
        }
    }
    let fields: BTreeSet<_> = metadata.keys().copied().collect();
    let required = BTreeSet::from(["id", "title", "version"]);
    let allowed = BTreeSet::from(["id", "status", "title", "version"]);
    if !required.is_subset(&fields) || !fields.is_subset(&allowed) {
        return Err(Error::new(format!(
            "frontmatter fields in {} must be id, title, version, and optional status",
            path.display()
        )));
    }

    let id = metadata["id"];
    if !valid_protocol_id(id) {
        return Err(Error::new(format!(
            "invalid protocol ID {id:?} in {}",
            path.display()
        )));
    }
    if path.file_name() != Some(OsStr::new(&format!("{id}.md"))) {
        return Err(Error::new(format!(
            "protocol filename must be {id}.md: {}",
            path.display()
        )));
    }
    let version = metadata["version"].parse::<u64>().map_err(|_| {
        Error::new(format!(
            "protocol version must be an integer in {}",
            path.display()
        ))
    })?;
    if version == 0 {
        return Err(Error::new(format!(
            "protocol version must be positive in {}",
            path.display()
        )));
    }
    let status = match metadata.get("status").copied() {
        None => None,
        Some("retired") => Some(ProtocolStatus::Retired),
        Some(_) => {
            return Err(Error::new(format!(
                "protocol status must be retired in {}",
                path.display()
            )));
        }
    };
    let body = markdown_content(&text[boundary + 5..]);
    if body.is_empty() {
        return Err(Error::new(format!(
            "protocol body is empty: {}",
            path.display()
        )));
    }
    let relative = path
        .strip_prefix(root)
        .map_err(|error| Error::new(error.to_string()))?;
    let relative = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    Ok(Protocol {
        id: id.into(),
        version,
        title: metadata["title"].into(),
        path: relative,
        sha256: digest(&raw),
        status,
        body: body.into(),
    })
}

pub(super) fn valid_protocol_id(id: &str) -> bool {
    let Some(rest) = id.strip_prefix("AP-") else {
        return false;
    };
    let Some((area, number)) = rest.rsplit_once('-') else {
        return false;
    };
    area.starts_with(|character: char| character.is_ascii_uppercase())
        && area.split('-').all(|part| !part.is_empty())
        && area.chars().all(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || character == '-'
        })
        && number.len() == 3
        && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn source_files(root: &Path, directory_name: &str, extension: &str) -> Result<Vec<PathBuf>> {
    let directory = root.join(directory_name);
    require_catalog_path(&directory, "real directory", fs::FileType::is_dir)?;
    let entries = fs::read_dir(&directory).map_err(|error| {
        Error::new(format!(
            "cannot read catalog source {}: {error}",
            directory.display(),
        ))
    })?;
    let mut paths = Vec::new();
    for entry in entries {
        let path = entry?.path();
        if path.extension() == Some(OsStr::new(extension)) {
            require_catalog_path(&path, "regular file", fs::FileType::is_file)?;
            paths.push(path);
        }
    }
    paths.sort();
    if paths.is_empty() {
        return Err(Error::new(format!(
            "no {directory_name} found in {}",
            directory.display()
        )));
    }
    Ok(paths)
}

fn load_protocols(root: &Path) -> Result<BTreeMap<String, Protocol>> {
    let mut protocols = BTreeMap::new();
    for path in source_files(root, "protocols", "md")? {
        let protocol = parse_protocol(root, &path)?;
        protocols.insert(protocol.id.clone(), protocol);
    }
    Ok(protocols)
}

pub(super) fn expected_index(root: &Path) -> Result<ProtocolIndex> {
    Ok(ProtocolIndex {
        schema: 1,
        protocols: load_protocols(root)?
            .values()
            .map(Protocol::index_record)
            .collect(),
    })
}

fn load_profiles(
    root: &Path,
    protocols: &BTreeMap<String, Protocol>,
) -> Result<BTreeMap<String, Profile>> {
    let mut profiles = BTreeMap::new();
    let mut used = BTreeSet::new();
    for path in source_files(root, "profiles", "json")? {
        let profile: Profile = read_json(&path)?;
        if !valid_inline_code(&profile.name) {
            return Err(Error::new(format!(
                "profile name must be non-empty single-line text without backticks: {}",
                path.display()
            )));
        }
        if path.file_name() != Some(OsStr::new(&format!("{}.json", profile.name))) {
            return Err(Error::new(format!(
                "profile name must match filename: {}",
                path.display()
            )));
        }
        if profile.schema != 1 {
            return Err(Error::new(format!(
                "unsupported profile schema in {}",
                path.display()
            )));
        }
        if profile.description.trim().is_empty() {
            return Err(Error::new(format!(
                "profile description is empty: {}",
                path.display()
            )));
        }
        if profile.protocols.is_empty() {
            return Err(Error::new(format!(
                "profile protocols must be a non-empty string list: {}",
                path.display()
            )));
        }
        let unique = profile.protocols.iter().collect::<BTreeSet<_>>();
        if unique.len() != profile.protocols.len() {
            return Err(Error::new(format!(
                "profile contains duplicate protocol IDs: {}",
                path.display()
            )));
        }
        let unknown = profile
            .protocols
            .iter()
            .filter(|id| !protocols.contains_key(*id))
            .cloned()
            .collect::<Vec<_>>();
        if !unknown.is_empty() {
            return Err(Error::new(format!(
                "profile {} references unknown protocols: {unknown:?}",
                profile.name
            )));
        }
        let retired = profile
            .protocols
            .iter()
            .filter(|id| {
                protocols
                    .get(*id)
                    .is_some_and(|protocol| protocol.status.is_some())
            })
            .cloned()
            .collect::<Vec<_>>();
        if !retired.is_empty() {
            return Err(Error::new(format!(
                "profile {} references retired protocols: {retired:?}",
                profile.name
            )));
        }
        for required in ["AP-SCOPE-001", "AP-EXCEPTION-001"] {
            if protocols
                .get(required)
                .is_none_or(|protocol| protocol.status.is_none())
                && !profile.protocols.iter().any(|id| id == required)
            {
                return Err(Error::new(format!(
                    "profile {} must include {required}",
                    profile.name
                )));
            }
        }
        used.extend(profile.protocols.iter().cloned());
        profiles.insert(profile.name.clone(), profile);
    }
    let unused = protocols
        .values()
        .filter(|protocol| protocol.status.is_none() && !used.contains(&protocol.id))
        .map(|protocol| protocol.id.clone())
        .collect::<Vec<_>>();
    if !unused.is_empty() {
        return Err(Error::new(format!(
            "active protocols are not included in any profile: {unused:?}"
        )));
    }
    Ok(profiles)
}

pub(super) fn validate_repository(
    root: &Path,
) -> Result<(BTreeMap<String, Protocol>, BTreeMap<String, Profile>)> {
    let protocols = load_protocols(root)?;
    let index_path = root.join("protocols/index.json");
    require_catalog_path(&index_path, "regular file", fs::FileType::is_file)?;
    let actual: ProtocolIndex = read_json(&index_path)?;
    let wanted = ProtocolIndex {
        schema: 1,
        protocols: protocols.values().map(Protocol::index_record).collect(),
    };
    if actual != wanted {
        return Err(Error::new(
            "protocols/index.json is stale; run `starcompass index`",
        ));
    }
    let profiles = load_profiles(root, &protocols)?;
    Ok((protocols, profiles))
}

pub(super) fn validate_version_transition(
    previous: &ProtocolIndex,
    current: &ProtocolIndex,
) -> Result<()> {
    if previous.schema != 1 {
        return Err(Error::new("base revision has an invalid protocol index"));
    }
    let before = previous
        .protocols
        .iter()
        .map(|record| (&record.id, record))
        .collect::<BTreeMap<_, _>>();
    if before.len() != previous.protocols.len() {
        return Err(Error::new("base revision has an invalid protocol index"));
    }
    let after = current
        .protocols
        .iter()
        .map(|record| (&record.id, record))
        .collect::<BTreeMap<_, _>>();
    let removed = before
        .keys()
        .filter(|id| !after.contains_key(*id))
        .map(|id| (*id).clone())
        .collect::<Vec<_>>();
    if !removed.is_empty() {
        return Err(Error::new(format!(
            "protocol IDs are permanent and cannot be removed: {removed:?}"
        )));
    }
    let introduced_retired = after
        .iter()
        .filter(|(id, record)| !before.contains_key(*id) && record.status.is_some())
        .map(|(id, _)| (*id).clone())
        .collect::<Vec<_>>();
    if !introduced_retired.is_empty() {
        return Err(Error::new(format!(
            "protocol IDs cannot be introduced as retired: {introduced_retired:?}"
        )));
    }
    for (id, old) in before {
        let new = after.get(id).expect("removed protocol IDs were rejected");
        if old.status.is_some() && new.status.is_none() {
            return Err(Error::new(format!(
                "{id} is retired and cannot be reactivated"
            )));
        }
        if new.version < old.version {
            return Err(Error::new(format!("{id} version regressed")));
        }
        if new.sha256 != old.sha256 && new.version <= old.version {
            return Err(Error::new(format!("{id} changed without a version bump")));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::starcompass::cli::command_version_check;
    use crate::starcompass::json_text;
    use crate::starcompass::test_support::*;

    #[test]
    fn protocol_id_validation_matches_the_catalog_grammar() {
        for valid in ["AP-SCOPE-001", "AP-FORK-V2-999"] {
            assert!(valid_protocol_id(valid));
        }
        for invalid in [
            "AP--001",
            "AP-A--001",
            "AP-lower-001",
            "AP-SCOPE-01",
            "SCOPE-001",
        ] {
            assert!(!valid_protocol_id(invalid));
        }
    }

    #[test]
    fn protocol_parser_preserves_markdown_indentation_and_u64_versions() {
        let directory = TestDirectory::copy_catalog();
        let path = directory.0.join("protocols/AP-SCOPE-001.md");
        let current = parse_protocol(&directory.0, &path).unwrap().version;
        let text = fs::read_to_string(&path).unwrap().replacen(
            &format!("version: {current}"),
            "version: 18446744073709551615",
            1,
        );
        fs::write(&path, text).unwrap();

        assert_eq!(
            parse_protocol(&directory.0, &path).unwrap().version,
            u64::MAX
        );

        let body = parse_protocol(&directory.0, &path).unwrap().body;
        let text = fs::read_to_string(&path)
            .unwrap()
            .replacen(&body, &format!("    {body}"), 1);
        fs::write(&path, text).unwrap();
        assert!(
            parse_protocol(&directory.0, &path)
                .unwrap()
                .body
                .starts_with("    ")
        );
    }

    #[test]
    fn protocol_history_requires_permanent_ids_and_version_bumps() {
        let current = expected_index(&root()).unwrap();
        validate_version_transition(&current, &current).unwrap();

        let mut unchanged_version = current.clone();
        unchanged_version.protocols[0].sha256 = "0".repeat(64);
        assert!(
            validate_version_transition(&unchanged_version, &current)
                .unwrap_err()
                .to_string()
                .contains("without a version bump")
        );

        let mut missing = current.clone();
        missing.protocols.push(IndexRecord {
            id: "AP-REMOVED-999".into(),
            version: 1,
            title: "Removed".into(),
            path: "protocols/AP-REMOVED-999.md".into(),
            sha256: "0".repeat(64),
            status: None,
        });
        assert!(
            validate_version_transition(&missing, &current)
                .unwrap_err()
                .to_string()
                .contains("cannot be removed")
        );

        let mut duplicate = current.clone();
        duplicate.protocols.push(current.protocols[0].clone());
        assert!(
            validate_version_transition(&duplicate, &current)
                .unwrap_err()
                .to_string()
                .contains("invalid protocol index")
        );

        for revision in ["", "--no-patch"] {
            assert!(
                command_version_check(&root(), revision)
                    .unwrap_err()
                    .to_string()
                    .contains("invalid Git revision")
            );
        }
    }

    #[test]
    fn retired_protocols_remain_indexed_but_leave_active_profiles() {
        let directory = TestDirectory::copy_catalog();
        let active = expected_index(&directory.0).unwrap();
        let mut null_status = serde_json::to_value(&active).unwrap();
        null_status["protocols"][0]["status"] = serde_json::Value::Null;
        assert!(serde_json::from_value::<ProtocolIndex>(null_status).is_err());

        let mut introduced = active.clone();
        let mut born_retired = introduced.protocols[0].clone();
        born_retired.id = "AP-RETIRED-999".into();
        born_retired.status = Some(ProtocolStatus::Retired);
        introduced.protocols.push(born_retired);
        assert_error(
            validate_version_transition(&active, &introduced),
            "cannot be introduced as retired",
        );

        let index_path = directory.0.join("protocols/index.json");
        let protocol_path = directory.0.join("protocols/AP-SCOPE-001.md");
        let active_version = active
            .protocols
            .iter()
            .find(|record| record.id == "AP-SCOPE-001")
            .unwrap()
            .version;
        let retired_version = active_version + 1;
        let protocol = fs::read_to_string(&protocol_path).unwrap().replacen(
            &format!("version: {active_version}\n"),
            &format!("version: {retired_version}\nstatus: retired\n"),
            1,
        );
        fs::write(&protocol_path, protocol).unwrap();

        for profile_path in source_files(&directory.0, "profiles", "json").unwrap() {
            let mut profile: Profile = read_json(&profile_path).unwrap();
            profile.protocols.retain(|id| id != "AP-SCOPE-001");
            fs::write(&profile_path, json_text(&profile).unwrap()).unwrap();
        }

        let retired = expected_index(&directory.0).unwrap();
        let record = retired
            .protocols
            .iter()
            .find(|record| record.id == "AP-SCOPE-001")
            .unwrap();
        assert_eq!(record.status, Some(ProtocolStatus::Retired));
        validate_version_transition(&active, &retired).unwrap();
        fs::write(&index_path, json_text(&retired).unwrap()).unwrap();

        validate_repository(&directory.0).unwrap();
        let profile_path = directory.0.join("profiles/regular.json");
        let mut profile: Profile = read_json(&profile_path).unwrap();
        profile.protocols.push("AP-SCOPE-001".into());
        fs::write(&profile_path, json_text(&profile).unwrap()).unwrap();
        assert_error(
            validate_repository(&directory.0),
            "references retired protocols",
        );

        profile.protocols.pop();
        fs::write(&profile_path, json_text(&profile).unwrap()).unwrap();
        let reactivated_version = retired_version + 1;
        let protocol = fs::read_to_string(&protocol_path).unwrap().replacen(
            &format!("version: {retired_version}\nstatus: retired\n"),
            &format!("version: {reactivated_version}\n"),
            1,
        );
        fs::write(&protocol_path, protocol).unwrap();
        let reactivated = expected_index(&directory.0).unwrap();
        fs::write(&index_path, json_text(&reactivated).unwrap()).unwrap();
        assert_error(
            validate_repository(&directory.0),
            "must include AP-SCOPE-001",
        );
        assert_error(
            validate_version_transition(&retired, &reactivated),
            "cannot be reactivated",
        );

        let invalid_status_version = reactivated_version + 1;
        let protocol = fs::read_to_string(&protocol_path).unwrap().replacen(
            &format!("version: {reactivated_version}\n"),
            &format!("version: {invalid_status_version}\nstatus: active\n"),
            1,
        );
        fs::write(&protocol_path, protocol).unwrap();
        assert_error(
            expected_index(&directory.0),
            "protocol status must be retired",
        );
    }

    #[test]
    fn catalog_rejects_stale_protocols_and_invalid_profiles() {
        let stale = TestDirectory::copy_catalog();
        let path = stale.0.join("protocols/AP-SCOPE-001.md");
        fs::write(&path, fs::read_to_string(&path).unwrap() + "\nDrift.\n").unwrap();
        assert!(
            validate_repository(&stale.0)
                .unwrap_err()
                .to_string()
                .contains("index.json is stale")
        );

        let unknown = TestDirectory::copy_catalog();
        let path = unknown.0.join("profiles/regular.json");
        let mut profile: Profile = read_json(&path).unwrap();
        profile.protocols.push("AP-NOT-REAL-999".into());
        fs::write(path, json_text(&profile).unwrap()).unwrap();
        assert!(
            validate_repository(&unknown.0)
                .unwrap_err()
                .to_string()
                .contains("unknown protocols")
        );

        let unrenderable = TestDirectory::copy_catalog();
        let old_path = unrenderable.0.join("profiles/regular.json");
        let mut profile: Profile = read_json(&old_path).unwrap();
        profile.name = "reg`ular".into();
        fs::write(
            unrenderable.0.join("profiles/reg`ular.json"),
            json_text(&profile).unwrap(),
        )
        .unwrap();
        fs::remove_file(old_path).unwrap();
        assert!(
            validate_repository(&unrenderable.0)
                .unwrap_err()
                .to_string()
                .contains("profile name")
        );
    }

    #[cfg(unix)]
    #[test]
    fn catalog_rejects_symlinked_sources() {
        use std::os::unix::fs::symlink;

        for relative in [
            "protocols/AP-SCOPE-001.md",
            "profiles/regular.json",
            "protocols/index.json",
            "protocols",
        ] {
            let directory = TestDirectory::copy_catalog();
            let path = directory.0.join(relative);
            let external = directory.0.join("external");
            fs::rename(&path, &external).unwrap();
            symlink(&external, &path).unwrap();

            assert!(
                validate_repository(&directory.0)
                    .unwrap_err()
                    .to_string()
                    .contains("catalog source")
            );
        }
    }
}

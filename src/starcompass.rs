use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    fmt, fs,
    io::Write,
    path::{Component, Path, PathBuf},
    process::Command,
};

const SOURCE_REPOSITORY: &str = "https://github.com/Yazelix/starcompass";
const LOCAL_OVERLAY: &str = ".agent-protocols.local.md";
/// Codex's default `project_doc_max_bytes`: it stops adding instruction files
/// on the root-to-directory chain once their combined size reaches this limit.
const INSTRUCTION_CHAIN_LIMIT: usize = 32 * 1024;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Error(String);

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }
}

#[derive(Debug)]
struct Protocol {
    id: String,
    version: u64,
    title: String,
    path: String,
    sha256: String,
    status: Option<ProtocolStatus>,
    body: String,
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
struct ProtocolIndex {
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
struct Profile {
    schema: u64,
    name: String,
    description: String,
    protocols: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ExceptionsDocument {
    schema: u64,
    exceptions: Vec<Exception>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct Exception {
    approved_at: String,
    approved_by: String,
    protocol: String,
    reason: String,
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    review_condition: Option<String>,
    scope: String,
}

fn present<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: u64,
    source: Source,
    profiles: Vec<String>,
    protocols: Vec<ImportRecord>,
    exceptions: Vec<Exception>,
    overlay: Overlay,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    nested: Vec<Nested>,
}

/// A directory whose generated `AGENTS.md` adds local rules to the root import.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct Nested {
    path: String,
    sha256: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct Source {
    repository: String,
    commit: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ImportRecord {
    id: String,
    version: u64,
    sha256: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct Overlay {
    sha256: String,
}

fn digest(data: &[u8]) -> String {
    let bytes = Sha256::digest(data);
    let mut output = String::with_capacity(64);
    for byte in bytes {
        use fmt::Write as _;
        write!(&mut output, "{byte:02x}").expect("writing to a string cannot fail");
    }
    output
}

fn read_text(path: &Path) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(Error::new(format!("missing file: {}", path.display())))
        }
        Err(error) => Err(error.into()),
    }
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

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text = read_text(path)?;
    serde_json::from_str(&text)
        .map_err(|error| Error::new(format!("invalid JSON in {}: {error}", path.display())))
}

fn json_text<T: Serialize>(value: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(value)? + "\n")
}

fn markdown_content(value: &str) -> &str {
    value.trim_start_matches(['\r', '\n']).trim_end()
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

fn valid_protocol_id(id: &str) -> bool {
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

fn expected_index(root: &Path) -> Result<ProtocolIndex> {
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

fn validate_repository(
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

fn validate_version_transition(previous: &ProtocolIndex, current: &ProtocolIndex) -> Result<()> {
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

fn validate_exceptions(document: &ExceptionsDocument, selected: &BTreeSet<String>) -> Result<()> {
    if document.schema != 1 {
        return Err(Error::new(
            "exceptions document must use schema 1 and an exceptions list",
        ));
    }
    let mut identities = BTreeSet::new();
    for (index, exception) in document.exceptions.iter().enumerate() {
        let number = index + 1;
        if [
            exception.approved_at.as_str(),
            exception.approved_by.as_str(),
            exception.protocol.as_str(),
            exception.reason.as_str(),
            exception.scope.as_str(),
        ]
        .into_iter()
        .any(|value| value.trim().is_empty())
            || exception
                .review_condition
                .as_deref()
                .is_some_and(|value| value.trim().is_empty())
        {
            return Err(Error::new(format!(
                "exception {number} fields must be non-empty strings"
            )));
        }
        if !selected.contains(&exception.protocol) {
            return Err(Error::new(format!(
                "exception {number} targets a protocol outside the profile"
            )));
        }
        if !valid_date(&exception.approved_at) {
            return Err(Error::new(format!(
                "exception {number} approved_at must be YYYY-MM-DD"
            )));
        }
        let identity = (exception.protocol.clone(), exception.scope.clone());
        if !identities.insert(identity) {
            return Err(Error::new(format!(
                "duplicate exception scope for {}",
                exception.protocol
            )));
        }
    }
    Ok(())
}

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes
            .iter()
            .enumerate()
            .any(|(index, byte)| !matches!(index, 4 | 7) && !byte.is_ascii_digit())
    {
        return false;
    }
    let Ok(year) = value[..4].parse::<u32>() else {
        return false;
    };
    if year == 0 {
        return false;
    }
    let Ok(month) = value[5..7].parse::<u32>() else {
        return false;
    };
    let Ok(day) = value[8..].parse::<u32>() else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}

fn assemble_import(
    root: &Path,
    profile_names: &[String],
    overlay: &str,
    exceptions_document: &ExceptionsDocument,
    source_repository: &str,
    source_commit: &str,
    directories: Vec<(String, String)>,
) -> Result<(Manifest, String, Vec<NestedFiles>)> {
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
    };
    let files = nest(&mut manifest, directories);
    let agents = render_agents(&manifest, &selected, overlay);
    Ok((manifest, agents, files))
}

/// One directory's overlay and generated `AGENTS.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
struct NestedFiles {
    path: String,
    overlay: String,
    agents: String,
}

/// Normalizes a consumer directory to plain relative names joined by `/`.
fn nested_path(value: &str) -> Result<String> {
    let invalid = || {
        Error::new(format!(
            "nested directory must be a relative path of plain directory names without backticks or line breaks: {value:?}"
        ))
    };
    let mut names = Vec::new();
    for component in Path::new(value).components() {
        match component {
            Component::Normal(name) => names.push(name.to_str().ok_or_else(invalid)?),
            Component::CurDir if !names.is_empty() => {}
            _ => return Err(invalid()),
        }
    }
    let path = names.join("/");
    if names.is_empty() || !valid_inline_code(&path) {
        return Err(invalid());
    }
    Ok(path)
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

fn render_import_record(manifest: &Manifest) -> String {
    let mut lines = vec![
        "# Agent Guidelines".into(),
        String::new(),
        "This file is self-contained. Canonical protocol text was rendered into it;".into(),
        "the source repository is needed only to update or verify the import.".into(),
        "Do not edit this generated file directly. Edit `.agent-protocols.local.md`".into(),
        "or `.agent-protocols.exceptions.json`, then render from the pinned source.".into(),
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
    ];
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
    lines.extend([
        String::new(),
        "## Canonical protocols".into(),
        String::new(),
    ]);
    lines.join("\n") + "\n"
}

fn render_local_rules(overlay: &str) -> String {
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
    for protocol in protocols {
        output.push_str(&format!(
            "### {} — {}\n\n{}\n\n",
            protocol.id, protocol.title, protocol.body
        ));
    }
    output.push_str(&render_local_rules(overlay));
    output
}

fn render_nested_agents(path: &str, overlay: &str) -> String {
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

/// Reads each manifest directory's files after rejecting paths that are not
/// normalized plain relative names, so a manifest cannot escape the consumer.
fn read_nested(base: &Path, manifest: &Manifest) -> Result<Vec<NestedFiles>> {
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

fn validate_nested(manifest: &Manifest, agents: &str, files: &[NestedFiles]) -> Result<()> {
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

fn valid_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_inline_code(value: &str) -> bool {
    !value.trim().is_empty() && !value.contains(['`', '\r', '\n'])
}

fn validate_consumer_import(
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
    if agents.len() < prefix.len() + suffix.len()
        || agents[prefix.len()..agents.len() - suffix.len()]
            .trim()
            .is_empty()
    {
        return Err(Error::new(
            "AGENTS.md canonical protocol section must be non-empty",
        ));
    }
    Ok(())
}

fn validate_source_import(
    root: &Path,
    manifest: &Manifest,
    overlay: &str,
    nested: &[NestedFiles],
    agents: &str,
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
    let (wanted, wanted_agents, _) = assemble_import(
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
    Ok(())
}

fn git(root: &Path, arguments: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()?;
    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(Error::new(if message.is_empty() {
            "git command failed".into()
        } else {
            message
        }));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn index_at_revision(root: &Path, revision: &str) -> Result<ProtocolIndex> {
    if revision.is_empty() || revision.starts_with('-') {
        return Err(Error::new("invalid Git revision"));
    }
    let object = format!("{revision}^{{commit}}");
    let commit = git(root, &["rev-parse", "--verify", &object])
        .map_err(|_| Error::new(format!("invalid Git revision: {revision}")))?;
    if !valid_commit_id(&commit) {
        return Err(Error::new(format!("invalid Git revision: {revision}")));
    }
    let reference = format!("{commit}:protocols/index.json");
    let raw = git(root, &["show", &reference])?;
    serde_json::from_str(&raw)
        .map_err(|_| Error::new(format!("{revision} has an invalid protocol index")))
}

fn source_head(root: &Path) -> Result<String> {
    let head = git(root, &["rev-parse", "HEAD"])?;
    if !valid_commit_id(&head) {
        return Err(Error::new(
            "source repository does not have a full Git HEAD",
        ));
    }
    Ok(head)
}

fn valid_commit_id(value: &str) -> bool {
    valid_lower_hex(value, 40)
}

fn require_clean_source(root: &Path) -> Result<()> {
    let status = git(
        root,
        &[
            "status",
            "--short",
            "--untracked-files=all",
            "--",
            "protocols",
            "profiles",
            "Cargo.toml",
            "Cargo.lock",
            "src",
        ],
    )?;
    if !status.is_empty() {
        return Err(Error::new(
            "protocol sources are dirty; commit them before rendering or checking",
        ));
    }
    Ok(())
}

fn path_identity(path: &Path) -> Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let (ancestor, mut identity) = absolute
        .ancestors()
        .find_map(|ancestor| {
            ancestor
                .canonicalize()
                .ok()
                .map(|identity| (ancestor, identity))
        })
        .ok_or_else(|| Error::new(format!("cannot resolve path: {}", path.display())))?;
    for component in absolute
        .strip_prefix(ancestor)
        .expect("ancestor came from this path")
        .components()
    {
        match component {
            Component::Normal(name) => identity.push(name),
            Component::ParentDir => {
                identity.pop();
            }
            _ => {}
        }
    }
    Ok(identity)
}

fn require_distinct_paths<'a>(paths: impl IntoIterator<Item = &'a Path>) -> Result<()> {
    let mut identities = BTreeSet::new();
    for path in paths {
        if !identities.insert(path_identity(path)?) {
            return Err(Error::new(
                "render inputs and outputs must use distinct paths",
            ));
        }
    }
    Ok(())
}

fn atomic_write(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    let options = atomic_write_file::OpenOptions::new();
    #[cfg(unix)]
    let options = {
        use atomic_write_file::unix::OpenOptionsExt as _;

        let mut options = options;
        options.preserve_mode(false).try_preserve_owner(false);
        options
    };
    let mut file = options.open(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;

        file.set_permissions(fs::Permissions::from_mode(0o644))?;
    }
    file.write_all(content.as_bytes())?;
    file.commit()?;
    Ok(())
}

fn command_index(root: &Path, base: &str) -> Result<String> {
    let wanted = expected_index(root)?;
    validate_version_transition(&index_at_revision(root, base)?, &wanted)?;
    let path = root.join("protocols/index.json");
    atomic_write(&path, &json_text(&wanted)?)?;
    Ok(format!("wrote {}", path.display()))
}

fn command_version_check(root: &Path, base: &str) -> Result<String> {
    validate_version_transition(&index_at_revision(root, base)?, &expected_index(root)?)?;
    Ok(format!(
        "valid: protocol IDs and versions advance from {base}"
    ))
}

/// Directories resolve relative to the root `AGENTS.md`.
fn consumer_base(agents_path: &Path) -> &Path {
    agents_path.parent().unwrap_or(Path::new(""))
}

fn directory_files(count: usize) -> String {
    match count {
        0 => String::new(),
        1 => ", with 1 directory file".into(),
        count => format!(", with {count} directory files"),
    }
}

#[allow(clippy::too_many_arguments)]
fn command_render(
    root: &Path,
    profiles: &[String],
    overlay_path: &Path,
    exceptions_path: &Path,
    manifest_path: &Path,
    agents_path: &Path,
    nested: &[String],
    source_repository: &str,
) -> Result<String> {
    let mut directories = BTreeSet::new();
    for value in nested {
        let path = nested_path(value)?;
        if !directories.insert(path.clone()) {
            return Err(Error::new(format!(
                "nested directory {path} is listed more than once"
            )));
        }
    }
    let base = consumer_base(agents_path);
    let nested_paths = directories
        .iter()
        .map(|path| {
            (
                base.join(path).join(LOCAL_OVERLAY),
                base.join(path).join("AGENTS.md"),
            )
        })
        .collect::<Vec<_>>();
    require_distinct_paths(
        [overlay_path, exceptions_path, manifest_path, agents_path]
            .into_iter()
            .chain(
                nested_paths
                    .iter()
                    .flat_map(|(input, output)| [input.as_path(), output.as_path()]),
            ),
    )?;
    require_clean_source(root)?;
    let overlay = read_text(overlay_path)?;
    let exceptions = read_json(exceptions_path)?;
    let nested_overlays = directories
        .into_iter()
        .zip(&nested_paths)
        .map(|(path, (input, _))| Ok((path, read_text(input)?)))
        .collect::<Result<Vec<_>>>()?;
    let (manifest, agents, files) = assemble_import(
        root,
        profiles,
        &overlay,
        &exceptions,
        source_repository,
        &source_head(root)?,
        nested_overlays,
    )?;
    validate_nested(&manifest, &agents, &files)?;
    for ((_, output), file) in nested_paths.iter().zip(&files) {
        atomic_write(output, &file.agents)?;
    }
    atomic_write(manifest_path, &json_text(&manifest)?)?;
    atomic_write(agents_path, &agents)?;
    Ok(format!(
        "wrote {} and {}{}",
        manifest_path.display(),
        agents_path.display(),
        directory_files(files.len())
    ))
}

fn command_check(
    root: &Path,
    overlay_path: &Path,
    exceptions_path: &Path,
    manifest_path: &Path,
    agents_path: &Path,
) -> Result<String> {
    let overlay = read_text(overlay_path)?;
    let exceptions: ExceptionsDocument = read_json(exceptions_path)?;
    let manifest: Manifest = read_json(manifest_path)?;
    let agents = read_text(agents_path)?;
    let nested = read_nested(consumer_base(agents_path), &manifest)?;
    validate_consumer_import(&manifest, &overlay, &exceptions, &agents)?;
    validate_nested(&manifest, &agents, &nested)?;
    require_clean_source(root)?;
    validate_source_import(
        root,
        &manifest,
        &overlay,
        &nested,
        &agents,
        &source_head(root)?,
    )?;
    Ok(format!(
        "valid: {} matches {}{}",
        agents_path.display(),
        manifest_path.display(),
        directory_files(nested.len())
    ))
}

fn command_check_consumer(
    overlay_path: &Path,
    exceptions_path: &Path,
    manifest_path: &Path,
    agents_path: &Path,
) -> Result<String> {
    let overlay = read_text(overlay_path)?;
    let exceptions: ExceptionsDocument = read_json(exceptions_path)?;
    let manifest: Manifest = read_json(manifest_path)?;
    let agents = read_text(agents_path)?;
    let nested = read_nested(consumer_base(agents_path), &manifest)?;
    validate_consumer_import(&manifest, &overlay, &exceptions, &agents)?;
    validate_nested(&manifest, &agents, &nested)?;
    Ok(format!(
        "valid: consumer files agree{}; canonical protocol prose and manifest protocol digests were not verified against source",
        directory_files(nested.len())
    ))
}

fn parse_root(arguments: &mut Vec<String>) -> Result<PathBuf> {
    if let Some(path) = arguments
        .first()
        .and_then(|argument| argument.strip_prefix("--root="))
    {
        let path = PathBuf::from(path);
        arguments.remove(0);
        return Ok(path);
    }
    if arguments.first().map(String::as_str) == Some("--root") {
        if arguments.len() < 2 {
            return Err(Error::new("--root requires a path"));
        }
        arguments.remove(0);
        return Ok(PathBuf::from(arguments.remove(0)));
    }
    discover_root()
}

fn discover_root() -> Result<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(current) = std::env::current_dir() {
        candidates.push(current);
    }
    if let Ok(executable) = std::env::current_exe() {
        if let Some(parent) = executable.parent() {
            candidates.push(parent.to_path_buf());
        }
    }
    for candidate in candidates {
        for ancestor in candidate.ancestors() {
            if ancestor.join("protocols/index.json").is_file() && ancestor.join("profiles").is_dir()
            {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    Err(Error::new(
        "could not locate a Starcompass checkout; pass --root PATH",
    ))
}

fn take_options(arguments: &mut Vec<String>, name: &str) -> Result<Vec<String>> {
    let mut values = Vec::new();
    let prefix = format!("{name}=");
    let mut index = 0;
    while index < arguments.len() {
        if let Some(value) = arguments[index].strip_prefix(&prefix) {
            values.push(value.into());
            arguments.remove(index);
        } else if arguments[index] == name {
            arguments.remove(index);
            if index == arguments.len() {
                return Err(Error::new(format!("{name} requires a value")));
            }
            values.push(arguments.remove(index));
        } else {
            index += 1;
        }
    }
    Ok(values)
}

fn take_option(arguments: &mut Vec<String>, name: &str) -> Result<Option<String>> {
    let mut values = take_options(arguments, name)?;
    if values.len() > 1 {
        return Err(Error::new(format!("{name} may be used only once")));
    }
    Ok(values.pop())
}

fn require_option(arguments: &mut Vec<String>, name: &str) -> Result<String> {
    take_option(arguments, name)?.ok_or_else(|| Error::new(format!("missing required {name}")))
}

fn reject_extra(arguments: &[String]) -> Result<()> {
    if arguments.is_empty() {
        Ok(())
    } else {
        Err(Error::new(format!(
            "unrecognized arguments: {}",
            arguments.join(" ")
        )))
    }
}

pub(crate) fn run<I, S>(arguments: I) -> std::result::Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    arguments
        .into_iter()
        .map(|argument| {
            argument
                .into()
                .into_string()
                .map_err(|_| Error::new("arguments must be valid UTF-8"))
        })
        .collect::<Result<Vec<_>>>()
        .and_then(run_inner)
        .map_err(|error| error.to_string())
}

fn run_inner(mut arguments: Vec<String>) -> Result<String> {
    if arguments
        .iter()
        .any(|argument| matches!(argument.as_str(), "-h" | "--help"))
    {
        return Ok(help());
    }
    if arguments.is_empty() {
        return Err(Error::new("missing command; run `starcompass --help`"));
    }
    if arguments.first().map(String::as_str) == Some("check-consumer") {
        arguments.remove(0);
        let overlay = PathBuf::from(require_option(&mut arguments, "--overlay")?);
        let exceptions = PathBuf::from(require_option(&mut arguments, "--exceptions")?);
        let manifest = PathBuf::from(require_option(&mut arguments, "--manifest")?);
        let agents = PathBuf::from(require_option(&mut arguments, "--agents")?);
        reject_extra(&arguments)?;
        return command_check_consumer(&overlay, &exceptions, &manifest, &agents);
    }
    let root = parse_root(&mut arguments)?;
    if arguments.is_empty() {
        return Err(Error::new("missing command"));
    }
    let command = arguments.remove(0);
    match command.as_str() {
        "validate" => {
            reject_extra(&arguments)?;
            let (protocols, profiles) = validate_repository(&root)?;
            Ok(format!(
                "valid: {} protocols, {} profiles",
                protocols.len(),
                profiles.len()
            ))
        }
        "index" => {
            let base = take_option(&mut arguments, "--base")?.unwrap_or_else(|| "HEAD".into());
            reject_extra(&arguments)?;
            command_index(&root, &base)
        }
        "version-check" => {
            let base = require_option(&mut arguments, "--base")?;
            reject_extra(&arguments)?;
            command_version_check(&root, &base)
        }
        "render" => {
            let profiles = take_options(&mut arguments, "--profile")?;
            if profiles.is_empty() {
                return Err(Error::new("missing required --profile"));
            }
            let overlay = PathBuf::from(require_option(&mut arguments, "--overlay")?);
            let exceptions = PathBuf::from(require_option(&mut arguments, "--exceptions")?);
            let manifest = PathBuf::from(require_option(&mut arguments, "--manifest-out")?);
            let agents = PathBuf::from(require_option(&mut arguments, "--agents-out")?);
            let nested = take_options(&mut arguments, "--nested")?;
            let repository = take_option(&mut arguments, "--source-repository")?
                .unwrap_or_else(|| SOURCE_REPOSITORY.into());
            reject_extra(&arguments)?;
            command_render(
                &root,
                &profiles,
                &overlay,
                &exceptions,
                &manifest,
                &agents,
                &nested,
                &repository,
            )
        }
        "check" => {
            let overlay = PathBuf::from(require_option(&mut arguments, "--overlay")?);
            let exceptions = PathBuf::from(require_option(&mut arguments, "--exceptions")?);
            let manifest = PathBuf::from(require_option(&mut arguments, "--manifest")?);
            let agents = PathBuf::from(require_option(&mut arguments, "--agents")?);
            reject_extra(&arguments)?;
            command_check(&root, &overlay, &exceptions, &manifest, &agents)
        }
        "check-consumer" => Err(Error::new("--root is not valid with check-consumer")),
        _ => Err(Error::new(format!("unknown command: {command}"))),
    }
}

fn help() -> String {
    "usage:\n  starcompass [--root PATH] {index,validate,version-check,render,check}\n  starcompass check-consumer --overlay PATH --exceptions PATH --manifest PATH --agents PATH\n\nIndex, render, and verify agent-protocol imports."
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    const COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";
    const EMPTY_EXCEPTIONS: ExceptionsDocument = ExceptionsDocument {
        schema: 1,
        exceptions: Vec::new(),
    };
    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn copy_catalog() -> Self {
            let path = std::env::temp_dir().join(format!(
                "starcompass-test-{}-{}",
                std::process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            copy_tree(&root().join("protocols"), &path.join("protocols"));
            copy_tree(&root().join("profiles"), &path.join("profiles"));
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn copy_tree(source: &Path, destination: &Path) {
        fs::create_dir(destination).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            fs::copy(entry.path(), destination.join(entry.file_name())).unwrap();
        }
    }

    fn assemble_root(
        root: &Path,
        profile_names: &[String],
        overlay: &str,
        exceptions_document: &ExceptionsDocument,
        source_repository: &str,
        source_commit: &str,
    ) -> Result<(Manifest, String)> {
        assemble_import(
            root,
            profile_names,
            overlay,
            exceptions_document,
            source_repository,
            source_commit,
            Vec::new(),
        )
        .map(|(manifest, agents, _)| (manifest, agents))
    }

    fn exception(protocol: &str, scope: &str) -> Exception {
        Exception {
            approved_at: "2026-08-04".into(),
            approved_by: "user".into(),
            protocol: protocol.into(),
            reason: "Recorded exception.".into(),
            review_condition: Some("Review later.".into()),
            scope: scope.into(),
        }
    }

    fn assert_error<T: std::fmt::Debug>(result: Result<T>, expected: &str) {
        assert!(result.unwrap_err().to_string().contains(expected));
    }

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
    fn repository_and_profiles_render_deterministically() {
        let (protocols, profiles) = validate_repository(&root()).unwrap();
        assert_eq!(protocols.len(), 22);
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
        const LIMIT: usize = 12 * 1024;

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
        const ORBIT_LIMIT: usize = 30 * 1024 - 17_690;
        assert!(
            actual <= ORBIT_LIMIT,
            "greenfield + terminal-engine canonical/import section is {actual} bytes; limit is {ORBIT_LIMIT}"
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
            exceptions: vec![exception("AP-CI-001", "private alpha")],
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
        let selected = BTreeSet::from(["AP-CI-001".into()]);
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
        let profile_path = directory.0.join("profiles/greenfield.json");
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

    #[cfg(unix)]
    #[test]
    fn atomic_output_restores_the_unix_mode_contract() {
        use std::os::unix::fs::PermissionsExt;

        let directory = TestDirectory::copy_catalog();
        let path = directory.0.join("output");
        fs::write(&path, "old").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();

        atomic_write(&path, "new").unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
        assert_eq!(
            fs::metadata(path).unwrap().permissions().mode() & 0o777,
            0o644
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
        let path = unknown.0.join("profiles/greenfield.json");
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
        let old_path = unrenderable.0.join("profiles/greenfield.json");
        let mut profile: Profile = read_json(&old_path).unwrap();
        profile.name = "green`field".into();
        fs::write(
            unrenderable.0.join("profiles/green`field.json"),
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
            "profiles/greenfield.json",
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

    #[test]
    fn render_rejects_colliding_paths_before_writing() {
        let directory = TestDirectory::copy_catalog();
        let overlay = directory.0.join("overlay");
        let exceptions = directory.0.join("exceptions");
        fs::create_dir(directory.0.join("sub")).unwrap();
        let alias = directory.0.join("sub").join("..").join("overlay");
        let agents = directory.0.join("AGENTS.md");
        fs::write(&overlay, "Local truth.\n").unwrap();
        let error = command_render(
            &root(),
            &["greenfield".into()],
            &overlay,
            &exceptions,
            &alias,
            &agents,
            &[],
            SOURCE_REPOSITORY,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "render inputs and outputs must use distinct paths"
        );
        assert_eq!(fs::read_to_string(overlay).unwrap(), "Local truth.\n");
        assert!(!agents.exists());
    }

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
        validate_source_import(&root(), &manifest, "Local truth.\n", &[], &agents, COMMIT).unwrap();

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

    fn nested_fixture(directories: &[(&str, &str)]) -> (Manifest, String, Vec<NestedFiles>) {
        assemble_import(
            &root(),
            &["greenfield".into()],
            "Local truth.\n",
            &EMPTY_EXCEPTIONS,
            SOURCE_REPOSITORY,
            COMMIT,
            directories
                .iter()
                .map(|(path, overlay)| (path.to_string(), overlay.to_string()))
                .collect(),
        )
        .unwrap()
    }

    #[test]
    fn nested_directories_must_be_plain_relative_names() {
        assert_eq!(nested_path("orbit").unwrap(), "orbit");
        assert_eq!(nested_path("orbit/./crates/").unwrap(), "orbit/crates");
        for invalid in [
            "",
            ".",
            "/orbit",
            "../orbit",
            "orbit/../venus",
            "or`bit",
            "or\nbit",
        ] {
            assert_error(nested_path(invalid), "nested directory");
        }
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
    fn consumer_check_reads_nested_files_and_rejects_escaping_paths() {
        let directory = TestDirectory::copy_catalog();
        let (mut manifest, agents, files) = nested_fixture(&[("orbit", "Orbit truth.\n")]);
        fs::write(directory.0.join("overlay.md"), "Local truth.\n").unwrap();
        fs::write(
            directory.0.join("exceptions.json"),
            json_text(&EMPTY_EXCEPTIONS).unwrap(),
        )
        .unwrap();
        fs::write(directory.0.join("AGENTS.md"), &agents).unwrap();
        fs::create_dir(directory.0.join("orbit")).unwrap();
        fs::write(
            directory.0.join("orbit").join(LOCAL_OVERLAY),
            "Orbit truth.\n",
        )
        .unwrap();
        fs::write(directory.0.join("orbit/AGENTS.md"), &files[0].agents).unwrap();
        let check = |manifest: &Manifest| {
            fs::write(
                directory.0.join("manifest.json"),
                json_text(manifest).unwrap(),
            )
            .unwrap();
            command_check_consumer(
                &directory.0.join("overlay.md"),
                &directory.0.join("exceptions.json"),
                &directory.0.join("manifest.json"),
                &directory.0.join("AGENTS.md"),
            )
        };
        assert!(check(&manifest).unwrap().contains("1 directory file"));

        fs::write(directory.0.join("orbit/AGENTS.md"), "edited\n").unwrap();
        assert_error(check(&manifest), "orbit/AGENTS.md drifted");

        manifest.nested[0].path = "../orbit".into();
        assert_error(check(&manifest), "nested directory");
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

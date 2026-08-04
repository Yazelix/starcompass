use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

const SOURCE_REPOSITORY: &str = "https://github.com/luccahuguet/starcompass";

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
        }
    }
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
        deserialize_with = "present_string",
        skip_serializing_if = "Option::is_none"
    )]
    review_condition: Option<String>,
    scope: String,
}

fn present_string<'de, D>(deserializer: D) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer).map(Some)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: u64,
    source: Source,
    profiles: Vec<String>,
    protocols: Vec<ImportRecord>,
    exceptions: Vec<Exception>,
    overlay: Overlay,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct Source {
    repository: String,
    commit: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ImportRecord {
    id: String,
    version: u64,
    sha256: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
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
    let expected = BTreeSet::from(["id", "title", "version"]);
    if fields != expected {
        return Err(Error::new(format!(
            "frontmatter fields in {} must be [\"id\", \"title\", \"version\"]",
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
    let body = text[boundary + 5..].trim();
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
        for required in ["AP-SCOPE-001", "AP-EXCEPTION-001"] {
            if !profile.protocols.iter().any(|id| id == required) {
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
        .keys()
        .filter(|id| !used.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    if !unused.is_empty() {
        return Err(Error::new(format!(
            "protocols are not included in any profile: {unused:?}"
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
    for (id, old) in before {
        let Some(new) = after.get(id) else {
            continue;
        };
        if new.version < old.version {
            return Err(Error::new(format!("{id} version regressed")));
        }
        if new.sha256 != old.sha256 && new.version <= old.version {
            return Err(Error::new(format!("{id} changed without a version bump")));
        }
    }
    Ok(())
}

fn validate_exceptions(
    document: Option<&ExceptionsDocument>,
    selected: &BTreeSet<String>,
) -> Result<Vec<Exception>> {
    let Some(document) = document else {
        return Ok(Vec::new());
    };
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
    Ok(document.exceptions.clone())
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
    exceptions_document: Option<&ExceptionsDocument>,
    source_repository: &str,
    source_commit: &str,
) -> Result<(Manifest, String)> {
    let (protocols, profiles) = validate_repository(root)?;
    if profile_names.is_empty() || profile_names.iter().any(String::is_empty) {
        return Err(Error::new("choose one or more unique profiles"));
    }
    let unique_profiles = profile_names.iter().collect::<BTreeSet<_>>();
    if unique_profiles.len() != profile_names.len() {
        return Err(Error::new("choose one or more unique profiles"));
    }
    if source_repository.trim().is_empty() {
        return Err(Error::new("source repository must be a non-empty string"));
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
    let exceptions = validate_exceptions(exceptions_document, &seen)?;
    let manifest = Manifest {
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
        exceptions,
        overlay: Overlay {
            sha256: digest(overlay.as_bytes()),
        },
    };
    let agents = render_agents(&manifest, &selected, overlay);
    Ok((manifest, agents))
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn render_agents(manifest: &Manifest, protocols: &[&Protocol], overlay: &str) -> String {
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
        String::new(),
        "| Protocol | Version | SHA-256 |".into(),
        "| --- | ---: | --- |".into(),
    ];
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
            let review = exception
                .review_condition
                .as_deref()
                .unwrap_or("none recorded");
            lines.push(format!(
                "- `{}` in {}: {}",
                exception.protocol,
                markdown_cell(&exception.scope),
                markdown_cell(&exception.reason)
            ));
            lines.push(format!(
                "  Approved by {} on {}; review condition: {}.",
                markdown_cell(&exception.approved_by),
                exception.approved_at,
                markdown_cell(review)
            ));
        }
    }
    lines.extend([
        String::new(),
        "## Canonical protocols".into(),
        String::new(),
    ]);
    for protocol in protocols {
        lines.extend([
            format!("### {} — {}", protocol.id, protocol.title),
            String::new(),
            protocol.body.clone(),
            String::new(),
        ]);
    }
    lines.extend([
        "## Repository-local rules".into(),
        String::new(),
        match overlay.trim() {
            "" => "No additional local rules.".into(),
            local => local.into(),
        },
        String::new(),
    ]);
    lines.join("\n")
}

fn validate_manifest(
    root: &Path,
    manifest: &Manifest,
    overlay: &str,
    source_commit: Option<&str>,
) -> Result<String> {
    if manifest.schema != 1 {
        return Err(Error::new("import manifest has invalid fields or schema"));
    }
    if let Some(commit) = source_commit {
        if manifest.source.commit != commit {
            return Err(Error::new(format!(
                "source checkout is {commit}, but the manifest pins {}",
                manifest.source.commit
            )));
        }
    }
    let exceptions = ExceptionsDocument {
        schema: 1,
        exceptions: manifest.exceptions.clone(),
    };
    let (wanted, agents) = assemble_import(
        root,
        &manifest.profiles,
        overlay,
        Some(&exceptions),
        &manifest.source.repository,
        &manifest.source.commit,
    )?;
    if manifest != &wanted {
        return Err(Error::new(
            "import manifest drifted from its profile, protocols, or overlay",
        ));
    }
    Ok(agents)
}

fn check_import(
    root: &Path,
    manifest: &Manifest,
    overlay: &str,
    agents: &str,
    source_commit: Option<&str>,
    exceptions_document: Option<&ExceptionsDocument>,
) -> Result<()> {
    let wanted_agents = validate_manifest(root, manifest, overlay, source_commit)?;
    if let Some(document) = exceptions_document {
        let selected = manifest
            .protocols
            .iter()
            .map(|protocol| protocol.id.clone())
            .collect::<BTreeSet<_>>();
        let exceptions = validate_exceptions(Some(document), &selected)?;
        if exceptions != manifest.exceptions {
            return Err(Error::new(
                "exceptions authoring file drifted from the import manifest",
            ));
        }
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
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
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

fn command_render(
    root: &Path,
    profiles: &[String],
    overlay_path: &Path,
    exceptions_path: Option<&Path>,
    manifest_path: &Path,
    agents_path: &Path,
    source_repository: &str,
) -> Result<String> {
    let outputs = [manifest_path, agents_path];
    if manifest_path == agents_path
        || outputs.contains(&overlay_path)
        || exceptions_path.is_some_and(|path| path == overlay_path || outputs.contains(&path))
    {
        return Err(Error::new(
            "render inputs and outputs must use distinct paths",
        ));
    }
    require_clean_source(root)?;
    let overlay = read_text(overlay_path)?;
    let exceptions = exceptions_path.map(read_json).transpose()?;
    let (manifest, agents) = assemble_import(
        root,
        profiles,
        &overlay,
        exceptions.as_ref(),
        source_repository,
        &source_head(root)?,
    )?;
    atomic_write(manifest_path, &json_text(&manifest)?)?;
    atomic_write(agents_path, &agents)?;
    Ok(format!(
        "wrote {} and {}",
        manifest_path.display(),
        agents_path.display()
    ))
}

fn command_check(
    root: &Path,
    overlay_path: &Path,
    exceptions_path: &Path,
    manifest_path: &Path,
    agents_path: &Path,
) -> Result<String> {
    require_clean_source(root)?;
    let overlay = read_text(overlay_path)?;
    let exceptions: ExceptionsDocument = read_json(exceptions_path)?;
    let manifest: Manifest = read_json(manifest_path)?;
    let agents = read_text(agents_path)?;
    check_import(
        root,
        &manifest,
        &overlay,
        &agents,
        Some(&source_head(root)?),
        Some(&exceptions),
    )?;
    Ok(format!(
        "valid: {} matches {}",
        agents_path.display(),
        manifest_path.display()
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
    Ok(take_options(arguments, name)?.into_iter().last())
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

pub fn run<I, S>(arguments: I) -> std::result::Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    run_inner(arguments).map_err(|error| error.to_string())
}

fn run_inner<I, S>(arguments: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let mut arguments = arguments
        .into_iter()
        .map(|argument| {
            argument
                .into()
                .into_string()
                .map_err(|_| Error::new("arguments must be valid UTF-8"))
        })
        .collect::<Result<Vec<_>>>()?;
    if arguments
        .iter()
        .any(|argument| matches!(argument.as_str(), "-h" | "--help"))
    {
        return Ok(help());
    }
    if arguments.is_empty() {
        return Err(Error::new("missing command; run `starcompass --help`"));
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
            let exceptions = take_option(&mut arguments, "--exceptions")?.map(PathBuf::from);
            let manifest = PathBuf::from(require_option(&mut arguments, "--manifest-out")?);
            let agents = PathBuf::from(require_option(&mut arguments, "--agents-out")?);
            let repository = take_option(&mut arguments, "--source-repository")?
                .unwrap_or_else(|| SOURCE_REPOSITORY.into());
            reject_extra(&arguments)?;
            command_render(
                &root,
                &profiles,
                &overlay,
                exceptions.as_deref(),
                &manifest,
                &agents,
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
        _ => Err(Error::new(format!("unknown command: {command}"))),
    }
}

fn help() -> String {
    "usage: starcompass [--root PATH] {index,validate,version-check,render,check}\n\nIndex, render, and verify agent-protocol imports."
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    const COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";
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

    #[test]
    fn protocol_id_validation_matches_the_catalog_grammar() {
        for valid in ["AP-SCOPE-001", "AP-FORK-V2-999"] {
            assert!(valid_protocol_id(valid));
        }
        for invalid in ["AP--001", "AP-lower-001", "AP-SCOPE-01", "SCOPE-001"] {
            assert!(!valid_protocol_id(invalid));
        }
    }

    #[test]
    fn protocol_versions_fill_the_declared_unsigned_range() {
        let directory = TestDirectory::copy_catalog();
        let path = directory.0.join("protocols/AP-SCOPE-001.md");
        let text = fs::read_to_string(&path).unwrap().replacen(
            "version: 1",
            "version: 18446744073709551615",
            1,
        );
        fs::write(&path, text).unwrap();

        assert_eq!(
            parse_protocol(&directory.0, &path).unwrap().version,
            u64::MAX
        );
    }

    #[test]
    fn repository_and_profiles_render_deterministically() {
        let (protocols, profiles) = validate_repository(&root()).unwrap();
        assert_eq!(protocols.len(), 21);
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
            let first = assemble_import(
                &root(),
                &names,
                "Local truth.\n",
                None,
                SOURCE_REPOSITORY,
                COMMIT,
            )
            .unwrap();
            let second = assemble_import(
                &root(),
                &names,
                "Local truth.\n",
                None,
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
    }

    #[test]
    fn release_requires_delivery_before_promotion() {
        let (manifest, agents) = assemble_import(
            &root(),
            &["release".into()],
            "Local truth.\n",
            None,
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
        let document = ExceptionsDocument {
            schema: 1,
            exceptions: vec![exception("AP-CI-001", "private alpha")],
        };
        let (manifest, agents) = assemble_import(
            &root(),
            &["greenfield".into()],
            "Local truth.\n",
            Some(&document),
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        assert_eq!(manifest.exceptions, document.exceptions);
        assert!(agents.contains("Recorded exception."));
        assert!(agents.contains("Approved by user on 2026-08-04"));

        let mut null_review = serde_json::to_value(&document).unwrap();
        null_review["exceptions"][0]["review_condition"] = serde_json::Value::Null;
        assert!(serde_json::from_value::<ExceptionsDocument>(null_review).is_err());

        let mut year_zero = document.clone();
        year_zero.exceptions[0].approved_at = "0000-01-01".into();
        let selected = BTreeSet::from(["AP-CI-001".into()]);
        assert!(
            validate_exceptions(Some(&year_zero), &selected)
                .unwrap_err()
                .to_string()
                .contains("YYYY-MM-DD")
        );

        let invalid = ExceptionsDocument {
            schema: 1,
            exceptions: vec![exception("AP-TERMINAL-001", "all")],
        };
        assert!(
            assemble_import(
                &root(),
                &["greenfield".into()],
                "Local truth.\n",
                Some(&invalid),
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
    fn catalog_rejects_stale_protocols_and_unknown_profile_members() {
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
        let shared = directory.0.join("shared");

        let error = command_render(
            &root(),
            &["greenfield".into()],
            &root().join(".agent-protocols.local.md"),
            None,
            &shared,
            &shared,
            SOURCE_REPOSITORY,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "render inputs and outputs must use distinct paths"
        );
        assert!(!shared.exists());
    }

    #[test]
    fn manifest_check_detects_every_owned_input_drifting() {
        let empty = ExceptionsDocument {
            schema: 1,
            exceptions: Vec::new(),
        };
        let (manifest, agents) = assemble_import(
            &root(),
            &["greenfield".into()],
            "Local truth.\n",
            Some(&empty),
            SOURCE_REPOSITORY,
            COMMIT,
        )
        .unwrap();
        check_import(
            &root(),
            &manifest,
            "Local truth.\n",
            &agents,
            Some(COMMIT),
            Some(&empty),
        )
        .unwrap();

        assert!(
            check_import(
                &root(),
                &manifest,
                "Local truth.\n",
                &(agents.clone() + "manual edit\n"),
                Some(COMMIT),
                Some(&empty),
            )
            .unwrap_err()
            .to_string()
            .contains("AGENTS.md drifted")
        );
        assert!(
            check_import(
                &root(),
                &manifest,
                "Changed local truth.\n",
                &agents,
                Some(COMMIT),
                Some(&empty),
            )
            .unwrap_err()
            .to_string()
            .contains("manifest drifted")
        );
        assert!(
            check_import(
                &root(),
                &manifest,
                "Local truth.\n",
                &agents,
                Some(&"f".repeat(40)),
                Some(&empty),
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
            check_import(
                &root(),
                &manifest,
                "Local truth.\n",
                &agents,
                Some(COMMIT),
                Some(&changed),
            )
            .unwrap_err()
            .to_string()
            .contains("authoring file drifted")
        );
    }

    #[test]
    fn profiles_compose_in_order_without_duplicates() {
        let (manifest, agents) = assemble_import(
            &root(),
            &["greenfield".into(), "terminal-engine".into()],
            "Local truth.\n",
            None,
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

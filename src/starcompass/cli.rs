//! Command dispatch and the side effects commands own: argument parsing,
//! Git queries, and atomic file output.

use super::catalog::{
    ProtocolIndex, expected_index, validate_repository, validate_version_transition,
};
use super::check::{
    read_nested, read_protocol_file, validate_consumer_import, validate_nested,
    validate_source_import,
};
use super::manifest::{
    ExceptionsDocument, LOCAL_OVERLAY, Manifest, nested_path, protocol_file_path, reject_symlinks,
};
use super::render::assemble_import;
use super::{Error, Result, json_text, read_json, read_text, valid_commit_id};
use std::{
    collections::BTreeSet,
    ffi::OsString,
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    process::Command,
};

pub(super) const SOURCE_REPOSITORY: &str = "https://github.com/Yazelix/starcompass";

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

pub(super) fn command_version_check(root: &Path, base: &str) -> Result<String> {
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
    protocol_file: Option<&str>,
    source_repository: &str,
) -> Result<String> {
    let protocol_file = protocol_file.map(protocol_file_path).transpose()?;
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
    let protocol_path = protocol_file.as_ref().map(|file| base.join(file));
    require_distinct_paths(
        [overlay_path, exceptions_path, manifest_path, agents_path]
            .into_iter()
            .chain(protocol_path.as_deref())
            .chain(
                nested_paths
                    .iter()
                    .flat_map(|(input, output)| [input.as_path(), output.as_path()]),
            ),
    )?;
    for path in &directories {
        reject_symlinks(base, path, "nested directory")?;
    }
    if let Some(file) = &protocol_file {
        reject_symlinks(base, file, "protocol file")?;
    }
    require_clean_source(root)?;
    let overlay = read_text(overlay_path)?;
    let exceptions = read_json(exceptions_path)?;
    let nested_overlays = directories
        .into_iter()
        .zip(&nested_paths)
        .map(|(path, (input, _))| Ok((path, read_text(input)?)))
        .collect::<Result<Vec<_>>>()?;
    let (manifest, agents, protocols, files) = assemble_import(
        root,
        profiles,
        &overlay,
        &exceptions,
        source_repository,
        &source_head(root)?,
        nested_overlays,
        protocol_file,
    )?;
    validate_nested(&manifest, &agents, &files)?;
    for ((_, output), file) in nested_paths.iter().zip(&files) {
        atomic_write(output, &file.agents)?;
    }
    if let (Some(path), Some(text)) = (&protocol_path, &protocols) {
        atomic_write(path, text)?;
    }
    atomic_write(manifest_path, &json_text(&manifest)?)?;
    atomic_write(agents_path, &agents)?;
    Ok(format!(
        "wrote {} and {}{}{}",
        manifest_path.display(),
        agents_path.display(),
        directory_files(files.len()),
        protocol_path
            .map(|path| format!(", with protocol text in {}", path.display()))
            .unwrap_or_default()
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
    let protocols = read_protocol_file(consumer_base(agents_path), &manifest)?;
    validate_consumer_import(&manifest, &overlay, &exceptions, &agents)?;
    validate_nested(&manifest, &agents, &nested)?;
    require_clean_source(root)?;
    validate_source_import(
        root,
        &manifest,
        &overlay,
        &nested,
        &agents,
        protocols.as_deref(),
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
    read_protocol_file(consumer_base(agents_path), &manifest)?;
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
            let protocol_file = take_option(&mut arguments, "--protocol-file")?;
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
                protocol_file.as_deref(),
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
    use crate::starcompass::test_support::*;

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
            None,
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

    #[cfg(unix)]
    #[test]
    fn symlinked_consumer_paths_are_rejected_before_reads_or_writes() {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::copy_catalog();
        let consumer = directory.0.join("consumer");
        let outside = directory.0.join("outside");
        fs::create_dir(&consumer).unwrap();
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join(LOCAL_OVERLAY), "Outside truth.\n").unwrap();
        symlink(&outside, consumer.join("orbit")).unwrap();
        symlink(&outside, consumer.join("ext")).unwrap();
        let overlay = consumer.join("overlay.md");
        let exceptions = consumer.join("exceptions.json");
        let manifest_path = consumer.join("manifest.json");
        let agents_path = consumer.join("AGENTS.md");
        fs::write(&overlay, "Local truth.\n").unwrap();
        fs::write(&exceptions, json_text(&EMPTY_EXCEPTIONS).unwrap()).unwrap();

        let render = |nested: &[String], protocol_file: Option<&str>| {
            command_render(
                &root(),
                &["greenfield".into()],
                &overlay,
                &exceptions,
                &manifest_path,
                &agents_path,
                nested,
                protocol_file,
                SOURCE_REPOSITORY,
            )
        };
        assert_error(
            render(&["orbit".into()], None),
            "nested directory \"orbit\" must not pass through a symlink",
        );
        assert_error(
            render(&[], Some("ext/PROTOCOLS.md")),
            "protocol file \"ext/PROTOCOLS.md\" must not pass through a symlink",
        );
        assert_eq!(fs::read_dir(&outside).unwrap().count(), 1);
        assert!(!agents_path.exists() && !manifest_path.exists());

        for (directories, protocol_file) in [
            (&[("orbit", "Orbit truth.\n")][..], None),
            (&[][..], Some("ext/PROTOCOLS.md")),
        ] {
            let (manifest, agents, _, _) = import_fixture(directories, protocol_file);
            fs::write(&manifest_path, json_text(&manifest).unwrap()).unwrap();
            fs::write(&agents_path, agents).unwrap();
            assert_error(
                command_check_consumer(&overlay, &exceptions, &manifest_path, &agents_path),
                "must not pass through a symlink",
            );
        }
    }
}

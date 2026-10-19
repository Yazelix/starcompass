//! Fixtures shared by the module tests.

use super::Result;
use super::cli::SOURCE_REPOSITORY;
use super::manifest::{Exception, ExceptionsDocument, Manifest, NestedFiles};
use super::render::assemble_import;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub(super) const COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";
pub(super) const EMPTY_EXCEPTIONS: ExceptionsDocument = ExceptionsDocument {
    schema: 1,
    exceptions: Vec::new(),
};
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

pub(super) struct TestDirectory(pub(super) PathBuf);

impl TestDirectory {
    pub(super) fn copy_catalog() -> Self {
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

pub(super) fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), destination.join(entry.file_name())).unwrap();
    }
}

pub(super) fn assemble_root(
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
        None,
    )
    .map(|(manifest, agents, _, _)| (manifest, agents))
}

pub(super) fn exception(protocol: &str, scope: &str) -> Exception {
    Exception {
        approved_at: "2026-08-04".into(),
        approved_by: "user".into(),
        protocol: protocol.into(),
        reason: "Recorded exception.".into(),
        review_condition: Some("Review later.".into()),
        scope: scope.into(),
    }
}

pub(super) fn assert_error<T: std::fmt::Debug>(result: Result<T>, expected: &str) {
    assert!(result.unwrap_err().to_string().contains(expected));
}

pub(super) fn import_fixture(
    directories: &[(&str, &str)],
    protocol_file: Option<&str>,
) -> (Manifest, String, Option<String>, Vec<NestedFiles>) {
    assemble_import(
        &root(),
        &["regular".into()],
        "Local truth.\n",
        &EMPTY_EXCEPTIONS,
        SOURCE_REPOSITORY,
        COMMIT,
        directories
            .iter()
            .map(|(path, overlay)| (path.to_string(), overlay.to_string()))
            .collect(),
        protocol_file.map(Into::into),
    )
    .unwrap()
}

pub(super) fn nested_fixture(directories: &[(&str, &str)]) -> (Manifest, String, Vec<NestedFiles>) {
    let (manifest, agents, _, files) = import_fixture(directories, None);
    (manifest, agents, files)
}

use std::{
    fs,
    path::{Path, PathBuf},
};

fn starcompass(arguments: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_starcompass"))
        .args(arguments)
        .output()
        .expect("starcompass should run")
}

fn assert_cli_error(arguments: &[&str], message: &str) {
    let output = starcompass(arguments);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, format!("error: {message}\n").into_bytes());
}

struct ConsumerFixture(PathBuf);

impl ConsumerFixture {
    fn new() -> Self {
        let path =
            std::env::temp_dir().join(format!("starcompass-consumer-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir(&path).unwrap();
        for name in [
            ".agent-protocols.local.md",
            ".agent-protocols.exceptions.json",
            ".agent-protocols.json",
            "AGENTS.md",
        ] {
            fs::copy(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(name),
                path.join(name),
            )
            .unwrap();
        }
        let binary = PathBuf::from(env!("CARGO_BIN_EXE_starcompass"));
        fs::copy(&binary, path.join(binary.file_name().unwrap())).unwrap();
        Self(path)
    }

    fn run(&self) -> std::process::Output {
        let binary = PathBuf::from(env!("CARGO_BIN_EXE_starcompass"));
        std::process::Command::new(self.path(binary.file_name().unwrap()))
            .current_dir(&self.0)
            .args([
                "check-consumer",
                "--overlay",
                ".agent-protocols.local.md",
                "--exceptions",
                ".agent-protocols.exceptions.json",
                "--manifest",
                ".agent-protocols.json",
                "--agents",
                "AGENTS.md",
            ])
            .output()
            .expect("starcompass should run")
    }

    fn path(&self, name: impl AsRef<Path>) -> PathBuf {
        self.0.join(name)
    }

    fn assert_mutation_rejected(
        &self,
        name: &str,
        expected: &str,
        mutate: impl FnOnce(&str) -> String,
    ) {
        let path = self.path(name);
        let original = fs::read_to_string(&path).unwrap();
        fs::write(&path, mutate(&original)).unwrap();
        let output = self.run();
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8(output.stderr).unwrap().contains(expected));
        fs::write(path, original).unwrap();
    }
}

impl Drop for ConsumerFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn check_consumer_validates_files_without_a_source_checkout() {
    let fixture = ConsumerFixture::new();
    assert!(fixture.0.ancestors().all(|root| {
        !(root.join(".git").exists()
            || root.join("protocols/index.json").is_file() && root.join("profiles").is_dir())
    }));

    let output = fixture.run();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "valid: consumer files agree; canonical protocol prose and manifest protocol digests were not verified against source\n"
    );
    assert!(output.stderr.is_empty());

    assert_cli_error(
        &["--root", env!("CARGO_MANIFEST_DIR"), "check-consumer"],
        "--root is not valid with check-consumer",
    );

    fixture.assert_mutation_rejected(
        ".agent-protocols.json",
        "manifest must use schema 1",
        |text| text.replacen("\"schema\": 1", "\"schema\": 2", 1),
    );
    fixture.assert_mutation_rejected(
        ".agent-protocols.local.md",
        "manifest overlay drifted",
        |text| format!("{text}drift\n"),
    );
    fixture.assert_mutation_rejected(
        ".agent-protocols.exceptions.json",
        "exceptions document must use schema 1",
        |text| text.replacen("\"schema\": 1", "\"schema\": 2", 1),
    );
    fixture.assert_mutation_rejected("AGENTS.md", "protocol import record", |text| {
        text.replacen("| `AP-SCOPE-001` |", "| `AP-NOT-REAL-999` |", 1)
    });
    fixture.assert_mutation_rejected("AGENTS.md", "repository-local rules", |text| {
        format!("{text}manual edit\n")
    });
}

#[test]
fn validates_the_canonical_repository() {
    let output = starcompass(&["--root", env!("CARGO_MANIFEST_DIR"), "validate"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "valid: 24 protocols, 5 profiles\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn invalid_cli_inputs_are_actionable_and_fail() {
    assert_cli_error(&["not-real"], "unknown command: not-real");
    assert_cli_error(
        &["render", "--profile=regular", "--overlay=overlay"],
        "missing required --exceptions",
    );
    assert_cli_error(
        &[
            "check-consumer",
            "--overlay",
            "first",
            "--overlay",
            "second",
        ],
        "--overlay may be used only once",
    );
}

#[test]
fn global_help_works_after_the_root_option() {
    let output = starcompass(&["--root", env!("CARGO_MANIFEST_DIR"), "--help"]);

    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .starts_with("usage:\n  starcompass ")
    );
    assert!(output.stderr.is_empty());
}

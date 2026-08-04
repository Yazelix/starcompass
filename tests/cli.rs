#[test]
fn validates_the_canonical_repository() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_starcompass"))
        .args(["--root", env!("CARGO_MANIFEST_DIR"), "validate"])
        .output()
        .expect("starcompass should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "valid: 21 protocols, 5 profiles\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn invalid_command_is_actionable_and_fails() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_starcompass"))
        .arg("not-real")
        .output()
        .expect("starcompass should run");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown command: not-real\n"
    );
}

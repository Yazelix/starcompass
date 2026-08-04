fn starcompass(arguments: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_starcompass"))
        .args(arguments)
        .output()
        .expect("starcompass should run")
}

#[test]
fn validates_the_canonical_repository() {
    let output = starcompass(&["--root", env!("CARGO_MANIFEST_DIR"), "validate"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "valid: 21 protocols, 5 profiles\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn invalid_command_is_actionable_and_fails() {
    let output = starcompass(&["not-real"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown command: not-real\n"
    );
}

#[test]
fn global_help_works_after_the_root_option() {
    let output = starcompass(&["--root", env!("CARGO_MANIFEST_DIR"), "--help"]);

    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .starts_with("usage: starcompass ")
    );
    assert!(output.stderr.is_empty());
}

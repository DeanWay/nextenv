use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        let examples_dir = Path::new(".").parent().unwrap().join("examples");
        env::set_current_dir(&examples_dir).unwrap();
    });
}

#[test]
fn simple_dotenv_and_local() {
    setup();

    let expectations = [
        ("A", ".env.local"),
        ("B", ".env.local"),
        ("C", ".env"),
        ("D", ".env"),
    ];

    for (key, expected_val) in expectations {
        let mut cmd = Command::new("nextenv");
        cmd.args(&["--", "printenv", key]);

        cmd.assert()
            .success()
            .stdout(predicate::eq(format!("{}\n", expected_val)));
    }
}

#[test]
fn specified_environment_development() {
    setup();

    let expectations = [
        ("A", ".env.development.local"),
        ("B", ".env.local"),
        ("C", ".env.development"),
        ("D", ".env"),
    ];

    for (key, expected_val) in expectations {
        let mut cmd = Command::new("nextenv");
        cmd.args(&["-e", "development", "--", "printenv", key]);

        cmd.assert()
            .success()
            .stdout(predicate::eq(format!("{}\n", expected_val)));
    }
}

#[test]
fn specified_environment_production() {
    setup();

    let expectations = [
        ("A", ".env.local"),
        ("B", ".env.local"),
        ("C", ".env.production"),
        ("D", ".env"),
    ];

    for (key, expected_val) in expectations {
        let mut cmd = Command::new("nextenv");
        cmd.args(&["-e", "production", "--", "printenv", key]);

        cmd.assert()
            .success()
            .stdout(predicate::eq(format!("{}\n", expected_val)));
    }
}

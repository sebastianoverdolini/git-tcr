//! End-to-end tests: run the real compiled binary against a real git repo
//! in a temp directory, with no mocking of git or the filesystem.
//!
//! These complement (not replace) the mocked unit tests in `src/`, which
//! cover branch logic quickly and exhaustively at the `exec` boundary.
//! These catch the class of bug a mock can't: a genuinely wrong git
//! invocation, or a wrong assumption about how git actually behaves.

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Initializes a git repo in `dir` with a local (not global) identity, so
/// commits work even on a CI runner with no git user configured.
fn init_repo(dir: &Path) {
    run(dir, "git", &["init", "-q"]);
    run(dir, "git", &["config", "user.email", "test@example.com"]);
    run(dir, "git", &["config", "user.name", "Test"]);
    run(dir, "git", &["config", "commit.gpgsign", "false"]);
}

fn run(dir: &Path, program: &str, args: &[&str]) -> std::process::Output {
    let output = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {program} {args:?}: {e}"));
    assert!(
        output.status.success(),
        "{program} {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn write_tcr_yaml(dir: &Path, program: &str) {
    fs::write(dir.join("tcr.yaml"), format!("test:\n  program: \"{program}\"\n  args: []\n"))
        .expect("write tcr.yaml");
}

fn git_tcr(dir: &Path) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_git-tcr"));
    cmd.current_dir(dir);
    cmd
}

fn log(dir: &Path) -> String {
    let output = run(dir, "git", &["log", "--pretty=%B"]);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn status(dir: &Path) -> String {
    let output = run(dir, "git", &["status", "--porcelain"]);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn commits_when_the_test_passes() {
    let temp = TempDir::new().unwrap();
    let dir = temp.path();
    init_repo(dir);
    write_tcr_yaml(dir, "true");
    fs::write(dir.join("foo.txt"), "hello").unwrap();

    let status_code = git_tcr(dir).status().expect("run git-tcr");

    assert!(status_code.success());
    assert_eq!(log(dir), "WIP");
    assert_eq!(status(dir), "", "working tree should be clean after commit");
    assert_eq!(fs::read_to_string(dir.join("foo.txt")).unwrap(), "hello");
}

#[test]
fn reverts_when_the_test_fails() {
    let temp = TempDir::new().unwrap();
    let dir = temp.path();
    init_repo(dir);
    write_tcr_yaml(dir, "true");
    fs::write(dir.join("tracked.txt"), "original").unwrap();
    run(dir, "git", &["add", "."]);
    run(dir, "git", &["commit", "-m", "seed"]);

    // Now make the test fail and dirty the working tree.
    write_tcr_yaml(dir, "false");
    fs::write(dir.join("tracked.txt"), "changed").unwrap();
    fs::write(dir.join("untracked.txt"), "new").unwrap();

    let status_code = git_tcr(dir).status().expect("run git-tcr");

    assert!(!status_code.success());
    assert_eq!(log(dir), "seed", "no new commit should have been made");
    assert_eq!(fs::read_to_string(dir.join("tracked.txt")).unwrap(), "original");
    assert!(!dir.join("untracked.txt").exists(), "untracked file should be cleaned up");
    assert_eq!(status(dir), "", "working tree should match the last commit");
}

#[test]
fn uses_custom_message_and_trailers() {
    let temp = TempDir::new().unwrap();
    let dir = temp.path();
    init_repo(dir);
    write_tcr_yaml(dir, "true");
    fs::write(dir.join("foo.txt"), "hello").unwrap();

    let status_code = git_tcr(dir)
        .args(["-m", "feat: add foo", "--trailer", "Issue: GDT-42"])
        .status()
        .expect("run git-tcr");

    assert!(status_code.success());
    assert_eq!(log(dir), "feat: add foo\n\nIssue: GDT-42");
}

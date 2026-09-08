use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use crate::config::{Config, TestConfig, TestSpec, MAX_SUPPORTED_VERSION};

/// Interactively builds a `tcr.yaml` in `location`, prompting on `output`
/// and reading answers from `input`.
///
/// Each test command is actually run (via `run`) before being kept: a
/// command that fails to run successfully is rejected automatically,
/// without asking, and the user has to type it again. One that runs
/// successfully is still confirmed with the user before being added, in
/// case it ran but isn't the command they meant to use.
///
/// Returns `Ok(true)` if the file was written, `Ok(false)` if the user
/// declined to overwrite an existing file.
pub fn init(
    location: impl Into<PathBuf>,
    input: &mut dyn BufRead,
    output: &mut dyn Write,
    run: &dyn Fn(&mut Command) -> io::Result<ExitStatus>,
) -> io::Result<bool> {
    let dir = location.into();
    let config_path = dir.join("tcr.yaml");

    if config_path.exists() {
        write!(output, "'tcr.yaml' already exists. Overwrite? [y/N] ")?;
        output.flush()?;
        if !read_answer(input, false)? {
            writeln!(output, "Aborted.")?;
            return Ok(false);
        }
    }

    let mut tests = Vec::new();
    loop {
        if tests.is_empty() {
            write!(output, "Test command to run (e.g. \"npm test\"): ")?;
        } else {
            write!(output, "Another test command? (leave empty to finish): ")?;
        }
        output.flush()?;

        let line = read_line(input)?;
        let line = line.trim();
        if line.is_empty() {
            if tests.is_empty() {
                writeln!(output, "A test command is required.")?;
                continue;
            }
            break;
        }

        let mut parts = line.split_whitespace();
        let program = match parts.next() {
            Some(program) => program.to_string(),
            None => continue,
        };
        let args: Vec<String> = parts.map(String::from).collect();

        writeln!(output, "Running \"{line}\"...")?;
        let mut cmd = Command::new(&program);
        cmd.args(&args);
        let succeeded = run(&mut cmd).map(|status| status.success()).unwrap_or(false);
        if !succeeded {
            writeln!(output, "\u{2717} \"{line}\" didn't run successfully \u{2014} try again.")?;
            continue;
        }

        write!(output, "Add this test command? [Y/n] ")?;
        output.flush()?;
        if !read_answer(input, true)? {
            continue;
        }

        tests.push(TestConfig { program, args });
    }

    write!(output, "Skip git hooks with --no-verify on commit? [y/N] ")?;
    output.flush()?;
    let no_verify = read_answer(input, false)?;

    let test = if tests.len() == 1 {
        TestSpec::Single(tests.remove(0))
    } else {
        TestSpec::Multiple(tests)
    };
    let config = Config {
        version: MAX_SUPPORTED_VERSION,
        test,
        no_verify: if no_verify { Some(true) } else { None },
    };

    let yaml = serde_yaml::to_string(&config).expect("config always serializes to yaml");
    std::fs::write(&config_path, yaml)?;
    writeln!(output, "Wrote {}", config_path.display())?;

    Ok(true)
}

/// Reads one line, distinguishing a real end of input from a blank line the
/// user just pressed enter on: an empty `String` means the latter, while
/// input genuinely running out (e.g. a pipe with too few lines) is an
/// error, so callers waiting on a required answer can't spin forever
/// re-reading an exhausted stream.
fn read_line(input: &mut dyn BufRead) -> io::Result<String> {
    let mut line = String::new();
    let bytes_read = input.read_line(&mut line)?;
    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "no more input while creating tcr.yaml"));
    }
    Ok(line)
}

/// Reads a y/n answer; a bare enter (empty answer) takes `default`.
fn read_answer(input: &mut dyn BufRead, default: bool) -> io::Result<bool> {
    let line = read_line(input)?;
    let answer = line.trim().to_lowercase();
    if answer.is_empty() {
        return Ok(default);
    }
    Ok(answer == "y" || answer == "yes")
}

#[cfg(test)]
mod init_tests {
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::io::{self, Cursor};
    use std::os::unix::process::ExitStatusExt;
    use std::path::Path;
    use std::process::{Command, ExitStatus};
    use crate::config;
    use crate::config::{Config, TestConfig, TestSpec, MAX_SUPPORTED_VERSION};
    use super::init;

    /// A fake `run` that reports every command as having run successfully,
    /// without actually spawning anything.
    fn succeeds(_cmd: &mut Command) -> io::Result<ExitStatus> {
        Ok(ExitStatus::from_raw(0))
    }

    /// A fake `run` that reports failure for `bad_program` and success for
    /// everything else, without actually spawning anything.
    fn fails_for(bad_program: &'static str) -> impl Fn(&mut Command) -> io::Result<ExitStatus> {
        move |cmd: &mut Command| {
            let failed = cmd.get_program().to_string_lossy() == bad_program;
            Ok(ExitStatus::from_raw(if failed { 1 } else { 0 }))
        }
    }

    fn run(dir: &Path, answers: &str) -> String {
        run_with(dir, answers, &succeeds)
    }

    fn run_with(dir: &Path, answers: &str, exec: &dyn Fn(&mut Command) -> io::Result<ExitStatus>) -> String {
        let mut input = Cursor::new(answers.as_bytes().to_vec());
        let mut output = Vec::new();
        init(dir, &mut input, &mut output, exec).expect("init succeeds");
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn it_writes_a_single_test_command() {
        let test_dir = "test-env-init-single";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        run(Path::new(test_dir), "npm test\n\n\nn\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_writes_multiple_test_commands() {
        let test_dir = "test-env-init-multiple";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        run(Path::new(test_dir), "tsc --noEmit\n\nnpm run test\n\n\nn\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Multiple(vec![
                TestConfig { program: String::from("tsc"), args: vec![String::from("--noEmit")] },
                TestConfig { program: String::from("npm"), args: vec![String::from("run"), String::from("test")] },
            ]),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_sets_no_verify_when_confirmed() {
        let test_dir = "test-env-init-no-verify";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        run(Path::new(test_dir), "npm test\n\n\ny\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: Some(true),
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_reprompts_until_a_test_command_is_given() {
        let test_dir = "test-env-init-reprompt";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        let output = run(Path::new(test_dir), "\n\nnpm test\n\n\nn\n");
        assert!(output.contains("A test command is required."));

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_errors_instead_of_looping_forever_when_input_runs_out() {
        let test_dir = "test-env-init-eof";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        // No answers at all: the very first prompt (for a required test
        // command) hits end of input immediately.
        let mut input = Cursor::new(Vec::new());
        let mut output = Vec::new();
        let result = init(Path::new(test_dir), &mut input, &mut output, &succeeds);

        let err = result.expect_err("should error rather than loop forever on exhausted input");
        assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_rejects_a_command_that_fails_to_run_without_asking_for_confirmation() {
        let test_dir = "test-env-init-run-fails";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        let output = run_with(
            Path::new(test_dir),
            "badcmd\nnpm test\n\n\nn\n",
            &fails_for("badcmd"),
        );
        assert!(output.contains("didn't run successfully"), "output was: {output}");
        // Only the successful retry ("npm test") should ever reach the
        // confirmation prompt — the failed "badcmd" attempt must not.
        assert_eq!(
            output.matches("Add this test command?").count(), 1,
            "expected exactly one confirmation prompt, output was: {output}"
        );

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_lets_the_user_decline_a_command_that_ran_successfully() {
        let test_dir = "test-env-init-decline-success";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        run(Path::new(test_dir), "npm test\nn\nnpm run test\n\n\nn\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("run"), String::from("test")],
            }),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_aborts_without_overwriting_when_declined() {
        let test_dir = "test-env-init-abort";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");
        write(format!("{}/tcr.yaml", test_dir), "test:\n  program: \"existing\"\n  args: []\n")
            .expect("Failed to write existing config");

        let mut input = Cursor::new(b"n\n".to_vec());
        let mut output = Vec::new();
        let wrote = init(Path::new(test_dir), &mut input, &mut output, &succeeds).expect("init succeeds");

        assert_eq!(wrote, false);
        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Single(TestConfig {
                program: String::from("existing"),
                args: vec![],
            }),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_overwrites_when_confirmed() {
        let test_dir = "test-env-init-overwrite";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");
        write(format!("{}/tcr.yaml", test_dir), "test:\n  program: \"existing\"\n  args: []\n")
            .expect("Failed to write existing config");

        run(Path::new(test_dir), "y\nnpm test\n\n\nn\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Ok(Config {
            version: MAX_SUPPORTED_VERSION,
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }
}

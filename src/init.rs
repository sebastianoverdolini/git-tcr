use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use crate::config::{Config, TestConfig, TestSpec};

/// Interactively builds a `tcr.yaml` in `location`, prompting on `output`
/// and reading answers from `input`.
///
/// Returns `Ok(true)` if the file was written, `Ok(false)` if the user
/// declined to overwrite an existing file.
pub fn init(location: impl Into<PathBuf>, input: &mut dyn BufRead, output: &mut dyn Write) -> io::Result<bool> {
    let dir = location.into();
    let config_path = dir.join("tcr.yaml");

    if config_path.exists() {
        write!(output, "'tcr.yaml' already exists. Overwrite? [y/N] ")?;
        output.flush()?;
        if !read_yes(input)? {
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
        let args = parts.map(String::from).collect();
        tests.push(TestConfig { program, args });
    }

    write!(output, "Skip git hooks with --no-verify on commit? [y/N] ")?;
    output.flush()?;
    let no_verify = read_yes(input)?;

    let test = if tests.len() == 1 {
        TestSpec::Single(tests.remove(0))
    } else {
        TestSpec::Multiple(tests)
    };
    let config = Config {
        test,
        no_verify: if no_verify { Some(true) } else { None },
    };

    let yaml = serde_yaml::to_string(&config).expect("config always serializes to yaml");
    std::fs::write(&config_path, yaml)?;
    writeln!(output, "Wrote {}", config_path.display())?;

    Ok(true)
}

fn read_line(input: &mut dyn BufRead) -> io::Result<String> {
    let mut line = String::new();
    input.read_line(&mut line)?;
    Ok(line)
}

fn read_yes(input: &mut dyn BufRead) -> io::Result<bool> {
    let line = read_line(input)?;
    let answer = line.trim().to_lowercase();
    Ok(answer == "y" || answer == "yes")
}

#[cfg(test)]
mod init_tests {
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::io::Cursor;
    use std::path::Path;
    use crate::config;
    use crate::config::{Config, TestConfig, TestSpec};
    use super::init;

    fn run(dir: &Path, answers: &str) -> String {
        let mut input = Cursor::new(answers.as_bytes().to_vec());
        let mut output = Vec::new();
        init(dir, &mut input, &mut output).expect("init succeeds");
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn it_writes_a_single_test_command() {
        let test_dir = "test-env-init-single";
        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        run(Path::new(test_dir), "npm test\n\nn\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Some(Config {
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

        run(Path::new(test_dir), "tsc --noEmit\nnpm run test\n\nn\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Some(Config {
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

        run(Path::new(test_dir), "npm test\n\ny\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Some(Config {
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

        let output = run(Path::new(test_dir), "\n\nnpm test\n\nn\n");
        assert!(output.contains("A test command is required."));

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Some(Config {
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
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
        let wrote = init(Path::new(test_dir), &mut input, &mut output).expect("init succeeds");

        assert_eq!(wrote, false);
        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Some(Config {
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

        run(Path::new(test_dir), "y\nnpm test\n\nn\n");

        let result = config::yaml_config(Path::new(test_dir));
        assert_eq!(result, Some(Config {
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: None,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }
}

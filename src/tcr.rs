use std::fmt;
use std::process::{Output};
use crate::commit::{Commit, CommitConfig};
use crate::config::Config;

pub fn tcr(
    config: fn() -> Option<Config>,
    commit: Commit,
    exec: impl Fn(&str, &[&str]) -> Result<Output, std::io::Error>,
) -> Result<(), ConfigurationNotFound>
{
    let config = config().ok_or(ConfigurationNotFound)?;

    let commit_cmd = commit(CommitConfig { no_verify: config.clone().no_verify });

    exec("git", &["add", "."]).expect("git add . works");
    let test_result = exec("sh", &["-c", &config.clone().test]).expect("test command works");

    match test_result.status.success() {
        true => {
            exec("sh", &["-c", &commit_cmd]).expect("commit command works");
        },
        false => {
            exec("git", &["clean", "-fdq", "."]).expect("revert command works");
            exec("git", &["reset", "--hard"]).expect("revert command works");
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationNotFound;

impl fmt::Display for ConfigurationNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration not found.")
    }
}

#[cfg(test)]
mod tcr_command_test
{
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;
    use std::process::{Output, ExitStatus};
    use std::os::unix::process::ExitStatusExt;
    use crate::config::Config;
    use crate::tcr::{tcr};

    fn dummy_output_with_status(code: i32) -> Output {
        Output {
            status: ExitStatus::from_raw(code),
            stdout: vec![],
            stderr: vec![],
        }
    }

    fn setup_mock_with_status(statuses: Vec<i32>) -> (Rc<RefCell<Vec<(String, Vec<String>)>>>, impl Fn(&str, &[&str]) -> Result<Output, std::io::Error>) {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let statuses = Rc::new(RefCell::new(statuses));
        let mock_exec = {
            let captured_calls = Rc::clone(&captured_calls);
            let statuses = Rc::clone(&statuses);
            move |program: &str, args: &[&str]| {
                captured_calls.borrow_mut().push((
                    program.to_string(),
                    args.iter().map(|&s| s.to_string()).collect(),
                ));
                let code = statuses.borrow_mut().remove(0);
                Ok(dummy_output_with_status(code))
            }
        };
        (captured_calls, mock_exec)
    }

    #[test]
    fn green_scenario_test_and_commit() {
        // test passes (exit code 0), so commit is run
        let (captured_calls, mock_exec) = setup_mock_with_status(vec![0, 0, 0]);
        let res = tcr(
            || Some(Config {
                test: String::from("test"),
                no_verify: Some(true),
            }),
            |config| format!("commit {config:?}").to_string(),
            mock_exec,
        );
        assert!(res.is_ok());
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0], ("git".to_string(), vec!["add".to_string(), ".".to_string()]));
        assert_eq!(calls[1], ("sh".to_string(), vec!["-c".to_string(), "test".to_string()]));
        assert_eq!(calls[2], ("sh".to_string(), vec!["-c".to_string(), "commit CommitConfig { no_verify: Some(true) }".to_string()]));
    }

    #[test]
    fn red_scenario_test_and_revert() {
        // test fails (exit code 1), so revert is run
        let (captured_calls, mock_exec) = setup_mock_with_status(vec![0, 1, 0, 0]);
        let res = tcr(
            || Some(Config {
                test: String::from("test"),
                no_verify: Some(false),
            }),
            |config| format!("commit {config:?}").to_string(),
            mock_exec,
        );
        assert!(res.is_ok());
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 4);
        assert_eq!(calls[0], ("git".to_string(), vec!["add".to_string(), ".".to_string()]));
        assert_eq!(calls[1], ("sh".to_string(), vec!["-c".to_string(), "test".to_string()]));
        assert_eq!(calls[2], ("git".to_string(), vec!["clean".to_string(), "-fdq".to_string(), ".".to_string()]));
        assert_eq!(calls[3], ("git".to_string(), vec!["reset".to_string(), "--hard".to_string()]));
    }

    #[test]
    fn no_conf() {
        let (captured_calls, mock_exec) = setup_mock_with_status(vec![]);
        let res = tcr(
            || None,
            |config| format!("commit {config:?}").to_string(),
            mock_exec,
        );
        assert!(res.is_err());
        let calls: Ref<Vec<(String, Vec<String>)>> = captured_calls.borrow();
        assert_eq!(calls.len(), 0);
    }
}

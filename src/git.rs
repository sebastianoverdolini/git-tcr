use std::process::{Command, Output};
use crate::config::Config;
use crate::tcr::Repository;

pub struct GitRepository {
    pub config: Config,
    pub exec: Box<dyn Fn(&mut Command) -> Result<Output, std::io::Error> + 'static>,
    pub message: fn(&str) -> String,
}

impl Repository for GitRepository {
    fn stage(&self) {
        (self.exec)(&mut Command::new("git").args(["add", "."]))
            .expect("git add . works");
    }

    fn revert(&self) {
        (self.exec)(&mut Command::new("git").args(["clean", "-fdq", "."]))
            .expect("clean command works");
        (self.exec)(&mut Command::new("git").args(["reset", "--hard"]))
            .expect("revert command works");
    }

    fn commit(&self) {
        let diff_output = (self.exec)(&mut Command::new("git").args(["diff", "--staged", "--color=never", "-U0"]))
            .expect("git diff works");
        let diff_str = String::from_utf8_lossy(&diff_output.stdout);
        let commit_message = (self.message)(&diff_str);
        let mut cmd = Command::new("git");
        cmd.arg("commit").arg("-m").arg(&commit_message);
        if self.config.clone().no_verify.unwrap_or(false) {
            cmd.arg("--no-verify");
        }
        (self.exec)(&mut cmd).expect("commit command works");
    }

    fn test(&self) -> bool {
        let mut cmd = Command::new(&self.config.test.program);
        for arg in &self.config.test.args {
            cmd.arg(arg);
        }
        (self.exec)(&mut cmd.stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit()))
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod git_test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::process::{Output, Command};
    use std::os::unix::process::ExitStatusExt;
    use crate::config::{Config, TestConfig};
    use crate::tcr::Repository;

    fn extract_cmd(cmd: &Command) -> (String, Vec<String>) {
        let program = cmd.get_program().to_string_lossy().to_string();
        let args = cmd.get_args().map(|a| a.to_string_lossy().to_string()).collect();
        (program, args)
    }

    fn setup_mock() -> (Rc<RefCell<Vec<(String, Vec<String>)>>>, impl Fn(&mut Command) -> Result<Output, std::io::Error>) {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let mock_exec = {
            let captured_calls = Rc::clone(&captured_calls);
            move |cmd: &mut Command| {
                let (program, args) = extract_cmd(cmd);
                captured_calls.borrow_mut().push((program.clone(), args.clone()));
                // Return fake diff for git diff
                if program == "git" && args == ["diff", "--staged", "--color=never", "-U0"] {
                    Ok(Output {
                        status: std::process::ExitStatus::from_raw(0),
                        stdout: b"fake-diff".to_vec(),
                        stderr: vec![],
                    })
                } else {
                    Ok(Output {
                        status: std::process::ExitStatus::from_raw(0),
                        stdout: vec![],
                        stderr: vec![],
                    })
                }
            }
        };
        (captured_calls, mock_exec)
    }

    #[test]
    fn stage_test() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: TestConfig { program: "foo".to_string(), args: vec![] }, no_verify: Some(true) },
            message: |_diff| "WIP".to_string(),
        };
        git.stage();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("git".to_string(), vec!["add".to_string(), ".".to_string()]));
    }

    #[test]
    fn revert_test() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: TestConfig { program: "foo".to_string(), args: vec![] }, no_verify: Some(true) },
            message: |_diff| "WIP".to_string(),
        };
        git.revert();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("git".to_string(), vec!["clean".to_string(), "-fdq".to_string(), ".".to_string()]));
        assert_eq!(calls[1], ("git".to_string(), vec!["reset".to_string(), "--hard".to_string()]));
    }

    #[test]
    fn commit_no_verify() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: TestConfig { program: "foo".to_string(), args: vec![] }, no_verify: Some(true) },
            message: |diff| format!("WIP: {diff}"),
        };
        git.commit();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("git".to_string(), vec!["diff".to_string(), "--staged".to_string(), "--color=never".to_string(), "-U0".to_string()]));
        assert_eq!(calls[1], ("git".to_string(), vec!["commit".to_string(), "-m".to_string(), "WIP: fake-diff".to_string(), "--no-verify".to_string()]));
    }

    #[test]
    fn commit_verifying() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: TestConfig { program: "foo".to_string(), args: vec![] }, no_verify: Some(false) },
            message: |diff| format!("WIP: {diff}"),
        };
        git.commit();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("git".to_string(), vec!["diff".to_string(), "--staged".to_string(), "--color=never".to_string(), "-U0".to_string()]));
        assert_eq!(calls[1], ("git".to_string(), vec!["commit".to_string(), "-m".to_string(), "WIP: fake-diff".to_string()]));
    }

    #[test]
    fn commit_none() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: TestConfig { program: "foo".to_string(), args: vec![] }, no_verify: None },
            message: |diff| format!("WIP: {diff}"),
        };
        git.commit();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("git".to_string(), vec!["diff".to_string(), "--staged".to_string(), "--color=never".to_string(), "-U0".to_string()]));
        assert_eq!(calls[1], ("git".to_string(), vec!["commit".to_string(), "-m".to_string(), "WIP: fake-diff".to_string()]));
    }

    #[test]
    fn test_success() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: TestConfig { program: "cargo".to_string(), args: vec!["test".to_string()] }, no_verify: None },
            message: |_diff| "WIP".to_string(),
        };
        let result = git.test();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("cargo".to_string(), vec!["test".to_string()]));
        assert_eq!(result, true);
    }

    #[test]
    fn test_failure() {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let mock_exec = {
            let captured_calls = Rc::clone(&captured_calls);
            move |cmd: &mut Command| {
                let (program, args) = extract_cmd(cmd);
                captured_calls.borrow_mut().push((program.clone(), args.clone()));
                Ok(Output {
                    status: std::process::ExitStatus::from_raw(1),
                    stdout: vec![],
                    stderr: vec![],
                })
            }
        };
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: TestConfig { program: "cargo".to_string(), args: vec!["test".to_string()] }, no_verify: None },
            message: |_diff| "WIP".to_string(),
        };
        let result = git.test();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("cargo".to_string(), vec!["test".to_string()]));
        assert_eq!(result, false);
    }
}

use std::process::Output;
use crate::config::Config;
use crate::tcr::Repository;

pub struct GitRepository {
    pub config: Config,
    pub exec: Box<dyn Fn(&str, &[&str]) -> Result<Output, std::io::Error> + 'static>,
}

impl Repository for GitRepository {
    fn revert(&self) {
        (self.exec)("git", &["clean", "-fdq", "."]).expect("revert command works");
        (self.exec)("git", &["reset", "--hard"]).expect("revert command works");
    }

    fn commit(&self) {
        (self.exec)("git", &["add", "."]).expect("git add . works");
        let commit_command = vec!["commit", "-m", "WIP"].into_iter()
            .chain(self.config.clone().no_verify.unwrap_or(false).then_some("--no-verify"))
            .collect::<Vec<_>>();
        (self.exec)("git", &commit_command).expect("commit command works");
    }

    fn test(&self) -> bool {
        (self.exec)("sh", &["-c", &self.config.clone().test])
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod git_test
{
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::process::Output;
    use std::os::unix::process::ExitStatusExt;
    use crate::config::Config;
    use crate::tcr::Repository;

    fn setup_mock() -> (Rc<RefCell<Vec<(String, Vec<String>)>>>, impl Fn(&str, &[&str]) -> Result<Output, std::io::Error>) {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let mock_exec = {
            let captured_calls = Rc::clone(&captured_calls);
            move |program: &str, args: &[&str]| {
                captured_calls.borrow_mut().push((
                    program.to_string(),
                    args.iter().map(|&s| s.to_string()).collect(),
                ));
                Ok(Output {
                    status: std::process::ExitStatus::from_raw(0),
                    stdout: vec![],
                    stderr: vec![],
                })
            }
        };
        (captured_calls, mock_exec)
    }

    #[test]
    fn revert_test() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: "foo".to_string(), no_verify: Some(true) },
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
            config: Config { test: "foo".to_string(), no_verify: Some(true) },
        };
        git.commit();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("git".to_string(), vec!["add".to_string(), ".".to_string()]));
        assert_eq!(calls[1], ("git".to_string(), vec!["commit".to_string(), "-m".to_string(), "WIP".to_string(), "--no-verify".to_string()]));
    }

    #[test]
    fn commit_verifying() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: "foo".to_string(), no_verify: Some(false) },
        };
        git.commit();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("git".to_string(), vec!["add".to_string(), ".".to_string()]));
        assert_eq!(calls[1], ("git".to_string(), vec!["commit".to_string(), "-m".to_string(), "WIP".to_string()]));
    }

    #[test]
    fn commit_none() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: "foo".to_string(), no_verify: None },
        };
        git.commit();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], ("git".to_string(), vec!["add".to_string(), ".".to_string()]));
        assert_eq!(calls[1], ("git".to_string(), vec!["commit".to_string(), "-m".to_string(), "WIP".to_string()]));
    }

    #[test]
    fn test_success() {
        let (captured_calls, mock_exec) = setup_mock();
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: "foo".to_string(), no_verify: None },
        };
        let result = git.test();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("sh".to_string(), vec!["-c".to_string(), "foo".to_string()]));
        assert_eq!(result, true);
    }

    #[test]
    fn test_failure() {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let mock_exec = {
            let captured_calls = Rc::clone(&captured_calls);
            move |program: &str, args: &[&str]| {
                captured_calls.borrow_mut().push((
                    program.to_string(),
                    args.iter().map(|&s| s.to_string()).collect(),
                ));
                Ok(Output {
                    status: std::process::ExitStatus::from_raw(1),
                    stdout: vec![],
                    stderr: vec![],
                })
            }
        };
        let git = super::GitRepository {
            exec: Box::new(mock_exec),
            config: Config { test: "foo".to_string(), no_verify: None },
        };
        let result = git.test();
        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("sh".to_string(), vec!["-c".to_string(), "foo".to_string()]));
        assert_eq!(result, false);
    }
}

use std::fmt;
use crate::commit::{Commit, CommitConfig};
use crate::config::Config;
use crate::revert::{Revert};
use crate::test::{Test};

pub fn tcr(
    config: fn() -> Option<Config>,
    test: Test,
    commit: Commit,
    revert: Revert,
    exec: impl Fn(&str, &[&str])) -> Result<(), ConfigurationNotFound>
{
    let config = config().ok_or(ConfigurationNotFound)?;

    let test = test(config.clone().test);
    let commit = commit(CommitConfig { no_verify: config.clone().no_verify });
    let revert = revert();

    let cmd = format!("git add . &&  [ -n \"$(git status --porcelain)\" ] && ({test} && {commit} || {revert})");

    Ok(exec("sh", &["-c", &cmd]))
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
    use crate::config::Config;
    use crate::tcr::{tcr};

    fn setup_mock() -> (Rc<RefCell<Vec<(String, Vec<String>)>>>, impl Fn(&str, &[&str])) {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let mock_exec = {
            let captured_calls = Rc::clone(&captured_calls); // Usa Rc::clone per evitare il move
            move |program: &str, args: &[&str]| {
                captured_calls.borrow_mut().push((
                    program.to_string(),
                    args.iter().map(|&s| s.to_string()).collect(),
                ));
            }
        };
        (captured_calls, mock_exec)
    }

    #[test]
    fn cmd() {
        let (captured_calls, mock_exec) = setup_mock();

        let res = tcr(
            || Some(Config {
                test: String::from("test"),
                no_verify: Some(true),
            }),
            |test| format!("{test}").to_string(),
            |config| format!("commit {config:?}").to_string(),
            || "revert".to_string(),
            mock_exec,
        );

        assert!(res.is_ok());

        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0],
            (
                "sh".to_string(),
                vec![
                    "-c".to_string(),
                    "git add . &&  [ -n \"$(git status --porcelain)\" ] && (test && commit CommitConfig { no_verify: Some(true) } || revert)".to_string()
                ]
            )
        );
    }

    #[test]
    fn no_conf() {
        let (captured_calls, mock_exec) = setup_mock();

        let res = tcr(
            || None,
            |test| format!("{test}").to_string(),
            |config| format!("commit {config:?}").to_string(),
            || "revert".to_string(),
            mock_exec,
        );

        assert!(res.is_err());

        let calls: Ref<Vec<(String, Vec<String>)>> = captured_calls.borrow();
        assert_eq!(calls.len(), 0);
    }
}


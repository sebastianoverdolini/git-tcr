use std::fmt;
use serde::{Deserialize, Serialize};
use crate::commit::{Commit, CommitConfig};
use crate::revert::Revert;
use crate::test::Test;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TcrConfig
{
    pub test: String,
    pub commit: CommitConfig
}

pub fn tcr_command(
    config: fn() -> Option<TcrConfig>,
    test: Test,
    commit: Commit,
    revert: Revert,
) -> Result<TcrCommand, ConfigurationNotFound>
{
    let config = config().ok_or(ConfigurationNotFound)?;

    let test = test(config.clone().test);
    let commit = commit(config.clone().commit);
    let revert = revert();

    Ok(format!("git add . &&  [ -n \"$(git status --porcelain)\" ] && ({test} && {commit} || {revert})"))
}

pub type TcrCommand = String;

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
    use crate::commit::CommitConfig;
    use crate::tcr::TcrConfig;
    use crate::tcr::{tcr_command, ConfigurationNotFound};

    #[test]
    fn cmd()
    {
        // TODO Test better with mocks?
        let cmd = tcr_command(
            || Some(TcrConfig
            {
                test: String::from("test"),
                commit: CommitConfig {
                    no_verify: Some(true)
                }
            }),
            |test| format!("{test}").to_string(),
            |config| format!("commit {config:?}").to_string(),
            || "revert".to_string());

        assert!(cmd.is_ok());
        assert_eq!(
            cmd.unwrap(),
            "git add . &&  [ -n \"$(git status --porcelain)\" ] && (test && commit CommitConfig { no_verify: Some(true) } || revert)");
    }

    #[test]
    fn no_conf()
    {
        // TODO Throw exception on usages?
        let cmd = tcr_command(
            || None,
            |_test| "".to_string(),
            |_config| "".to_string(),
            || "".to_string());

        assert!(cmd.is_err());
        assert_eq!(cmd.unwrap_err(), ConfigurationNotFound);
    }
}

pub fn tcr(
    mut exec: impl FnMut(String, Vec<String>) -> (),
    config: fn() -> Option<TcrConfig>,
    test: Test,
    commit: Commit,
    revert: Revert,
) -> Result<(), ConfigurationNotFound>
{
    let r = tcr_command(
        config,
        test,
        commit,
        revert
    )?;
    Ok(exec("sh".to_string(), vec!["-c".to_string(), r]))
}

#[cfg(test)]
mod tcr_test
{
    use crate::commit::CommitConfig;
    use crate::tcr::{tcr, ConfigurationNotFound, TcrConfig};

    #[test]
    fn cmd()
    {
        let mut cmd = "".to_string();
        let res = tcr(
            |program, args| cmd = {
                let a = args.join(" ");
                format!("{program} {a}")
            },
            || Some(TcrConfig
            {
                test: String::from("test"),
                commit: CommitConfig {
                    no_verify: Some(true)
                }
            }),
            |test| format!("{test}").to_string(),
            |config| format!("commit {config:?}").to_string(),
            || "revert".to_string());

        assert!(res.is_ok());
        assert_eq!(
            cmd,
            "sh -c git add . &&  [ -n \"$(git status --porcelain)\" ] && (test && commit CommitConfig { no_verify: Some(true) } || revert)");
    }

    #[test]
    fn config_not_found()
    {
        let mut cmd = "".to_string();

        let result = tcr(
            |_, _| cmd = "called".to_string(),
            || None,
            |_test| "".to_string(),
            |_config| "".to_string(),
            || "".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ConfigurationNotFound);
        assert_eq!(cmd, "");
    }
}

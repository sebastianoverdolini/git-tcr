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

pub fn tcr(
    mut exec: impl FnMut(&str, Vec<&str>) -> (),
    config: fn() -> Option<TcrConfig>,
    test: Test,
    commit: Commit,
    revert: Revert,
) -> Result<(), ConfigurationNotFound> {
    let config = config().ok_or(ConfigurationNotFound)?;

    let r = format!(
        "git add . &&  [ -n \"$(git status --porcelain)\" ] && ({} && {} || {})",
        test(config.test),
        commit(config.commit),
        revert()
    );

    exec("sh", vec!["-c", r.as_str()]);
    Ok(())
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

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationNotFound;

impl fmt::Display for ConfigurationNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration not found.")
    }
}

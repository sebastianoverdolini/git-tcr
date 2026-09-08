use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TestConfig {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TestSpec {
    Single(TestConfig),
    Multiple(Vec<TestConfig>),
}

impl TestSpec {
    pub fn as_vec(&self) -> Vec<TestConfig> {
        match self {
            TestSpec::Single(test) => vec![test.clone()],
            TestSpec::Multiple(tests) => tests.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub test: TestSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_verify: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    /// No `tcr.yaml` in the given directory.
    NotFound,
    /// `tcr.yaml` is present but couldn't be parsed (e.g. it uses a shape
    /// this build of git-tcr doesn't understand).
    Invalid(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotFound => write!(f, "\
            'tcr.yaml' not found in the current directory. \
            Please create the configuration file to proceed. \
            Run 'git tcr init' to create one interactively."),
            ConfigError::Invalid(reason) => write!(f, "\
            'tcr.yaml' could not be parsed: {reason}. \
            This may mean the config uses a feature this version of git-tcr doesn't support yet \
            \u{2014} check that git-tcr is up to date."),
        }
    }
}

pub fn yaml_config(location: impl Into<PathBuf>) -> Result<Config, ConfigError> {
    let config_path = location.into().join("tcr.yaml");
    let content = std::fs::read_to_string(&config_path).map_err(|_| ConfigError::NotFound)?;
    serde_yaml::from_str(&content).map_err(|err| ConfigError::Invalid(err.to_string()))
}

#[cfg(test)]
mod yaml_config_tests {
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::path::Path;
    use crate::config;
    use crate::config::{Config, ConfigError, TestConfig, TestSpec};

    #[test]
    fn it_returns_the_content_of_the_config_if_the_file_is_present_in_the_current_location() {
        let test_dir = "test-env-content-check";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        let yaml_string = r#"
        test:
          program: "npm"
          args:
            - "test"
        no_verify: true
        "#;
        write(&config_path, yaml_string).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert_eq!(result, Ok(Config {
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: Some(true)
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_returns_a_list_of_test_commands_when_the_config_declares_multiple() {
        let test_dir = "test-env-multiple-tests";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        let yaml_string = r#"
        test:
          - program: "tsc"
            args:
              - "--noEmit"
          - program: "npm"
            args:
              - "run"
              - "test"
        "#;
        write(&config_path, yaml_string).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert_eq!(result, Ok(Config {
            test: TestSpec::Multiple(vec![
                TestConfig { program: String::from("tsc"), args: vec![String::from("--noEmit")] },
                TestConfig { program: String::from("npm"), args: vec![String::from("run"), String::from("test")] },
            ]),
            no_verify: None
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn no_verify_option_is_optional() {
        let test_dir = "test-env-no-verify";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        write(&config_path, r#"
        test:
          program: "npm"
          args:
            - "test"
        "#).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert_eq!(result, Ok(Config {
            test: TestSpec::Single(TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            }),
            no_verify: None
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_returns_not_found_if_the_config_file_is_not_present_in_the_current_location() {
        let test_dir = "test-env-none-check";

        let _ = remove_dir_all(test_dir);

        assert_eq!(config::yaml_config(Path::new(test_dir)), Err(ConfigError::NotFound));
    }

    #[test]
    fn it_returns_invalid_when_the_file_is_present_but_has_an_unparseable_shape() {
        let test_dir = "test-env-invalid-shape";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        // e.g. a config written for a newer git-tcr, or simply malformed.
        write(&config_path, "test: 42\n").expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert!(matches!(result, Err(ConfigError::Invalid(_))), "expected Invalid, got {result:?}");
        // The failure must be distinguishable from a missing file.
        assert_ne!(result, Err(ConfigError::NotFound));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }
}

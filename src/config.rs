use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TestConfig {
    pub program: String,
    pub args: Vec<String>,
}

/// The highest `version` this build of git-tcr knows how to interpret.
/// Bump this only when a `tcr.yaml` shape change would otherwise be
/// misread (silently or not) by older builds.
///
/// History:
/// - 1: `test` was a single command or a list of them.
/// - 2: `test` is a single command only.
pub const MAX_SUPPORTED_VERSION: u32 = 2;

fn default_version() -> u32 {
    // Configs written before `version` existed are all shape-1: assume that
    // when the field is absent.
    1
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: u32,
    pub test: TestConfig,
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
    /// `tcr.yaml` declares a `version` newer than this build understands.
    UnsupportedVersion { found: u32, max_supported: u32 },
    /// `tcr.yaml` declares a `version` older than this build's, and its
    /// content doesn't fit the current shape: it most likely uses
    /// something a later version dropped.
    Outdated { found: u32, max_supported: u32, reason: String },
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
            ConfigError::UnsupportedVersion { found, max_supported } => write!(f, "\
            'tcr.yaml' declares version {found}, but this build of git-tcr ({}) only understands up to \
            version {max_supported}. Please upgrade git-tcr.", env!("CARGO_PKG_VERSION")),
            ConfigError::Outdated { found, max_supported, reason } => write!(f, "\
            'tcr.yaml' declares version {found}, but this build of git-tcr reads it as version {max_supported} \
            and could not parse it: {reason}. The file probably uses a shape that a later version dropped \
            (since version 2, 'test' is a single command; run multi-step checks from a script instead). \
            Update the file and set 'version: {max_supported}'."),
        }
    }
}

/// Just the `version` field, read ahead of the full config so that a
/// parse failure can be explained in terms of the version the file
/// declares rather than as a generic error.
#[derive(Deserialize)]
struct DeclaredVersion {
    #[serde(default = "default_version")]
    version: u32,
}

pub fn yaml_config(location: impl Into<PathBuf>) -> Result<Config, ConfigError> {
    let config_path = location.into().join("tcr.yaml");
    let content = std::fs::read_to_string(&config_path).map_err(|_| ConfigError::NotFound)?;
    let declared: DeclaredVersion = serde_yaml::from_str(&content).map_err(|err| ConfigError::Invalid(err.to_string()))?;
    if declared.version > MAX_SUPPORTED_VERSION {
        return Err(ConfigError::UnsupportedVersion { found: declared.version, max_supported: MAX_SUPPORTED_VERSION });
    }
    serde_yaml::from_str(&content).map_err(|err| {
        let reason = err.to_string();
        if declared.version < MAX_SUPPORTED_VERSION {
            ConfigError::Outdated { found: declared.version, max_supported: MAX_SUPPORTED_VERSION, reason }
        } else {
            ConfigError::Invalid(reason)
        }
    })
}

#[cfg(test)]
mod yaml_config_tests {
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::path::Path;
    use crate::config;
    use crate::config::{Config, ConfigError, TestConfig};

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
            version: 1,
            test: TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            },
            no_verify: Some(true)
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
            version: 1,
            test: TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            },
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
    fn it_accepts_an_explicit_version_at_or_below_max_supported() {
        let test_dir = "test-env-explicit-version";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        write(&config_path, r#"
        version: 1
        test:
          program: "npm"
          args:
            - "test"
        "#).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert_eq!(result, Ok(Config {
            version: 1,
            test: TestConfig {
                program: String::from("npm"),
                args: vec![String::from("test")],
            },
            no_verify: None
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_rejects_a_version_newer_than_this_build_supports() {
        let test_dir = "test-env-unsupported-version";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        write(&config_path, r#"
        version: 99
        test:
          program: "npm"
          args:
            - "test"
        "#).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert_eq!(result, Err(ConfigError::UnsupportedVersion {
            found: 99,
            max_supported: config::MAX_SUPPORTED_VERSION,
        }));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn unsupported_version_message_names_both_the_found_and_the_installed_git_tcr_version() {
        let message = ConfigError::UnsupportedVersion { found: 99, max_supported: 2 }.to_string();

        assert!(message.contains("99"), "should name the version found in the file: {message}");
        assert!(message.contains("2"), "should name the max version supported: {message}");
        assert!(message.contains(env!("CARGO_PKG_VERSION")), "should name the installed git-tcr version: {message}");
    }

    #[test]
    fn it_returns_invalid_when_the_file_is_present_but_has_an_unparseable_shape() {
        let test_dir = "test-env-invalid-shape";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        // Declares the current version, so this is simply malformed rather
        // than written for an older shape.
        write(&config_path, "version: 2\ntest: 42\n").expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert!(matches!(result, Err(ConfigError::Invalid(_))), "expected Invalid, got {result:?}");
        // The failure must be distinguishable from a missing file.
        assert_ne!(result, Err(ConfigError::NotFound));

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_explains_a_version_1_config_that_still_declares_a_list_of_test_commands() {
        let test_dir = "test-env-outdated-shape";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        // The version-1 list form, which version 2 dropped. No explicit
        // `version`, as most files written back then didn't have one.
        let yaml_string = r#"
        test:
          - program: "tsc"
            args: ["--noEmit"]
          - program: "npm"
            args: ["run", "test"]
        "#;
        write(&config_path, yaml_string).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        let err = result.expect_err("a list of test commands is no longer a valid config");
        assert!(matches!(err, ConfigError::Outdated { found: 1, max_supported: 2, .. }), "expected Outdated, got {err:?}");
        let message = err.to_string();
        assert!(message.contains("single command"), "should explain what changed: {message}");
        assert!(message.contains("version: 2"), "should say which version to declare: {message}");

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }
}

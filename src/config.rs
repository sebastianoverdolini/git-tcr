use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config
{
    pub test: String,
    #[serde(default)]
    pub no_verify: Option<bool>
}

pub fn yaml_config(location: impl Into<PathBuf>) -> Option<Config>
{
    let config_path = location.into().join("tcr.yaml");
    let content = std::fs::read_to_string(&config_path).ok()?;
    let config = serde_yaml::from_str(&content).ok()?;
    Some(config)
}

#[cfg(test)]
mod yaml_config_tests {
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::path::Path;
    use crate::config;
    use crate::config::Config;

    #[test]
    fn it_returns_the_content_of_the_config_if_the_file_is_present_in_the_current_location() {
        let test_dir = "test-env-content-check";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        let yaml_string = r#"
        test: npm test
        no_verify: true
        "#;
        write(&config_path, yaml_string).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), Config {
            test: String::from("npm test"),
            no_verify: Some(true)
        });

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn no_verify_option_is_optional() {
        let test_dir = "test-env-no-verify";
        let config_path = format!("{}/tcr.yaml", test_dir);

        let _ = remove_dir_all(test_dir);
        create_dir_all(test_dir).expect("Failed to create test directory");

        write(&config_path, r#"
        test: npm test
        before: []
        "#).expect("Failed to write test config");

        let result = config::yaml_config(Path::new(test_dir));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), Config {
            test: String::from("npm test"),
            no_verify: None
        });

        remove_dir_all(test_dir).expect("Failed to remove test directory");
    }

    #[test]
    fn it_returns_none_if_the_config_file_is_not_present_in_the_current_location() {
        let test_dir = "test-env-none-check";

        let _ = remove_dir_all(test_dir);

        assert!(config::yaml_config(Path::new(test_dir)).is_none());
    }
}


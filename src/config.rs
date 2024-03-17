use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config
{
    pub test: String,
    pub before: Vec<String>
}

pub fn yaml_config(location: impl Into<PathBuf>) -> Option<Config>
{
    let config_path = location.into().join("tcr.yaml");
    if !config_path.exists()
    {
        return None;
    }
    let content = std::fs::read_to_string(config_path).unwrap();
    let config = serde_yaml::from_str(&content).expect("");
    return Some(config)
}

#[cfg(test)]
mod yaml_config_tests
{
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::path::Path;
    use crate::config;
    use crate::config::Config;

    #[test]
    fn it_returns_the_content_of_the_config_if_the_file_is_present_in_the_current_location()
    {
        create_dir_all("test-env").expect("TODO: panic message");
        let c = Config
        {
            test: String::from("npm test"),
            before: vec![
                String::from("pnpm tc"),
                String::from("prettier --write .")
            ]
        };
        let yaml = serde_yaml::to_string(&c).unwrap();
        write("test-env/tcr.yaml", yaml).expect("TODO: panic message");

        let result = config::yaml_config(Path::new("./test-env"));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), c);

        remove_dir_all("test-env").expect("TODO: panic message");
    }

    #[test]
    fn it_returns_none_if_the_config_file_is_not_present_in_the_current_location()
    {
        assert!(config::yaml_config(String::from("./test-env")).is_none());
    }
}

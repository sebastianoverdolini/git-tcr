use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config
{
    pub test: String
}

pub fn config(location: String) -> Option<Config>
{
    let config_path = location.to_owned() + "/tcr.yaml";
    if !Path::new(&(config_path)).exists()
    {
        return None;
    }
    let content = std::fs::read_to_string(config_path).unwrap();
    let config = serde_yaml::from_str(&content).expect("");
    return Some(config)
}

#[cfg(test)]
mod file_config_tests
{
    use std::fs::{create_dir_all, remove_dir_all, write};
    use crate::config;
    use crate::config::Config;

    #[test]
    fn it_returns_the_content_of_the_config_if_the_file_is_present_in_the_current_location()
    {
        create_dir_all("test-env").expect("TODO: panic message");
        let c = Config
        {
            test: String::from("npm test")
        };
        let yaml = serde_yaml::to_string(&c).unwrap();
        write("test-env/tcr.yaml", yaml).expect("TODO: panic message");

        let result = config::config(String::from("./test-env"));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), c);

        remove_dir_all("test-env").expect("TODO: panic message");
    }

    #[test]
    fn it_returns_none_if_the_config_file_is_not_present_in_the_current_location()
    {
        assert!(config::config(String::from("./test-env")).is_none());
    }
}

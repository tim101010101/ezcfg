use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;

use crate::error::ReadConfigError::*;
use crate::link_transform::link_transform;
use crate::{adapter::adapter, error::ReadConfigError};

pub type Links = Vec<(String, String)>;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub rewrite: bool,
    pub links: Links,
}

impl Config {
    pub fn new(links: Links, rewrite: bool) -> Self {
        Config { links, rewrite }
    }
}

impl TryFrom<&PathBuf> for Config {
    type Error = ReadConfigError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let ext = {
            if let Some(ext_os_str) = path.extension() {
                ext_os_str.to_str().unwrap()
            } else {
                return Err(FailedToGetExtension(path.to_owned()));
            }
        };
        let content = {
            if let Ok(content) = read_to_string(path) {
                content
            } else {
                return Err(FailedToReadFile(path.to_owned()));
            }
        };
        let raw_config = adapter(path, ext, &content)?;

        let links = link_transform(raw_config.links);

        Ok(Config {
            links,
            ..raw_config
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use assert_fs::{prelude::*, TempDir};

    use super::*;

    fn test_with_temp_dir<F>(f: F)
    where
        F: FnOnce(&TempDir) -> (),
    {
        let temp_dir = TempDir::new().unwrap();
        f(&temp_dir);
        temp_dir.close().unwrap();
    }

    fn concat_pwd(path: &str) -> String {
        current_dir()
            .unwrap()
            .join(path)
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn it_should_get_config_from_a_path() {
        test_with_temp_dir(|temp_dir| {
            temp_dir
                .child("ezcfg.toml")
                .write_str(
                    r#"
                        links = [
                            ["a", "b"],
                            ["c", "d"],
                        ]
                    "#,
                )
                .unwrap();

            let config_path = temp_dir.child("ezcfg.toml").path().to_path_buf();
            let config = Config::try_from(&config_path).unwrap();

            assert_eq!(config.links.len(), 2);
            assert_eq!(config.links[0].0, concat_pwd("a"));
            assert_eq!(config.links[0].1, "b");
            assert_eq!(config.links[1].0, concat_pwd("c"));
            assert_eq!(config.links[1].1, "d");
        })
    }
}

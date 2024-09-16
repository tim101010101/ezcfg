use std::{env::current_dir, path::PathBuf, vec};

use ezcfg_cli::warn;
use serde::Deserialize;
use toml::from_str as toml_from_str;

use crate::{error::ReadConfigError, Config, Links};

#[derive(Debug, Default, Deserialize)]
struct ConfigStruct {
    /// Whether to rewrite the target file if it already exists
    rewrite: Option<bool>,

    /// Cross-platform shared configuration
    links: Option<Links>,

    /// System-specific configuration
    linux: Option<Links>,
    /// System-specific configuration
    macos: Option<Links>,
    /// System-specific configuration
    windows: Option<Links>,
}

pub fn adapter(path: &PathBuf, kind: &str, raw: &str) -> Result<Config, ReadConfigError> {
    let config_struct = match kind {
        "toml" => toml_from_str(raw).map_err(|e| panic!("{:?}", e)),

        _ => {
            return Err(ReadConfigError::UnsupportedConfigFile(
                current_dir()
                    .unwrap()
                    .join(path)
                    .to_str()
                    .unwrap()
                    .to_string(),
            ))
        }
    };
    let config_struct: ConfigStruct = config_struct.unwrap();

    let rewrite = config_struct.rewrite.unwrap_or(false);

    let relative_links = {
        let system_specified_links = {
            if cfg!(target_os = "linux") {
                config_struct.linux
            } else if cfg!(target_os = "macos") {
                config_struct.macos
            } else if cfg!(target_os = "windows") {
                config_struct.windows
            } else {
                None
            }
        };

        if let Some(links) = system_specified_links {
            links
        } else if let Some(links) = config_struct.links {
            links
        } else {
            let msg = format!("No links found in the config file: {:?}", path);
            warn!(msg);
            vec![]
        }
    };

    let current_dir = current_dir().unwrap();
    let links = relative_links
        .iter()
        .map(|(source, target)| {
            (
                current_dir.join(source).to_str().unwrap().to_string(),
                // TODO Fine, we need to use some placeholders like `~` to
                // TODO represent the home directory...
                // The target should be the absolute path
                target.to_string(),
            )
        })
        .collect();

    Ok(Config::new(links, rewrite))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn concat_pwd(path: &str) -> String {
        current_dir()
            .unwrap()
            .join(path)
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn test_ser() {
        let path = PathBuf::from("~/config");
        let path = path.canonicalize().unwrap();

        println!("{}", path.to_str().unwrap());
    }

    #[test]
    fn it_should_parse_toml_str_into_config() {
        let raw = r#"
            links = [
                ["a", "b"],
                ["c", "d"],
            ]
        "#;

        let config = adapter(&PathBuf::default(), "toml", raw).unwrap();

        assert_eq!(config.links.len(), 2);
        assert_eq!(config.links[0].0, concat_pwd("a"));
        assert_eq!(config.links[0].1, "b");
        assert_eq!(config.links[1].0, concat_pwd("c"));
        assert_eq!(config.links[1].1, "d");
    }

    #[test]
    fn it_should_return_error_with_unsupport_file() {
        let e = match adapter(&PathBuf::from_str("a/b.unknown").unwrap(), "unknown", "") {
            Ok(_) => panic!("Should return error"),
            Err(e) => e,
        };

        assert_eq!(
            e,
            ReadConfigError::UnsupportedConfigFile(concat_pwd("a/b.unknown"))
        );
    }

    #[test]
    fn it_should_not_panic_with_empty_config() {
        adapter(&PathBuf::default(), "toml", "").unwrap();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn it_should_use_system_specified_first_on_macos() {
        {
            let raw = r#"
                macos = [
                    ["m_a", "m_b"],
                    ["m_c", "m_d"],
                ]
                links = [
                    ["a", "b"],
                    ["c", "d"],
                ]
            "#;
            let config = adapter(&PathBuf::default(), "toml", raw).unwrap();

            assert_eq!(config.links.len(), 2);
            assert_eq!(config.links[0].0, concat_pwd("m_a"));
            assert_eq!(config.links[0].1, "m_b");
            assert_eq!(config.links[1].0, concat_pwd("m_c"));
            assert_eq!(config.links[1].1, "m_d");
        }

        {
            let raw = r#"
                linux = [
                    ["m_a", "m_b"],
                    ["m_c", "m_d"],
                ]
                links = [
                    ["a", "b"],
                    ["c", "d"],
                ]
            "#;
            let config = adapter(&PathBuf::default(), "toml", raw).unwrap();

            assert_eq!(config.links.len(), 2);
            assert_eq!(config.links[0].0, concat_pwd("a"));
            assert_eq!(config.links[0].1, "b");
            assert_eq!(config.links[1].0, concat_pwd("c"));
            assert_eq!(config.links[1].1, "d");
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn it_should_use_system_specified_first_on_linux() {
        {
            let raw = r#"
                linux = [
                    ["m_a", "m_b"],
                    ["m_c", "m_d"],
                ]
                links = [
                    ["a", "b"],
                    ["c", "d"],
                ]
            "#;
            let config = adapter(&PathBuf::default(), "toml", raw).unwrap();

            assert_eq!(config.links.len(), 2);
            assert_eq!(config.links[0].0, concat_pwd("m_a"));
            assert_eq!(config.links[0].1, "m_b");
            assert_eq!(config.links[1].0, concat_pwd("m_c"));
            assert_eq!(config.links[1].1, "m_d");
        }

        {
            let raw = r#"
                macos = [
                    ["m_a", "m_b"],
                    ["m_c", "m_d"],
                ]
                links = [
                    ["a", "b"],
                    ["c", "d"],
                ]
            "#;
            let config = adapter(&PathBuf::default(), "toml", raw).unwrap();

            assert_eq!(config.links.len(), 2);
            assert_eq!(config.links[0].0, concat_pwd("a"));
            assert_eq!(config.links[0].1, "b");
            assert_eq!(config.links[1].0, concat_pwd("c"));
            assert_eq!(config.links[1].1, "d");
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn it_should_use_system_specified_first_on_windows() {
        {
            let raw = r#"
                windows = [
                    ["m_a", "m_b"],
                    ["m_c", "m_d"],
                ]
                links = [
                    ["a", "b"],
                    ["c", "d"],
                ]
            "#;
            let config = adapter(&PathBuf::default(), "toml", raw).unwrap();

            assert_eq!(config.links.len(), 2);
            assert_eq!(config.links[0].0, concat_pwd("m_a"));
            assert_eq!(config.links[0].1, "m_b");
            assert_eq!(config.links[1].0, concat_pwd("m_c"));
            assert_eq!(config.links[1].1, "m_d");
        }

        {
            let raw = r#"
                linux = [
                    ["m_a", "m_b"],
                    ["m_c", "m_d"],
                ]
                links = [
                    ["a", "b"],
                    ["c", "d"],
                ]
            "#;
            let config = adapter(&PathBuf::default(), "toml", raw).unwrap();

            assert_eq!(config.links.len(), 2);
            assert_eq!(config.links[0].0, concat_pwd("a"));
            assert_eq!(config.links[0].1, "b");
            assert_eq!(config.links[1].0, concat_pwd("c"));
            assert_eq!(config.links[1].1, "d");
        }
    }
}

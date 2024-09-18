use std::{env::current_dir, path::PathBuf};

use ezcfg_cli::warn;
use ezcfg_config::Config;

static CONFIG_FILES: [&str; 2] = [".ezcfg.toml", ".ezcfg/ezcfg.toml"];

pub fn read_config() -> Option<Config> {
    let config_file_list = CONFIG_FILES
        .iter()
        .map(|path| concat_pwd(path))
        .collect::<Vec<_>>();

    for path in config_file_list.iter() {
        if !path.exists() {
            continue;
        }

        match Config::try_from(path) {
            Ok(config) => return Some(config),
            Err(e) => {
                let path = format!("Failed to read config file: {:?}", path);
                let raw_err = format!("{:?}", e);
                warn!(path, raw_err);

                return None;
            }
        };
    }

    warn!(
        "No config file found in current directory",
        "Try to create a config file named '.ezcfg.toml' in current directory"
    );

    None
}

fn concat_pwd(path: &str) -> PathBuf {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        path
    } else {
        current_dir().unwrap().join(path)
    }
}

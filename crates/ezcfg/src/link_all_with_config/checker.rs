use std::{
    fs::{remove_dir_all, remove_file},
    path::Path,
};

use ezcfg_cli::warn;
use ezcfg_config::Config;

type CheckResult = Result<(), ()>;

pub fn check_path(config: &Config, source: &str, target: &str) -> CheckResult {
    let source_path = Path::new(source);
    let target_path = Path::new(target);

    check_source_exist(source_path)?;
    check_rewrite_target(target_path, config.rewrite)?;

    Ok(())
}

#[inline]
fn check_source_exist(source: &Path) -> CheckResult {
    if !source.exists() {
        let msg = format!("Path dose not exist: {:?}", source);
        warn!(msg);

        return Err(());
    }
    Ok(())
}

#[inline]
fn check_rewrite_target(target: &Path, rewrite: bool) -> CheckResult {
    if !target.exists() {
        return Ok(());
    }

    match rewrite {
        false => Err(()),

        true if target.is_file() || target.is_symlink() => {
            remove_file(target).unwrap();
            Ok(())
        }
        true if target.is_dir() => {
            remove_dir_all(target).unwrap();
            Ok(())
        }
        _ => Err(()),
    }
}

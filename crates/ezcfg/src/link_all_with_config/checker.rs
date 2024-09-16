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

// TODO maybe I should remove the existed target in the reading stage of config file
// TODO instead of here
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

#[cfg(test)]
mod tests {
    use std::os::unix::fs::symlink;

    use assert_fs::prelude::{FileTouch, PathChild, PathCreateDir};
    use ezcfg_test::test_in_temp_dir;

    use super::*;

    test_in_temp_dir!(it_should_pass_when_source_exist, |temp_dir| {
        let config = Config::default();

        let source = temp_dir.child("source");
        source.touch().unwrap();

        let source_path = source.path().to_str().unwrap();
        assert!(check_path(&config, source_path, "").is_ok());
    });

    test_in_temp_dir!(it_should_not_pass_when_source_not_exist, |_| {
        let config = Config::default();
        assert!(check_path(&config, "not-exist", "").is_err());
    });

    test_in_temp_dir!(it_should_pass_when_target_not_exist, |temp_dir| {
        let config = Config::default();

        let source = temp_dir.child("source");
        source.touch().unwrap();

        let source_path = source.path().to_str().unwrap();
        assert!(check_path(&config, source_path, "not-exist").is_ok());
    });

    test_in_temp_dir!(
        it_should_pass_when_enable_rewrite_and_target_not_exist,
        |temp_dir| {
            let mut config = Config::default();
            config.rewrite = true;

            let source = temp_dir.child("source");
            source.touch().unwrap();

            let source_path = source.path().to_str().unwrap();
            assert!(check_path(&config, source_path, "not-exist").is_ok());
        }
    );

    test_in_temp_dir!(
        it_should_pass_and_delete_target_when_enable_rewrite_and_target_file_exist,
        |temp_dir| {
            let mut config = Config::default();
            config.rewrite = true;

            let source = temp_dir.child("source");
            source.touch().unwrap();

            let target = temp_dir.child("target");
            target.touch().unwrap();

            assert!(target.exists());
            assert!(target.is_file());

            let source_path = source.path().to_str().unwrap();
            let target_path = target.path().to_str().unwrap();

            assert!(check_path(&config, source_path, target_path).is_ok());

            assert!(!target.exists());
        }
    );

    test_in_temp_dir!(
        it_should_pass_and_delete_target_when_enable_rewrite_and_target_dir_exist,
        |temp_dir| {
            let mut config = Config::default();
            config.rewrite = true;

            let source = temp_dir.child("source");
            source.touch().unwrap();

            let target = temp_dir.child("target");
            target.create_dir_all().unwrap();

            assert!(target.exists());
            assert!(target.is_dir());

            let source_path = source.path().to_str().unwrap();
            let target_path = target.path().to_str().unwrap();

            assert!(check_path(&config, source_path, target_path).is_ok());

            assert!(!target.exists());
        }
    );

    test_in_temp_dir!(
        it_should_pass_and_delete_target_when_enable_rewrite_and_target_link_exist,
        |temp_dir| {
            let mut config = Config::default();
            config.rewrite = true;

            let source = temp_dir.child("source");
            source.touch().unwrap();

            let link_source = temp_dir.child("link_source");
            link_source.create_dir_all().unwrap();

            let target = temp_dir.child("target");
            symlink(link_source.path(), target.path()).unwrap();

            assert!(target.exists());
            assert!(target.is_symlink());

            let source_path = source.path().to_str().unwrap();
            let target_path = target.path().to_str().unwrap();

            assert!(check_path(&config, source_path, target_path).is_ok());

            assert!(!target.exists());
        }
    );
}

use assert_fs::TempDir;
use std::{
    fs::{create_dir_all, read, symlink_metadata, write},
    path::PathBuf,
};

use ezcfg_config::Config;
use ezcfg_linker::link_all;

fn ensure_exist(path: PathBuf) {
    if path.extension().is_some() {
        let parent = path.parent().unwrap();
        if !parent.exists() {
            create_dir_all(parent).unwrap();
        }
        if !path.exists() {
            write(&path, path.to_str().unwrap()).unwrap();
        }
    } else if !path.exists() {
        create_dir_all(path).unwrap();
    }
}

pub fn run_in_temp_dir_with_config<F>(config: Config, f: F)
where
    F: FnOnce(Vec<(String, String)>),
{
    let temp_source_dir = TempDir::new().unwrap();

    let base_source_path = &temp_source_dir;
    let base_target_path = base_source_path.join("target");

    create_dir_all(&base_target_path).unwrap();

    let Config { links, .. } = config;

    let links = links
        .iter()
        .map(|(target, generated)| {
            (
                base_source_path.join(target).to_str().unwrap().to_string(),
                base_target_path
                    .join(generated)
                    .to_str()
                    .unwrap()
                    .to_string(),
            )
        })
        .collect::<Vec<(String, String)>>();

    // Create source files
    links.iter().for_each(|(source, _)| {
        ensure_exist(PathBuf::from(source));
    });

    f(links);

    temp_source_dir.close().unwrap();
}

fn run_and_test_all_links(links: &Vec<(String, String)>) {
    link_all(links);

    links.iter().for_each(|(source, target)| {
        let metadata = symlink_metadata(target).unwrap();
        assert!(metadata.file_type().is_symlink());
        if PathBuf::from(source).extension().is_some() {
            assert_eq!(read(target).unwrap(), read(source).unwrap());
        }
    });
}

pub fn test_in_temp_dir_with_links<S>(links: Vec<(S, S)>)
where
    S: AsRef<str>,
{
    let config = Config::new(
        links
            .iter()
            .map(|(source, target)| (source.as_ref().to_string(), target.as_ref().to_string()))
            .collect(),
        false,
    );

    run_in_temp_dir_with_config(config, |links| {
        run_and_test_all_links(&links);
    })
}

pub fn test_in_temp_dir_with_config(config: Config) {
    run_in_temp_dir_with_config(config, |links| {
        run_and_test_all_links(&links);
    })
}

pub fn test_in_temp_dir<F>(f: F)
where
    F: FnOnce(&mut TempDir),
{
    let mut temp_source_dir = TempDir::new().unwrap();
    f(&mut temp_source_dir);
    temp_source_dir.close().unwrap()
}

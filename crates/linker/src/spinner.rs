use std::{io, path::PathBuf, sync::OnceLock, time::Duration};

use ezcfg_cli::warn;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

static MULTI_PROGRESS: OnceLock<MultiProgress> = OnceLock::<MultiProgress>::new();
fn mpb() -> &'static MultiProgress {
    #[allow(clippy::redundant_closure)]
    MULTI_PROGRESS.get_or_init(|| MultiProgress::new())
}

pub fn pb_setup() -> ProgressBar {
    mpb().add(ProgressBar::new_spinner())
}

pub fn pb_waiting(pb: &ProgressBar, _source: &str, _target: &str, idx: usize, len: usize) {
    let spinner_style = ProgressStyle::with_template("{prefix:7.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

    pb.set_style(spinner_style.clone());
    pb.set_prefix(format!("[{}/{}]", idx + 1, len));
    pb.set_message("Waiting".to_string());
}

pub fn pb_spinning(pb: &ProgressBar, source: &str, target: &str, _idx: usize, _len: usize) {
    pb.set_message(format!("Linking {} -> {}", source, target));
    pb.enable_steady_tick(Duration::from_millis(50));
}

pub fn pb_task_success(pb: &ProgressBar, source: &str, target: &str, _idx: usize, _len: usize) {
    let style = ProgressStyle::with_template("{prefix:.bold.green} {wide_msg}").unwrap();
    pb.set_style(style);
    pb.set_prefix("✔");
    pb.finish_with_message(format!(
        "{source} -> {target}",
        source = shorten_path(source),
        target = target
    ));
}

pub fn pb_task_fail(
    pb: &ProgressBar,
    source: &str,
    target: &str,
    _idx: usize,
    _len: usize,
    err: &io::Error,
) {
    let style = ProgressStyle::with_template("{prefix:.bold.red} {wide_msg}").unwrap();
    pb.set_style(style);
    pb.set_prefix("✘");
    pb.finish_with_message(format!(
        "Panic when linking {source} -> {target}\n{raw_err:?}",
        source = shorten_path(source),
        target = target,
        raw_err = err
    ));
}

fn shorten_path(path: &str) -> String {
    let path = PathBuf::from(path);
    if path.is_dir() {
        return format!("{}/", path.file_name().unwrap().to_str().unwrap());
    } else if path.is_file() {
        return path.file_name().unwrap().to_str().unwrap().to_string();
    }

    let msg = format!("Failed to shorten path: {:?}", path);
    warn!(msg);

    path.to_str().unwrap().to_string()
}

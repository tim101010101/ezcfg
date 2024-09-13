use std::env::consts::OS;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::os::unix::fs::symlink as unix_symlink;

use crate::spinner::{pb_setup, pb_spinning, pb_task_fail, pb_task_success, pb_waiting};
use crate::ThreadPool;

/// Create all soft links according to a [`Links`]
///
/// Additionally, it allows passing in a series of closure functions
// pub fn link_all_with_hooks<FTaskSend, ResBeforeSend, FOnSetup, ResOnSetup, FOnSuccess, FOnFail>(
//     links: &Vec<(String, String)>,
//     before_task_send: FTaskSend,
//     on_task_setup: FOnSetup,
//     on_task_success: FOnSuccess,
//     on_task_fail: FOnFail,
// ) where
//     ResBeforeSend: Send + 'static,
//     FTaskSend: Fn(&str, &str, usize) -> ResBeforeSend,
//     FOnSetup: Fn(&str, &str, usize, ResBeforeSend) -> ResOnSetup + Copy + Send + 'static,
//     FOnSuccess: Fn(&str, &str, usize, ResOnSetup) + Copy + Send + 'static,
//     FOnFail: Fn(&str, &str, usize, &IoError, ResOnSetup) + Copy + Send + 'static,
// {
//     let pool = ThreadPool::global();

//     links
//         .iter()
//         .enumerate()
//         .filter(|(_, (source, target))| check_path(source, target).is_some())
//         .for_each(|(idx, (source, target))| {
//             let source = source.to_string();
//             let target = target.to_string();
//             let res_on_task_setup = before_task_send(&source, &target, idx);

//             pool.execute(move || {
//                 let res_before_task = on_task_setup(&source, &target, idx, res_on_task_setup);
//                 match soft_link(&source, &target) {
//                     Ok(_) => on_task_success(&source, &target, idx, res_before_task),
//                     Err(e) => on_task_fail(&source, &target, idx, &e, res_before_task),
//                 }
//             });
//         });

//     pool.join();
// }
pub fn link_all_with_filter<F>(links: &Vec<(String, String)>, filter: F)
where
    F: Fn(&str, &str, usize, usize) -> bool,
{
    let pool = ThreadPool::global();

    let len = links.len();
    links
        .iter()
        .enumerate()
        .filter(|(idx, (source, target))| filter(source, target, *idx, len))
        .for_each(|(idx, (source, target))| {
            let source = source.to_string();
            let target = target.to_string();

            let pb = pb_setup();
            pb_waiting(&pb, &source, &target, idx, len);

            pool.execute(move || {
                pb_spinning(&pb, &source, &target, idx, len);
                match soft_link(&source, &target) {
                    Ok(_) => pb_task_success(&pb, &source, &target, idx, len),
                    Err(e) => pb_task_fail(&pb, &source, &target, idx, len, &e),
                }
            });
        });

    pool.join();
}

pub fn link_all(links: &Vec<(String, String)>) {
    link_all_with_filter(links, |_source, _target, _idx, _len| true)
}

fn soft_link(source: &str, target: &str) -> Result<(), IoError> {
    if cfg!(target_family = "unix") {
        unix_symlink(source, target)?;
        return Ok(());
    } else if cfg!(target_family = "windows") {
        return Err(IoError::new(
            IoErrorKind::Other,
            format!("Unsupported OS: {}", OS),
        ));
    }

    Err(IoError::new(
        IoErrorKind::Other,
        format!("Unsupported OS: {}", OS),
    ))
}

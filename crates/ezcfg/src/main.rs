mod link_all_with_config;
mod read_config;

use ezcfg_cli::command::{cli, version};
use link_all_with_config::link_all_with_config;
use read_config::read_config;

fn link_all() {
    if let Some(config) = read_config() {
        link_all_with_config(&config);
    }
}

fn main() {
    let matches = cli().get_matches();

    if matches.get_flag("version") {
        version(option_env!("CARGO_PKG_VERSION").unwrap_or("N/A"));
    } else {
        link_all();
    }
}

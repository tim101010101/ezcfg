mod link_all_with_config;
mod read_config;

use link_all_with_config::link_all_with_config;
use read_config::read_config;

fn main() {
    if let Some(config) = read_config() {
        link_all_with_config(&config);
    }
}

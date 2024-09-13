mod checker;

use checker::check_path;
use ezcfg_config::Config;
use ezcfg_linker::link_all_with_filter;

pub fn link_all_with_config(config: &Config) {
    link_all_with_filter(&config.links, |source, target, _idx, _len| {
        check_path(config, source, target).is_ok()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use ezcfg_test::*;

    fn test_in_temp_dir_with_config(config: Config) {
        run_in_temp_dir_with_config(config, |links| {
            let config = Config::new(links.clone(), false);
            link_all_with_config(&config)
        })
    }

    #[test]
    fn smoke() {
        test_in_temp_dir_with_config(Config {
            links: vec![
                ("a.txt".to_string(), "a.txt".to_string()),
                ("b.txt".to_string(), "b.txt".to_string()),
                ("c.txt".to_string(), "c.txt".to_string()),
                ("d.txt".to_string(), "d.txt".to_string()),
                ("anywhere/a.txt".to_string(), "e.txt".to_string()),
                ("anywhere/b.txt".to_string(), "f.txt".to_string()),
                ("anywhere/c.txt".to_string(), "g.txt".to_string()),
                ("anywhere/d.txt".to_string(), "h.txt".to_string()),
                ("anywhere/e.txt".to_string(), "i.txt".to_string()),
                ("anywhere/f.txt".to_string(), "j.txt".to_string()),
            ],
            rewrite: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use ezcfg_config::Config;

    #[test]
    fn test_creation() {
        let links = vec![
            ("a.txt".to_string(), "b.txt".to_string()),
            ("c.txt".to_string(), "d.txt".to_string()),
        ];
        let _config = Config::new(links, false);
    }

    test_with_config!(
        test_should_create_soft_link_to_file,
        vec![
            ("a.txt", "a.txt"),
            ("b.txt", "c.txt"),
            ("anywhere/b.txt", "d.txt"),
        ]
    );

    test_with_config!(
        test_should_create_soft_link_to_dir,
        vec![("a", "a"), ("b", "c"), ("anywhere/b", "d")]
    );

    test_with_config!(
        test_plenty_of_links,
        (0..=10000)
            .into_iter()
            .map(|i| (format!("{}.txt", i), format!("{}", (i + 1))))
            .collect::<Vec<(String, String)>>()
    );
}

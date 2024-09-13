#[macro_export]
macro_rules! test_with_config {
    ($test_name:ident, $config:expr) => {
        #[test]
        fn $test_name() {
            $crate::utils::test_in_temp_dir_with_links($config)
        }
    };
}

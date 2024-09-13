use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadConfigError {
    FailedToGetExtension(PathBuf),
    FailedToReadFile(PathBuf),

    UnsupportedConfigFile(String),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config file already exists: {0}")]
    ConfigExists(std::path::PathBuf),

    #[error("Config file not found: {0}")]
    ConfigNotFound(std::path::PathBuf),

    #[error(transparent)]
    ConfigV1(#[from] crate::config::v1::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
}

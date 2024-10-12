pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config error, cause: {cause}")]
    Config { cause: String },

    #[error("DB error, cause: {cause}")]
    DB { cause: String },
}

impl From<serde_yml::Error> for Error {
    fn from(value: serde_yml::Error) -> Self {
        Error::Config {
            cause: value.to_string(),
        }
    }
}

impl From<sqlite::Error> for Error {
    fn from(value: sqlite::Error) -> Self {
        Error::DB {
            cause: value.to_string(),
        }
    }
}

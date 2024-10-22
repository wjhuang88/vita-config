pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config error, cause: {cause}")]
    Config { cause: String },

    #[error("DB error, cause: {cause}")]
    DB { cause: String },

    #[error("IO error, cause: {cause}")]
    IO { cause: std::io::Error },

    #[error("IO error, cause: {cause}")]
    Convert { cause: String },
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

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO { cause: value }
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(value: std::convert::Infallible) -> Self {
        Error::Convert {
            cause: value.to_string(),
        }
    }
}

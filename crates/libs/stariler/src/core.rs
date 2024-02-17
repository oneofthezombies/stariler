pub static TS_CONFIG_FILE_NAME: &str = "tsconfig.json";

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    ConflictArgs { reason: String },
    NotFound { path: String },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait TryFromAsync<T>: Sized {
    type Error;

    fn try_from_async(t: T) -> impl std::future::Future<Output = std::result::Result<Self, Error>>;
}

pub static TS_CONFIG_FILE_NAME: &str = "tsconfig.json";

#[derive(Debug)]
pub enum Error {
    StdIo(std::io::Error),
    StdStrUtf8(std::str::Utf8Error),
    GlobPattern(glob::PatternError),
    GlobGlob(glob::GlobError),
    SerdeJson(serde_json::Error),
    TokioTaskJoin(tokio::task::JoinError),
    TsConfigNotFound { path: std::path::PathBuf },
    FilesArgEmpty,
    FileArgNotFound { path: std::path::PathBuf },
    FilesArgAndProjectArgConflict { files: Vec<String>, project: String },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::StdIo(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::StdStrUtf8(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Self {
        Self::GlobPattern(err)
    }
}

impl From<glob::GlobError> for Error {
    fn from(err: glob::GlobError) -> Self {
        Self::GlobGlob(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::TokioTaskJoin(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

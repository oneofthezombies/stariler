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

#[derive(Debug)]
pub enum InputKind {
    Files(Vec<String>),
    Project(String),
}

#[derive(Debug)]
pub struct Input {
    pub kind: InputKind,
}

impl Input {
    pub fn parse(&self) -> crate::Result<Config> {
        match &self.kind {
            InputKind::Files(files) => {
                let mut paths = vec![];
                for file in files {
                    let path = std::path::PathBuf::from(file);
                    if !path.try_exists()? {
                        return Err(crate::Error::NotFound {
                            path: file.to_string(),
                        });
                    }
                    paths.push(path);
                }
                Ok(Config { paths })
            }
            InputKind::Project(project) => {
                let path = std::path::PathBuf::from(project);
                if !path.try_exists()? {
                    return Err(crate::Error::NotFound {
                        path: project.to_string(),
                    });
                }
                if path.is_dir() {
                    let ts_config = crate::tsconfig::read_ts_config(&path.join("tsconfig.json"))?;
                } else {
                }
            }
        }
    }
}

struct Config {
    paths: Vec<std::path::PathBuf>,
}

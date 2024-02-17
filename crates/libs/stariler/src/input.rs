#[derive(Debug)]
pub enum DataKind {
    Files {
        source_paths: Vec<std::path::PathBuf>,
    },
    Project {
        ts_config_path: std::path::PathBuf,
    },
}

#[derive(Debug)]
pub struct Data {
    pub kind: DataKind,
}

struct Info {
    paths: Vec<std::path::PathBuf>,
}

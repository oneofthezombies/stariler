use tracing::debug;

#[derive(Debug)]
pub struct ArgInput {
    pub files: Option<Vec<String>>,
    pub project: Option<String>,
}

async fn parse_arg_input(arg_input: ArgInput) -> crate::Result<ArgOutput> {
    let kind = match (arg_input.files, arg_input.project) {
        (None, None) => {
            debug!("No input files or project specified, use current directory as project path");
            let ts_config_path = std::env::current_dir()?.join(crate::core::TS_CONFIG_FILE_NAME);
            let metadata = tokio::fs::metadata(&ts_config_path).await?;
            if !metadata.is_file() {
                return Err(crate::Error::TsConfigNotFound {
                    path: ts_config_path,
                });
            }
            ArgOutputKind::Project { ts_config_path }
        }
        (Some(files), None) => {
            debug!("Input files specified, use as source paths");
            if files.is_empty() {
                return Err(crate::Error::FilesArgEmpty);
            }
            let mut source_paths = vec![];
            for file in files {
                let path = std::path::PathBuf::from(file);
                let metadata = tokio::fs::metadata(&path).await?;
                if !metadata.is_file() {
                    return Err(crate::Error::FileArgNotFound { path });
                }
                source_paths.push(path);
            }
            ArgOutputKind::Files { source_paths }
        }
        (None, Some(project)) => {
            debug!("Project specified, use as project path");
            let ts_config_path =
                std::path::PathBuf::from(project).join(crate::core::TS_CONFIG_FILE_NAME);
            let metadata = tokio::fs::metadata(&ts_config_path).await?;
            if !metadata.is_file() {
                return Err(crate::Error::TsConfigNotFound {
                    path: ts_config_path,
                });
            }
            ArgOutputKind::Project { ts_config_path }
        }
        (Some(files), Some(project)) => {
            debug!("Input files and project specified, disallow both at the same time");
            return Err(crate::Error::FilesArgAndProjectArgConflict { files, project });
        }
    };
    Ok(ArgOutput { kind })
}

#[derive(Debug)]
enum ArgOutputKind {
    Files {
        source_paths: Vec<std::path::PathBuf>,
    },
    Project {
        ts_config_path: std::path::PathBuf,
    },
}

#[derive(Debug)]
struct ArgOutput {
    kind: ArgOutputKind,
}

#[derive(Debug)]
struct ConfigInput {
    ts_config_path: std::path::PathBuf,
}

#[derive(Debug)]
struct ConfigOutput {
    source_paths: Vec<std::path::PathBuf>,
}

async fn parse_config_input(config_input: ConfigInput) -> crate::Result<ConfigOutput> {
    let ts_config_path = config_input.ts_config_path;
    let ts_config_content = tokio::fs::read_to_string(&ts_config_path).await?;
    let ts_config: crate::tsconfig::TsConfig = serde_json::from_str(&ts_config_content)?;
    let ts_config = crate::tsconfig::update_exclude(ts_config);
    Ok(ConfigOutput {
        source_paths: vec![],
    })
}

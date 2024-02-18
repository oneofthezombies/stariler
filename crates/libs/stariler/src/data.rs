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
            let mut join_set: tokio::task::JoinSet<
                crate::Result<(std::path::PathBuf, std::fs::Metadata)>,
            > = tokio::task::JoinSet::new();
            let _ = files
                .iter()
                .map(std::path::PathBuf::from)
                .map(|path| async move {
                    {
                        let metadata = tokio::fs::metadata(&path).await?;
                        Ok((path, metadata))
                    }
                })
                .map(|task| join_set.spawn(task));
            let mut source_paths = Vec::new();
            while let Some(join_res) = join_set.join_next().await {
                let (path, metadata) = join_res??;
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
    let ts_config = crate::tsconfig::resolve_exclude(ts_config);
    let source_path_set = ts_config.files.iter().map(std::path::PathBuf::from).fold(
        std::collections::HashSet::new(),
        |mut set, path| {
            set.insert(path);
            set
        },
    );
    let exclude_path_set: std::collections::HashSet<std::path::PathBuf> = ts_config
        .exclude
        .into_iter()
        .map(std::path::PathBuf::from)
        .collect();
    let source_path_set = ts_config
        .include
        .iter()
        .map(|include| glob::glob(include))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|path| !exclude_path_set.contains(path))
        .fold(source_path_set, |mut set, path| {
            set.insert(path);
            set
        });
    Ok(ConfigOutput {
        source_paths: source_path_set.into_iter().collect(),
    })
}

async fn parse_arg_output(arg_output: ArgOutput) -> crate::Result<Vec<std::path::PathBuf>> {
    match arg_output.kind {
        ArgOutputKind::Files { source_paths } => Ok(source_paths),
        ArgOutputKind::Project { ts_config_path } => {
            let config_input = ConfigInput { ts_config_path };
            let config_output = parse_config_input(config_input).await?;
            Ok(config_output.source_paths)
        }
    }
}

async fn run(arg_input: ArgInput) -> crate::Result<()> {
    let arg_output = parse_arg_input(arg_input).await?;
    let source_paths = parse_arg_output(arg_output).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run() {
        let arg_input = ArgInput {
            files: None,
            project: None,
        };
        // let result = run(arg_input).await;
        // assert!(result.is_ok());
    }
}

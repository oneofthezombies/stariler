#[derive(Debug, clap::Parser)]
#[command(version)]
#[command(about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(conflicts_with("project"))]
    pub(crate) files: Option<Vec<String>>,

    #[arg(short, long)]
    #[arg(
        help = "Compile the project given the path to its configuration file, or to a folder with a 'tsconfig.json'."
    )]
    #[arg(conflicts_with("files"))]
    pub(crate) project: Option<String>,
}

impl stariler::core::TryFromAsync<crate::cli::Cli> for stariler::input::Data {
    type Error = stariler::Error;

    async fn try_from_async(cli: crate::cli::Cli) -> stariler::Result<stariler::input::Data> {
        let kind = match (cli.files, cli.project) {
            (None, None) => {
                let ts_config_path =
                    std::env::current_dir()?.join(stariler::core::TS_CONFIG_FILE_NAME);
                let metadata = tokio::fs::metadata(&ts_config_path).await?;
                if !metadata.is_file() {
                    return Err(stariler::Error::NotFound {
                        path: ts_config_path.to_string_lossy().to_string(),
                    });
                }
                Ok(stariler::input::DataKind::Project { ts_config_path })
            }
            (None, Some(project)) => {
                let project_path = std::path::PathBuf::from(project);
                let metadata = tokio::fs::metadata(&project_path).await?;
                if !metadata.is_dir() {
                    return Err(stariler::Error::NotFound {
                        path: project_path.to_string_lossy().to_string(),
                    });
                }

                let ts_config_path = project_path.join(stariler::core::TS_CONFIG_FILE_NAME);
                let metadata = tokio::fs::metadata(&ts_config_path).await?;
                if !metadata.is_file() {
                    return Err(stariler::Error::NotFound {
                        path: ts_config_path.to_string_lossy().to_string(),
                    });
                }

                Ok(stariler::input::DataKind::Project { ts_config_path })
            }
            (Some(files), None) => {
                let mut source_paths = Vec::new();
                for file in files {
                    let source_path = std::path::PathBuf::from(file);
                    let metadata = tokio::fs::metadata(&source_path).await?;
                    if !metadata.is_file() {
                        return Err(stariler::Error::NotFound {
                            path: source_path.to_string_lossy().to_string(),
                        });
                    }
                    source_paths.push(source_path);
                }
                Ok(stariler::input::DataKind::Files { source_paths })
            }
            (Some(_), Some(_)) => Err(stariler::Error::ConflictArgs {
                reason: "Cannot use both 'files' and 'project' arguments".to_owned(),
            }),
        }?;
        Ok(stariler::input::Data { kind })
    }
}

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

impl TryFrom<crate::cli::Cli> for stariler::core::Input {
    type Error = stariler::Error;

    fn try_from(cli: crate::cli::Cli) -> stariler::Result<stariler::core::Input> {
        let kind = match (cli.files, cli.project) {
            (None, None) => stariler::core::InputKind::Project(".".to_string()),
            (None, Some(project)) => stariler::core::InputKind::Project(project),
            (Some(files), None) => stariler::core::InputKind::Files(files),
            (Some(_), Some(_)) => {
                return Err(stariler::Error::ConflictArgs {
                    reason: "files and project are mutually exclusive".to_string(),
                })
            }
        };
        Ok(stariler::core::Input { kind })
    }
}

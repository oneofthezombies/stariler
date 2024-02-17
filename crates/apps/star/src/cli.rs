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

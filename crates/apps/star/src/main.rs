mod cli;

use clap::Parser;
use tracing::debug;

fn init_log() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}

fn main() -> stariler::Result<()> {
    init_log();
    let cli = crate::cli::Cli::parse();
    debug!(cli = ?cli, "cli");
    let input = stariler::core::Input::try_from(cli)?;
    debug!(input = ?input, "input");
    Ok(())
}

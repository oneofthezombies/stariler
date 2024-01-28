use clap::{command, Command};
use std::error::Error;

fn init() -> Result<(), Box<dyn Error>> {
    println!("Initializing development environment...");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .name("dev")
        .about("Development environment management tool for this project.")
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("Initialize development environment."))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init()?;
    }

    Ok(())
}

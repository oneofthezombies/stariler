use clap::{command, Command};
use std::error::Error;

fn init() -> Result<(), Box<dyn Error>> {
    println!("Initializing development environment...");
    // create directory references/typescript

    // "git", "clone", "--depth", "1", "--branch", "v5.3.3", "https://github.com/oneofthezombies/TypeScript.git" on references/typescript
    // npm install on references/sample
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

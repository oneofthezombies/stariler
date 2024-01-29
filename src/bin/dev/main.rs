use clap::{command, Command as ClapCommand};
use std::convert::AsRef;
use std::error::Error;
use std::fs;
use std::process::Command as ProcCommand;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
enum DevCommand {
    #[strum(serialize = "init")]
    Init,
}

fn init() -> Result<(), Box<dyn Error>> {
    println!("Initialize development environment...");

    fs::create_dir_all("references/typescript")?;
    if fs::metadata("references/typescript").is_ok() {
        println!("TypeScript repository already exists. Skip clone.");
    } else {
        println!("Clone TypeScript repository...");
        let _ = ProcCommand::new("git")
            .args(&[
                "clone",
                "--depth",
                "1",
                "--branch",
                "v5.3.3",
                "https://github.com/oneofthezombies/TypeScript.git",
                ".",
            ])
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir("references/typescript")
            .spawn()?
            .wait()?;
    }

    println!("Install Sample project dependencies...");
    let _ = ProcCommand::new("npm")
        .args(&["install"])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .current_dir("references/sample")
        .spawn()?
        .wait()?;

    println!("Done.");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .name("dev")
        .about("Development environment management tool for this project.")
        .arg_required_else_help(true)
        .subcommand(
            ClapCommand::new(DevCommand::Init.as_ref())
                .about("Initialize development environment."),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches(DevCommand::Init.as_ref()) {
        init()?;
    }

    Ok(())
}

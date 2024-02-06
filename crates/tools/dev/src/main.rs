use clap::{arg, command, Parser, Subcommand};
use std::io::Write;
use std::{
    env,
    path::Path,
    process::{self, Stdio},
};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Check,
    Clippy,
    Fmt,
    Build {
        #[arg(short, long)]
        target: String,
    },
    Test {
        #[arg(short, long)]
        target: Option<String>,
    },
    PrePush,
}

fn run(program: &str, args: &[&str]) {
    let mut command = process::Command::new(program);
    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(args);
    println!("Run {program} {args:?}");
    match command.status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Exit code: {:?}", status.code());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        }
    }
}

fn check() {
    run("cargo", &["check", "--workspace"]);
}

fn clippy() {
    run(
        "cargo",
        &[
            "clippy",
            "--",
            "-D",
            "clippy::all",
            "-D",
            "clippy::pedantic",
        ],
    );
}

fn fmt() {
    run("cargo", &["fmt", "--", "--check"]);
}

fn build(target: &str) {
    if env::var("GITHUB_ACTIONS").is_ok() && cfg!(target_os = "linux") {
        run("sudo", &["apt", "install", "musl-tools"]);
    }

    env::set_var("RUSTFLAGS", "-C target-feature=+crt-static");
    run("rustup", &["target", "add", target]);
    run("cargo", &["build", "-p", "star", "-r", "--target", target]);

    if env::var("GITHUB_ACTIONS").is_ok() {
        let output = env::var("GITHUB_OUTPUT").expect("No GITHUB_OUTPUT");
        let windows_path = Path::new("target")
            .join(target)
            .join("release")
            .join("star.exe");
        let file_path = if windows_path.exists() {
            windows_path
        } else {
            Path::new("target")
                .join(target)
                .join("release")
                .join("star")
        };

        if cfg!(unix) {
            run("chmod", &["+x", file_path.to_str().unwrap()]);
        }

        let mut output_path = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(output)
            .unwrap();
        writeln!(output_path, "ARTIFACT_PATH={}", file_path.to_str().unwrap()).unwrap();
    }
}

fn test(target: Option<String>) {
    let Some(target) = target else {
        run("cargo", &["test", "--workspace"]);
        return;
    };

    run("cargo", &["test", "--target", target.as_str()]);
}

fn pre_push() {
    check();
    clippy();
    fmt();
    test(None);
}

fn main() {
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        panic!("No command");
    };

    match command {
        Command::Check => check(),
        Command::Clippy => clippy(),
        Command::Fmt => fmt(),
        Command::Build { target } => build(&target),
        Command::Test { target } => test(target),
        Command::PrePush => pre_push(),
    }
}

// use clap::{command, Command as ClapCommand};
// use std::convert::AsRef;
// use std::error::Error;
// use std::fs;
// use std::process::Command as ProcCommand;
// use strum_macros::AsRefStr;

// #[derive(AsRefStr, Debug)]
// enum DevCommand {
//     #[strum(serialize = "init")]
//     Init,
// }

// fn init() -> Result<(), Box<dyn Error>> {
//     println!("Initialize development environment...");

//     fs::create_dir_all("references/typescript")?;
//     if fs::metadata("references/typescript").is_ok() {
//         println!("TypeScript repository already exists. Skip clone.");
//     } else {
//         println!("Clone TypeScript repository...");
//         let _ = ProcCommand::new("git")
//             .args(&[
//                 "clone",
//                 "--depth",
//                 "1",
//                 "--branch",
//                 "v5.3.3",
//                 "https://github.com/oneofthezombies/TypeScript.git",
//                 ".",
//             ])
//             .stdin(std::process::Stdio::inherit())
//             .stdout(std::process::Stdio::inherit())
//             .stderr(std::process::Stdio::inherit())
//             .current_dir("references/typescript")
//             .spawn()?
//             .wait()?;
//     }

//     println!("Install Sample project dependencies...");
//     let _ = ProcCommand::new("npm")
//         .args(&["install"])
//         .stdin(std::process::Stdio::inherit())
//         .stdout(std::process::Stdio::inherit())
//         .stderr(std::process::Stdio::inherit())
//         .current_dir("references/sample")
//         .spawn()?
//         .wait()?;

//     println!("Done.");
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let matches = command!()
//         .name("dev")
//         .about("Development environment management tool for this project.")
//         .arg_required_else_help(true)
//         .subcommand(
//             ClapCommand::new(DevCommand::Init.as_ref())
//                 .about("Initialize development environment."),
//         )
//         .get_matches();

//     if let Some(_) = matches.subcommand_matches(DevCommand::Init.as_ref()) {
//         init()?;
//     }

//     Ok(())
// }

use clap::{arg, command, Parser, Subcommand};
use std::fs;
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
    Init,
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

struct Runner {
    program: String,
    args: Vec<String>,
    cwd: String,
}

impl Runner {
    fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: Vec::new(),
            cwd: ".".to_string(),
        }
    }

    fn args(mut self, args: &[&str]) -> Self {
        self.args
            .extend(args.iter().map(std::string::ToString::to_string));
        self
    }

    fn cwd(mut self, cwd: &str) -> Self {
        self.cwd = cwd.to_string();
        self
    }

    fn run(self) {
        let mut command = process::Command::new(self.program.to_string());
        command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .args(&self.args)
            .current_dir(self.cwd);
        println!("Run {} {:?}", self.program, self.args);
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
}

fn init() {
    fs::create_dir_all("references/typescript").unwrap();
    if fs::metadata("references/typescript").is_ok() {
        println!("TypeScript repository already exists. Skip clone.");
    } else {
        println!("Clone TypeScript repository...");
        Runner::new("git")
            .args(&[
                "clone",
                "--depth",
                "1",
                "--branch",
                "v5.3.3",
                "https://github.com/oneofthezombies/TypeScript.git",
            ])
            .cwd("references/typescript")
            .run();
    }

    Runner::new("npm")
        .args(&["install"])
        .cwd("references/sample")
        .run();
    println!("Done init.");
}

fn check() {
    Runner::new("cargo").args(&["check", "--workspace"]).run();
}

fn clippy() {
    Runner::new("cargo")
        .args(&[
            "clippy",
            "--",
            "-D",
            "clippy::all",
            "-D",
            "clippy::pedantic",
        ])
        .run();
}

fn fmt() {
    Runner::new("cargo").args(&["fmt", "--", "--check"]).run();
}

fn build(target: &str) {
    if env::var("GITHUB_ACTIONS").is_ok() && cfg!(target_os = "linux") {
        Runner::new("sudo")
            .args(&["apt", "install", "musl-tools"])
            .run();
    }

    env::set_var("RUSTFLAGS", "-C target-feature=+crt-static");
    Runner::new("rustup").args(&["target", "add", target]).run();
    Runner::new("cargo")
        .args(&["build", "-p", "star", "-r", "--target", target])
        .run();

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
            Runner::new("chmod")
                .args(&["+x", file_path.to_str().unwrap()])
                .run();
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
        Runner::new("cargo").args(&["test", "--workspace"]).run();
        return;
    };

    Runner::new("cargo")
        .args(&["test", "--target", target.as_str()])
        .run();
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
        Command::Init => init(),
        Command::Check => check(),
        Command::Clippy => clippy(),
        Command::Fmt => fmt(),
        Command::Build { target } => build(&target),
        Command::Test { target } => test(target),
        Command::PrePush => pre_push(),
    }
}

use clap::{arg, command, Parser, Subcommand};
use sheller::{new, run, CommandExt};
use std::fs;
use std::io::Write;
use std::{env, path::Path};
use tracing::info;

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
    Test,
    PrePush,
}

/// During Github Actions Workflow, when running `rustup install nightly` inside a `cargo run --package tool-dev -- init` command on a Windows platform, it will fail with the following error:
/// ```text
/// error: could not create link from 'C:\Users\runneradmin\.cargo\bin\rustup.exe' to 'C:\Users\runneradmin\.cargo\bin\cargo.exe'
/// ```
/// So for Github Action, I changed to call `rustup install nightly` before calling `cargo run --package tool-dev -- init`.
/// Please see the workflow file at `.github/workflows/CI.yml`.
fn init() {
    let is_run_on_github_actions = env::var("GITHUB_ACTIONS").is_ok();
    if !is_run_on_github_actions {
        run!("rustup install nightly");
    }

    run!("rustup component add rustfmt clippy --toolchain nightly");
    run!("rustup override set nightly");

    if !is_run_on_github_actions {
        fs::create_dir_all("references/typescript").unwrap();
        if fs::metadata("references/typescript").is_ok() {
            info!("TypeScript repository already exists. Skip clone.");
        } else {
            info!("Clone TypeScript repository...");
            new!(
                "git clone --depth 1 --branch v5.3.3 https://github.com/oneofthezombies/TypeScript.git"
            )
            .build()
            .current_dir("references/typescript")
            .run();
        }
        new!("npm install")
            .build()
            .current_dir("references/sample")
            .run();
        info!("TypeScript references is ready.");
    }
}

fn check() {
    run!("cargo check --workspace --all-targets --all-features");
}

fn clippy() {
    run!("cargo clippy --workspace --all-targets --all-features -- -D clippy::all -D clippy::pedantic");
}

fn fmt() {
    run!("cargo fmt --all --check");
}

fn build(target: &str) {
    let is_run_on_github_actions = env::var("GITHUB_ACTIONS").is_ok();
    if is_run_on_github_actions && cfg!(target_os = "linux") {
        run!("sudo apt install musl-tools");
    }

    env::set_var("RUSTFLAGS", "-C target-feature=+crt-static");
    run!("rustup target add {target}");
    run!("cargo build --package star --release --target {target}");

    if is_run_on_github_actions {
        let output_path = env::var("GITHUB_OUTPUT").expect("No GITHUB_OUTPUT");
        let release_dir_path = Path::new("target").join(target).join("release");
        let windows_exe_path = release_dir_path.join("star.exe");
        let exe_path = if windows_exe_path.exists() {
            windows_exe_path
        } else {
            release_dir_path.join("star")
        };

        if cfg!(unix) {
            run!("chmod +x {}", exe_path.display());
        }

        let mut output_file = std::fs::OpenOptions::new()
            .append(true)
            .open(output_path)
            .unwrap();
        writeln!(output_file, "ARTIFACT_PATH={}", exe_path.display()).unwrap();
    }
}

fn test() {
    run!("cargo test --workspace --all-targets --all-features -- --nocapture");
}

fn pre_push() {
    check();
    clippy();
    fmt();
    test();
}

fn init_log() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}

fn main() {
    init_log();
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
        Command::Test => test(),
        Command::PrePush => pre_push(),
    }
}

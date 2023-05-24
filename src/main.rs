use std::env;
use std::error::Error;
use std::process::Command;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    environment: Option<String>,
    #[arg(required = true)]
    command: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _errors = load_dotenvs_with_default_precedence(&args.environment);

    let mut child = Command::new(&args.command[0])
        .args(&args.command[1..])
        .spawn()?;
    let exit_code = child.wait()?;
    std::process::exit(exit_code.code().unwrap_or(0))
}

/// precedence order is
/// 1. already set env vars
/// 2. .env.{environment}.local
/// 3. .env.local
/// 4. .env.{environment}
/// 5. .env
fn load_dotenvs_with_default_precedence(
    environment: &Option<String>,
) -> Result<(), Vec<Box<dyn Error>>> {
    let mut errors: Vec<Box<dyn Error>> = Vec::new();

    let mut load_dotenv_file = |filename: &str| {
        dotenv::from_path(
            env::current_dir()
                .expect("current directory is invalid")
                .join(filename),
        )
        .map_err(|err| errors.push(err.into()))
        .ok();
    };

    // .env.{environment}.local
    if let Some(env) = &environment {
        load_dotenv_file(&format!(".env.{env}.local"));
    }

    // .env.local
    load_dotenv_file(".env.local");

    // .env.{environment}
    if let Some(env) = &environment {
        load_dotenv_file(&format!(".env.{env}"));
    }

    // .env
    load_dotenv_file(".env");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

use std::process::Command;

use anyhow::bail;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    environment: Option<String>,
    command: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let _errors = load_dotenvs_with_default_precedence(&args.environment);

    if args.command.get(0).is_none() {
        bail!("must provide a command")
    }
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
) -> Result<(), Vec<anyhow::Error>> {
    let mut errors: Vec<anyhow::Error> = Vec::new();
    let mut load_dotenv_file = |filename: &str| {
        dotenv::from_filename(filename)
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

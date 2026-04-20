use anyhow::Result;
use clap::Parser;

mod error;
mod tracer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Executable to run.
    #[arg(required = true, trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();
    tracer::spawn(&cli_args.command)?;
    Ok(())
}

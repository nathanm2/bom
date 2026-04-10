use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Configuration file.
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// Executable to run.
    #[arg(required = true, trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() {
    let args = Args::parse();
}

use clap::Parser;
use gytmir_lib::sync;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None
)]

struct Cli {
    #[arg(short = None, long, default_value = "../doxtractor")]
    /// Path to repository directory
    repo_dir: String,

    #[arg(short = None, long, default_value = "master")]
    /// Branch to fetch
    branch: String,

    #[arg(short = None, long, default_value = "origin")]
    /// Remote name
    remote: String,

    #[arg(short = None, long, default_value = ".ssh/id_rsa")]
    /// Set ssh key
    key_file: String,
}

fn main() {
    let cli = Cli::parse();
    eprintln!("{:?}", cli);

    sync(&cli.repo_dir, &cli.remote, &cli.key_file, &cli.branch);
}

use git2::Repository;
use git2::{Cred, RemoteCallbacks, FetchOptions, PushOptions};
use std::env;
use clap::Parser;

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

fn credentials_remote_callbacks(key_file: &str) -> RemoteCallbacks {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, username_from_url, _allowed_types | {
      Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        std::path::Path::new(
            &format!("{}/{}", env::var("HOME").unwrap(), key_file)
        ),
        None,
      )
    });

    callbacks
}

fn credentials_in_fetch_options(key_file: &str) -> FetchOptions {
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(credentials_remote_callbacks(key_file));

    fetch_options
}

fn credentials_in_push_options(key_file: &str) -> PushOptions {
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(credentials_remote_callbacks(key_file));

    push_options
}

fn main() {
    let cli = Cli::parse();
    eprintln!("{:?}", cli);

    let repo = Repository::open(cli.repo_dir).unwrap();
    println!("{:?}", repo.head().unwrap().name().unwrap());
    for remote in repo.remotes().unwrap().iter() {
        println!("{:?}", remote);
    }
    let mut remote = repo.find_remote(&cli.remote).unwrap();
    for refspec in remote.fetch_refspecs().unwrap().iter() {
        println!("{:?}", refspec);
    }

    let mut fetch_options = credentials_in_fetch_options(&cli.key_file);

    let () = remote.fetch(&[&cli.branch], Some(&mut fetch_options), None).unwrap();

    let mut mirror_remote = repo.remote("mirror", "git@gitlab.com:olorin37/doxtractor.git").unwrap();

    let mut push_options = credentials_in_push_options(&cli.key_file);
    let _ = mirror_remote.push(&["refs/remotes/origin/master"], Some(&mut push_options)).unwrap();

    println!("Mirror remote: {:?}", mirror_remote.url().unwrap());
}

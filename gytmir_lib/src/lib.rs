use git2::Repository;
use git2::{Cred, RemoteCallbacks, FetchOptions, PushOptions};
use std::env;

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

pub fn sync(
    repo_dir: &str,
    remote: &str,
    key_file: &str,
    branch: &str,
) {
    let repo = Repository::open(repo_dir).unwrap();
    println!("{:?}", repo.head().unwrap().name().unwrap());
    for remote in repo.remotes().unwrap().iter() {
        println!("{:?}", remote);
    }
    let mut remote = repo.find_remote(&remote).unwrap();
    for refspec in remote.fetch_refspecs().unwrap().iter() {
        println!("{:?}", refspec);
    }

    let mut fetch_options = credentials_in_fetch_options(&key_file);

    let () = remote.fetch(&[&branch], Some(&mut fetch_options), None).unwrap();

    let mut mirror_remote = match repo.find_remote("mirror") {
        Ok(remote) => remote,
        Err(_) => repo.remote("mirror", "git@gitlab.com:olorin37/doxtractor.git").unwrap(),
    };

    let mut push_options = credentials_in_push_options(&key_file);
    let _ = mirror_remote.push(&["refs/remotes/origin/master"], Some(&mut push_options)).unwrap();

    println!("Mirror remote: {:?}", mirror_remote.url().unwrap());
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

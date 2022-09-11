use anyhow::Result;
use rust_solv::{config, repo, solve};
use std::{env, path::Path};

fn main() -> Result<()> {
    let packages: Vec<String> = env::args()
        .enumerate()
        .filter(|&(i, _)| i > 0)
        .map(|(_, v)| v)
        .collect();
    if packages.is_empty() {
        panic!("Package name not found!");
    } else {
        let cfg = config::Config::from_file(Path::new("~/.config/rust-solv/config.toml"))?;
        if let Some(repo_baseurl) = cfg.get_repo_baseurl() {
            let repo = repo::Repo::from_baseurl(repo_baseurl)?;
            for package_name in packages {
                match solve::check_package_satisfiability_in_repo(&repo, &package_name) {
                    Ok(true) => println!("Congratulations! Package {}'s dependencies can be satisfied in the repo. :)", package_name),
                    Ok(false) => println!("Sorry, package {}'s dependencies can not be satisfied in the repo. :(", package_name),
                    Err(_) => println!("Error: something wrong happened while solving the dependency problem of package {}.", package_name),
                }
            }
            Ok(())
        } else {
            panic!("Repo baseurl not found! Please check the config file!");
        }
    }
}
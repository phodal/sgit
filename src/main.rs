#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use std::thread;

use clap::Command;
use ini::Ini;
use url::Url;
use walkdir::{DirEntry, WalkDir};

use crate::git_wrapper::GitWrapper;
use crate::sgit::Sgit;

mod sgit;
mod git_wrapper;

static SGIT_FILE: &str = "sgit.yaml";

fn cli() -> Command<'static> {
    Command::new("sgit")
        .about("A multiple repo's git cli")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(Command::new("init").about("init sgit config"))
        .subcommand(Command::new("clone").about("Clones repos"))
        .subcommand(Command::new("push").about("pushes things"))
        .subcommand(Command::new("gen").about("generate sgit by org"))
        .subcommand(Command::new("add").about("add a repos !! not implement"))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            if !PathBuf::from(SGIT_FILE).exists() {
                let mut file = File::create(SGIT_FILE).unwrap();

                let repos: Vec<String> = try_load_git_config_by_paths();
                let sgit = Sgit { repos, organization: None, token: None };

                Sgit::write_sgit_to_file(&mut file, sgit)
            } else {
                error!("{}", format!("{} is exists, will not create", SGIT_FILE));
            }
        }
        Some(("gen", _)) => {
            let sgit = Sgit::load_sgit();
            if sgit.token.is_none() || sgit.organization.is_none() {
                error!("{}", "cannot found token or organization");
                exit(1);
            }

            let repos = Sgit::fetch_repos(&sgit).await;

            let mut file = File::create(SGIT_FILE).unwrap();
            let sgit = Sgit { repos, organization: sgit.organization, token: sgit.token };
            Sgit::write_sgit_to_file(&mut file, sgit)
        }
        Some(("clone", _)) => {
            let sgit = Sgit::load_sgit();

            let threads: Vec<_> = sgit.repos.into_iter()
                .map(|repo| {
                    thread::spawn(move || {
                        GitWrapper::new(&repo).try_clone();
                    })
                })
                .collect();

            for handle in threads {
                handle.join().unwrap()
            }
        }
        Some(("pull", _)) => {
            let sgit = Sgit::load_sgit();
            let threads: Vec<_> = sgit.repos.into_iter()
                .map(|repo| {
                    thread::spawn(move || {
                        GitWrapper::new(&repo).try_pull();
                    })
                })
                .collect();

            for handle in threads {
                handle.join().unwrap()
            }
        }
        _ => {
            error!("unsupported command")
        }
    }
}

fn is_clone_url_correct(repo: Option<Url>) -> Option<String> {
    if repo.is_some() {
        return Some(repo.unwrap().to_string())
    } else {
        None
    }
}

fn try_load_git_config_by_paths() -> Vec<String> {
    let walker = WalkDir::new(".").max_depth(1).into_iter();
    walker
        .filter_map(|e| e.ok())
        .filter(|entry| git_config_path(entry).exists())
        .map(|entry| {
            let conf = Ini::load_from_file(git_config_path(&entry)).unwrap();
            match conf.section(Some("remote \"origin\"")) {
                Some(section) => {
                    section.get("url").unwrap().to_string()
                }
                None => {
                    "".to_string()
                }
            }
        })
        .filter(|path| !path.is_empty())
        .collect()
}

fn git_config_path(entry: &DirEntry) -> PathBuf {
    entry.path().join(".git").join("config")
}

#[cfg(test)]
mod tests {
    use crate::{Sgit, try_load_git_config_by_paths};

    #[test]
    fn test_load_git_config() {
        let paths = try_load_git_config_by_paths();
        assert!(paths.len() >= 1);
    }

    #[test]
    fn test_load_sgit_config() {
        let sgit = Sgit::load_sgit();
        assert_eq!(sgit.repos.len(), 1);
    }
}
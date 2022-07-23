#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::thread;

use clap::Command;
use ini::Ini;
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
        .subcommand(Command::new("add").about("add a repos !! not implement"))
}

fn main() {
    pretty_env_logger::init();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            if !PathBuf::from(SGIT_FILE).exists() {
                let mut file = File::create(SGIT_FILE).unwrap();

                // try: to load from .git/config
                let repos: Vec<String> = try_load_from_path();
                let sgit = Sgit { repos };

                file.write_all(sgit.to_str().as_ref()).expect("init with write file failure")
            } else {
                error!("{}", format!("{} is exists, will not create", SGIT_FILE));
            }
        }
        Some(("clone", _)) => {
            let sgit = load_sgit();

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
            let sgit = load_sgit();
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

fn try_load_from_path() -> Vec<String> {
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

fn load_sgit() -> Sgit {
    let maybe_file = File::open(SGIT_FILE);
    if maybe_file.is_err() {
        error!("{}", format!("cannot find `{}` file", SGIT_FILE));
        exit(1);
    }

    let mut file = maybe_file.unwrap();

    let mut str: String = "".to_string();
    file.read_to_string(&mut str).expect(&*format!("cannot read `{}` file", SGIT_FILE));
    let sgit = Sgit::from_str(str.as_str());
    sgit
}

#[cfg(test)]
mod tests {
    use crate::try_load_from_path;

    #[test]
    fn load_path() {
        let paths = try_load_from_path();
        assert!(paths.len() >= 1);
    }
}
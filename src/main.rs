#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::path::PathBuf;
use std::process::exit;
use std::thread;

use clap::Command;

use crate::git_config::GitConfig;
use crate::git_wrapper::GitWrapper;
use crate::sgit::Sgit;

mod sgit;
mod git_wrapper;
mod git_config;

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
                let repos: Vec<String> = GitConfig::try_load_git_config_by_paths();

                let sgit = Sgit { repos, organization: None, token: None };
                sgit.write_to_file();
            } else {
                error!("{}", format!("{} is exists, will not create", SGIT_FILE));
            }
        }
        Some(("gen", _)) => {
            let sgit = Sgit::from_path();
            if sgit.token.is_none() || sgit.organization.is_none() {
                error!("{}", "cannot found token or organization");
                exit(1);
            }

            let repos = Sgit::fetch_repos(&sgit).await;

            let sgit = Sgit { repos, organization: sgit.organization, token: sgit.token };
            sgit.write_to_file();
        }
        Some(("clone", _)) => {
            execute_in_threads(Sgit::from_path(), GitWrapper::clone_action);
        }
        Some(("pull", _)) => {
            execute_in_threads(Sgit::from_path(), GitWrapper::pull_action);
        }
        _ => {
            error!("unsupported command")
        }
    }
}

fn execute_in_threads(sgit: Sgit, action: fn(&String)) {
    let threads: Vec<_> = sgit.repos.into_iter()
        .map(|repo| {
            thread::spawn(move || {
                action(&repo);
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::git_config::GitConfig;
    use crate::Sgit;

    #[test]
    fn test_load_git_config() {
        let paths = GitConfig::try_load_git_config_by_paths();
        assert!(paths.len() >= 1);
    }

    #[test]
    fn test_load_sgit_config() {
        let sgit = Sgit::from_path();
        assert_eq!(sgit.repos.len(), 1);
    }
}
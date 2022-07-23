#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::fs::File;
use std::io::Read;
use std::process::exit;

use clap::Command;
use pretty_env_logger::init;

use crate::git_wrapper::GitWrapper;
use crate::sgit::Sgit;

mod sgit;
mod git_wrapper;

fn cli() -> Command<'static> {
    Command::new("sgit")
        .about("A multiple repo's git cli")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(Command::new("clone").about("Clones repos"))
        .subcommand(Command::new("push").about("pushes things"))
        .subcommand(Command::new("add").about("add a repos !! not implement"))
}

fn main() {
    pretty_env_logger::init();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("clone", _)) => {
            let maybe_file = File::open("sbgit.yaml");
            if maybe_file.is_err() {
                error!("cannot find file");
                exit(1);
            }

            let mut file = maybe_file.unwrap();

            let mut str: String = "".to_string();
            file.read_to_string(&mut str).expect("cannot read file");
            let sgit = Sgit::from_str(str.as_str());
            for repo in &sgit.repos {
                GitWrapper::new(repo).try_clone();
            };
        }
        Some(("pull", _)) => {
            info!("todo: ...");
        }
        _ => {
            error!("unsupported command")
        }
    }
}
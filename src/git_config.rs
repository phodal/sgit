use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use ini::Ini;
use walkdir::{DirEntry, WalkDir};

pub struct GitConfig {}

impl GitConfig {
    pub fn try_load_git_config_by_paths() -> Vec<String> {
        let walker = WalkDir::new(".").max_depth(1).into_iter();
        walker
            .filter_map(|e| e.ok())
            .filter(|entry| GitConfig::build_git_config_path(entry).exists())
            .map(|entry| { GitConfig::load_remote_url_by_path(&entry) })
            .filter(|path| !path.is_empty())
            .collect()
    }

    fn load_remote_url_by_path(entry: &DirEntry) -> String {
        let config_path = GitConfig::build_git_config_path(&entry);

        let mut str = String::new();
        let mut dst = File::open(config_path).unwrap();
        dst.read_to_string(&mut str).unwrap();

        GitConfig::remote_url_by_str(&str)
    }

    pub fn remote_url_by_str(str: &str) -> String {
        let conf = Ini::load_from_str(&str).unwrap();
        match conf.section(Some("remote \"origin\"")) {
            Some(section) => {
                section.get("url").unwrap().to_string()
            }
            None => {
                "".to_string()
            }
        }
    }

    fn build_git_config_path(entry: &DirEntry) -> PathBuf {
        entry.path().join(".git").join("config")
    }
}


#[cfg(test)]
mod tests {
    use crate::GitConfig;

    #[test]
    fn parse_remote_url() {
        let url = GitConfig::remote_url_by_str(r#"
[remote "origin"]
	url = https://github.com/phodal/sgit
	fetch = +refs/heads/*:refs/remotes/origin/*
"#);

        assert_eq!("https://github.com/phodal/sgit", url);
    }
}
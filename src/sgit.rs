use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::process::exit;
use std::io::{Read, Write};
use octocrab::Octocrab;
use octocrab::params::repos::{Sort, Type};
use crate::SGIT_FILE;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sgit {
    pub repos: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>
}

impl Sgit {
    pub fn from_str(str: &str) -> Sgit {
        let maybe_sgit: Sgit = serde_yaml::from_str(str).expect("cannot parse str");
        maybe_sgit
    }

    pub fn to_str(&self) -> String {
        serde_yaml::to_string(&self).unwrap_or_default()
    }
}

impl Sgit {
    pub fn load_sgit() -> Sgit {
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

    pub fn write_sgit_to_file(file: &mut File, sgit: Sgit) {
        file.write_all(sgit.to_str().as_ref()).expect("init with write file failure")
    }

    pub async fn fetch_repos(sgit: &Sgit) -> Vec<String> {
        let octocrab = Octocrab::builder().personal_token(sgit.token.clone().unwrap()).build().unwrap();
        let page = octocrab
            .orgs(sgit.organization.clone().unwrap())
            .list_repos()
            .repo_type(Type::Private)
            .per_page(100)
            .sort(Sort::Pushed)
            .send()
            .await
            .unwrap();

        page.into_iter()
            .map(|repo| repo.clone_url)
            .filter_map(|repo| crate::is_clone_url_correct(repo))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::sgit::Sgit;

    #[test]
    fn serial_yaml_from_string() {
        let sgit: Sgit = serde_yaml::from_str(r#"repos:
 - https://github.com/phodal/batch_git.git
"#).expect("TODO: panic message");

        assert_eq!(1, sgit.repos.len());
        assert_eq!("https://github.com/phodal/batch_git.git", sgit.repos[0]);
    }
}


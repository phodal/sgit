use std::process::Command;
use url::{Url};

#[derive(Debug)]
pub struct GitWrapper<'a> {
    repo: &'a str
}

impl<'a> GitWrapper<'a> {
    pub(crate) fn new(repo: &'a str) -> GitWrapper<'a> {
        GitWrapper {
            repo
        }
    }

    #[must_use]
    #[inline]
    fn git(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.env("GIT_DIR", &self.repo);
        cmd
    }

    pub fn get_repo_name(&self) -> Option<String> {
        let url = Url::parse(self.repo);
        if url.is_err() {
            return None
        }

        let url = url.unwrap();
        let maybe_paths = url.path_segments()
            .map(|c| {
                c.collect::<Vec<_>>()
            });

        match &maybe_paths {
            None => { None }
            Some(paths) => {
                let last = paths.last().unwrap_or(&"").to_string();
                Some(last)
            }
        }
    }

    pub fn clone(&self) {
        let mut cmd = self.git();
        cmd.arg("clone")
            .arg(self.repo);

        let output = cmd.output().expect("git command failed to start");

        info!("{}", String::from_utf8_lossy(&*output.stderr));
        info!("{:?}", String::from_utf8_lossy(&*output.stdout).to_string());
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::git_wrapper::GitWrapper;

    #[test]
    fn successful_clone() {
        GitWrapper::new("https://github.com/phodal/batch-git").clone();

        assert!(Path::new("batch-git").exists());
    }

    #[test]
    fn repo_name_success() {
        let name = GitWrapper::new("https://github.com/phodal/batch-git").get_repo_name();

        assert_eq!("batch-git", name.unwrap());
    }

    #[test]
    fn repo_name_return_empty() {
        let name = GitWrapper::new("batch-git").get_repo_name();

        assert!(name.is_none());
    }
}
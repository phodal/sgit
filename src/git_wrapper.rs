use std::path::PathBuf;
use std::process::Command;

use url::Url;

#[derive(Debug)]
pub struct GitWrapper<'a> {
    repo: &'a str,
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
        let cmd = Command::new("git");
        cmd
    }

    pub fn get_repo_name(&self) -> Option<String> {
        let url = Url::parse(self.repo);
        if url.is_err() {
            return None;
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

    pub fn try_clone(&self) {
        let maybe_repo_name = self.get_repo_name();
        if maybe_repo_name.is_none() {
            error!("cannot get repo name, try to fetch");
        }

        let repo_name = maybe_repo_name.unwrap();

        let is_already_exists = PathBuf::from(repo_name).exists();
        if is_already_exists {
            error!("target repo is exists, try to pull it");
            self.try_pull();
        } else {
            self.done_clone()
        }
    }

    pub fn done_clone(&self) {
        let mut cmd = self.git();
        cmd.arg("clone")
            .arg(self.repo);

        let output = cmd.output().expect("git command failed to start");

        info!("{}", String::from_utf8_lossy(&*output.stderr));
        info!("{:?}", String::from_utf8_lossy(&*output.stdout).to_string());
    }

    pub fn try_pull(&self) {
        let mut cmd = self.git();
        cmd.arg("-C").arg(self.get_repo_name().unwrap());
        cmd.arg("pull");

        info!("{:?}", cmd);

        let output = cmd.output().expect("pull code");

        info!("{}", String::from_utf8_lossy(&*output.stderr));
        info!("{}", String::from_utf8_lossy(&*output.stdout).to_string());
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::git_wrapper::GitWrapper;

    #[test]
    fn successful_clone() {
        GitWrapper::new("https://github.com/phodal/batch-git").done_clone();

        assert!(Path::new("batch-git").exists());
    }

    #[test]
    fn pull_code() {
        let wrapper = GitWrapper::new("https://github.com/phodal/batch-git");
        wrapper.done_clone();

        // todo: wrapper logger for test
        wrapper.try_pull();

        assert!(true);
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
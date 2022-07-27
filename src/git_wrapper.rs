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

    pub fn clone(repo: &String) {
        GitWrapper::new(&repo).try_clone()
    }

    pub fn pull(repo: &String) {
        GitWrapper::new(&repo).try_pull()
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
                let mut last = paths.last().unwrap_or(&"").to_string();
                let maybe_suffix = last.strip_suffix(".git");
                if maybe_suffix.is_some() {
                    last = maybe_suffix.unwrap().to_string();
                }

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

        info!("run `git clone` from path: {}", self.repo);

        let output = cmd.output().expect("git command failed to start");

        info!("{}", String::from_utf8_lossy(&*output.stderr));
        info!("{:?}", String::from_utf8_lossy(&*output.stdout).to_string());
    }

    pub fn try_clean(&self) {
        let mut cmd = self.with_path();
        cmd.arg("stash");

        info!("run `git stash` from path: {}", self.repo);

        let output = cmd.output().expect("stash code");
        info!("{}", String::from_utf8_lossy(&*output.stderr));
        info!("{}", String::from_utf8_lossy(&*output.stdout).to_string());
    }

    pub fn try_pull(&self) {
        self.try_clean();

        let mut cmd = self.with_path();
        cmd.arg("pull");

        info!("run `git pull` from path: {}", self.repo);

        let output = cmd.output().expect("pull code");

        info!("{}", String::from_utf8_lossy(&*output.stderr));
        info!("{}", String::from_utf8_lossy(&*output.stdout).to_string());
    }

    fn with_path(&self) -> Command {
        let mut cmd = self.git();
        cmd.arg("-C").arg(self.get_repo_name().unwrap());
        cmd
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::git_wrapper::GitWrapper;

    #[test]
    fn pull_code() {
        let wrapper = GitWrapper::new("https://github.com/phodal/batch-git");

        wrapper.done_clone();
        wrapper.try_pull();

        let path = Path::new("batch-git");
        assert!(path.exists());

        fs::remove_dir_all(Path::new("batch-git")).unwrap();
    }

    #[test]
    fn repo_name_success() {
        let name = GitWrapper::new("https://github.com/phodal/batch-git").get_repo_name();
        assert_eq!("batch-git", name.unwrap());
    }

    #[test]
    fn suffix_name_success() {
        let name = GitWrapper::new("https://github.com/phodal/batch-git.git").get_repo_name();
        assert_eq!("batch-git", name.unwrap());
    }

    #[test]
    fn repo_name_return_empty() {
        let name = GitWrapper::new("batch-git").get_repo_name();

        assert!(name.is_none());
    }
}

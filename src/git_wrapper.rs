use std::process::Command;
use url::{Url};

#[derive(Debug)]
pub struct GitWrapper<'a>(&'a str);

impl<'a> GitWrapper<'a> {
    #[must_use]
    #[inline]
    fn git(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.env("GIT_DIR", &self.0);
        cmd
    }

    pub fn get_repo_name(&self) -> Option<String> {
        let url = Url::parse(self.0);
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
            .arg(self.0);

        let output = cmd.output().expect("git command failed to start");

        println!("{}", String::from_utf8_lossy(&*output.stderr));
        println!("{:?}", String::from_utf8_lossy(&*output.stdout).to_string());
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::git_wrapper::GitWrapper;

    #[test]
    fn successful_clone() {
        GitWrapper("https://github.com/phodal/batch-git").clone();

        assert!(Path::new("batch-git").exists());
    }

    #[test]
    fn repo_name() {
        let name = GitWrapper("https://github.com/phodal/batch-git").get_repo_name();

        assert_eq!("batch-git", name.unwrap());
    }
}
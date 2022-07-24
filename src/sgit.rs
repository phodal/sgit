use serde::Deserialize;
use serde::Serialize;

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
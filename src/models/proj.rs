use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Language {
    Haskell,
    Rust,
    Python,
    Go,
    C,
    Cpp,
    OCaml,
    Javascript,
    Java,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RawProject {
    pub title: String,
    pub time: String,
    pub languages: Vec<Language>,
    pub status: ProjectStatus,
    pub tags: Vec<String>,
    pub preview: String,
    pub link: String,
    pub desc: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug, Eq, Serialize)]
pub enum ProjectStatus {
    Completed,
    Ongoing,
    Discontinued,
}

impl ProjectStatus {
    fn priority(&self) -> u8 {
        match self {
            ProjectStatus::Completed => 0,
            ProjectStatus::Ongoing => 100,
            ProjectStatus::Discontinued => 200,
        }
    }
}

impl PartialOrd for ProjectStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProjectStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

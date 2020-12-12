use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub url: String,
    pub watchers: usize,
    pub forks: usize,
    pub stars: usize,
}

pub struct Issue {
    author: String,
    url: String,
    title: String,
}

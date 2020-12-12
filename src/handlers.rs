use rocket::{get, post};
use rocket_contrib::json::Json;

use crate::types::Project;

#[get("/")]
pub fn root() -> String {
    "It works!".to_string()
}

#[get("/projects")]
pub fn get_projects() -> Json<Vec<Project>> {
    Json(vec![Project {
        url: "https://github.com/foo/bar".to_string(),
        name: "Bar Project".to_string(),
        stars: 0,
        watchers: 0,
        forks: 0,
    }])
}

// #[post("/projects")]
// pub fn create_project() -> Project {}

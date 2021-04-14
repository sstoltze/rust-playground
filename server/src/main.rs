#![feature(proc_macro_hygiene, decl_macro)] // Nightly-only language features needed by rocket

// Macros from rocket
#[macro_use]
extern crate rocket;
use rocket::State;
use rocket_contrib::json::Json;
use serde::*;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Create route / that returns "Hello, World!"
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TaskPerson {
    incubation: Option<i32>,
    contacts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    verdict: String,
    category: String,
    expectation: String,
    timeout: Option<f64>,
    people: HashMap<String, TaskPerson>,
}

impl Task {
    fn from_file(f: String) -> Option<Self> {
        let c = get_file(f).unwrap();
        serde_json::from_str(&c).ok()
    }

    fn get_person(&self, name: &str) -> Option<TaskPerson> {
        self.people.get(&name.to_string()).map(|c| c.clone())
    }
}

#[get("/<name>")]
fn get_name(task: State<Task>, name: String) -> Option<Json<TaskPerson>> {
    let s = task.get_person(&name)?;
    Some(Json(s))
}

fn get_file(file: String) -> Option<String> {
    let mut f = if let Ok(p) = std::fs::canonicalize(Path::new(&file)) {
        File::open(p)
    } else {
        File::open(format!("files/{}", file))
    }
    .ok()?;
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    Some(contents)
}

#[get("/names")]
fn get_names(task: State<Task>) -> Json<Vec<String>> {
    Json(task.people.keys().map(|s| s.clone()).collect())
}

#[get("/task")]
fn get_task(task: State<Task>) -> Json<Task> {
    Json(task.inner().clone())
}

fn main() {
    // Read in the given task and parse it
    let t = Task::from_file("greedy.task".to_string()).unwrap();

    rocket::ignite()
        .manage(t)
        .mount("/", routes![index, get_name, get_names, get_task])
        .launch();
}

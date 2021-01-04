use reqwest::{Response, Result};
use std::path::Path;

#[allow(dead_code)]
pub struct Account<'a> {
    username: &'a str,
    password: &'a str,
    premium: bool,
}

impl Account<'_> {
    pub fn new<'a>(username: &'a str, password: &'a str) -> Account<'a> {
        Account {
            username,
            password,
            premium: false,
        }
    }

    pub fn get_premium_status(&mut self) {
        unimplemented!();
    }

    pub async fn authenticate(&self) -> Result<Response> {
        unimplemented!();
    }
}

#[allow(dead_code)]
pub struct Package {
    name: String,
    parts: Vec<Part>,
    progress: f64,
    size: f64,
}

#[allow(dead_code)]
impl Package {
    fn new(name: String, parts: Vec<Part>, progress: f64, size: f64) -> Self {
        Self {
            name,
            parts,
            progress,
            size,
        }
    }
}

#[allow(dead_code)]
pub struct Part {
    name: String,
    path: Box<Path>,
    progress: f64,
    size: f64,
    state: State,
}

#[allow(dead_code)]
impl Part {
    fn new(name: String, path: Box<Path>, progress: f64, size: f64, state: State) -> Self {
        Self {
            name,
            path,
            progress,
            size,
            state,
        }
    }
}

#[allow(dead_code)]
enum State {
    Online,
    Offline,
}

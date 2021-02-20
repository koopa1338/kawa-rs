// use reqwest::{Response, Result};
use druid::Data;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/* we dont care about accounts and authentication at this point.
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
*/

#[allow(dead_code)]
#[derive(Clone, Data, PartialEq)]
enum State {
    Added,
    Started,
    Finished,
    Extracted,
}

#[derive(Clone, Data)]
pub struct Part {
    name: String,
    path: Arc<PathBuf>,
    #[data(same_fn = "PartialEq::eq")]
    url: String,
    progress: f64,
    size: f64,
    state: State,
}

#[derive(Clone, Data)]
pub struct Package {
    name: String,
    parts: Arc<Vec<Part>>,
    progress: f64,
    size: f64,
    state: State,
}

#[derive(Clone, Data)]
pub struct AppData {
    data: Arc<Vec<Package>>,
}

impl AppData {
    pub fn new() -> AppData {
        // FIXME: temporary for testing
        let part = Part {
            name: String::from("part"),
            path: Arc::new(Path::new("test.txt").to_owned()),
            url: String::from("url"),
            progress: 0.0,
            size: 0.0,
            state: State::Added,
        };

        let package = Package {
            name: String::from("package"),
            parts: Arc::new(vec![part]),
            progress: 0.0,
            size: 0.0,
            state: State::Added,
        };
        AppData {
            data: Arc::new(vec![package]),
        }
    }
}

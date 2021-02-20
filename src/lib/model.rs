use druid::Data;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Clone, Data, PartialEq)]
enum State {
    Added,
    Started,
    Finished,
    Extracted,
}

#[derive(Clone, Data)]
pub struct Account {
    username: String,
    password: String,
    premium: bool,
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
    accounts: Arc<Vec<Account>>,
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

        let acc = Account {
            username: String::from("test"),
            password: String::from("password"),
            premium: false,
        };

        AppData {
            data: Arc::new(vec![package]),
            accounts: Arc::new(vec![acc]),
        }
    }

    /* 
    pub fn new(packages: Vec<Package>, accounts: Vec<Account>) -> Self {
        Self {
            data: Arc::new(packages),
            accounts: Arc::new(accounts),
        }
    }
    */
}

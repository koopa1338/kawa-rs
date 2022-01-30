use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum State {
    Added,
    Started,
    Finished,
    Extracted,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    username: String,
    password: String,
    premium: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Part {
    name: String,
    path: Arc<PathBuf>,
    url: String,
    progress: f64,
    size: f64,
    state: State,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Package {
    name: String,
    parts: Arc<Vec<Part>>,
    progress: f64,
    size: f64,
    state: State,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppData {
    data: Arc<Vec<Package>>,
    accounts: Arc<Vec<Account>>,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            data: Arc::new(Vec::new()),
            accounts: Arc::new(Vec::new()),
        }
    }
}

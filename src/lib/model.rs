use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum State {
    Added,
    Started,
    Finished,
    Extracted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub premium: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Part {
    pub name: String,
    pub url: String,
    pub progress: f32,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub parts: Arc<Vec<Part>>,
    pub path: Arc<PathBuf>,
    pub progress: f32,
    pub size: u64,
    pub state: State,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub url_window_open: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            url_window_open: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppData {
    pub packages: Arc<Vec<Package>>,
    pub accounts: Arc<Vec<Account>>,
    pub settings: AppSettings,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            packages: Arc::new(Vec::new()),
            accounts: Arc::new(Vec::new()),
            ..Default::default()
        }
    }
}

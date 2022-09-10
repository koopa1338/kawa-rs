use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub premium: bool,
    pub provider: AccountProvider,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AccountProvider {
    Ddownload,
    UploadedTo,
}

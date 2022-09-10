use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub provider: AccountProvider,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AccountProvider {
    Ddownload { premium: bool },
    UploadedTo { premium: bool },
}

impl Account {
    pub fn new(username: String, password: String, provider: AccountProvider) -> Self {
        Self {
            username,
            password,
            provider,
        }
    }

    pub fn auth_with_account_data(&mut self) {
        match self.provider {
            AccountProvider::Ddownload { premium } => todo!(),
            AccountProvider::UploadedTo { premium } => todo!(),
        }
    }
}

use super::models::account::{Account, AccountProvider::*};

trait AccountAuth {
    fn auth_with_account_data(&mut self);
}

impl AccountAuth for Account {
    fn auth_with_account_data(&mut self) {
        match self.provider {
            Ddownload => todo!(),
            UploadedTo => todo!(),
        }
    }
}

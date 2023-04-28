pub struct User {
    account_id: String,
    username: String,
}

impl plugins_core::User for User {
    fn id(&self) -> &str {
        &self.account_id
    }

    fn name(&self) -> &str {
        &self.username
    }

    fn region(&self) -> Option<&str> {
        None
    }

    fn discriminator(&self) -> Option<&str> {
        None
    }
}

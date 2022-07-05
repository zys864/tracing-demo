use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Validate)]
pub struct User {
    name: String,
    #[validate(email)]
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }
}

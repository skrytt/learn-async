use inquire::{
    InquireError, Select
};
use std::fmt::Display;

pub struct UserOption {
    name: String,
}

impl UserOption {
    pub fn new(name: String) -> Self {
        Self {name}
    }
}

impl Display for UserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
pub fn get_test_type() -> Result<UserOption, InquireError> {
    let options = vec![
        UserOption::new("HTTP".to_string()),
        UserOption::new("TCP".to_string()),
        UserOption::new("DNS".to_string()),
        UserOption::new("CloudFormation".to_string()),
    ];

    let result = Select::new("Select Test Mode:", options).prompt()?;
    Ok(result)
}
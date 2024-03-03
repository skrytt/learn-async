mod user_option;

use inquire::{
    InquireError, Select
};
use user_option::UserOption;

fn get_user_decision() -> Result<UserOption, InquireError> {
    let options = vec![
        UserOption::new("Follow the road".to_string()),
        UserOption::new("Take a nap".to_string()),
    ];

    let result = Select::new("What would you like to do?", options).prompt()?;

    Ok(result)
}

fn main() {
  get_user_decision().unwrap();
}

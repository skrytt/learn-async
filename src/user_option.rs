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

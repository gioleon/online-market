use std::fmt;


#[derive(Debug)]
pub struct NoIdProvided {
    details: String
}

impl NoIdProvided {
    pub fn new(message: &str) -> Self {
        NoIdProvided {
            details: message.to_string()
        }
    }
}

impl std::fmt::Display for NoIdProvided {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for NoIdProvided {
    fn description(&self) -> &str {
        &self.details
    }
}
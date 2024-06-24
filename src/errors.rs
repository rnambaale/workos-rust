use std::fmt;

#[derive(Debug)]
pub enum WorkOsError {
    ConfigurationError(String),
}

impl fmt::Display for WorkOsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            WorkOsError::ConfigurationError(ref err) => write!(f, "Invalid input: {}", err),
        }
    }
}

impl std::error::Error for WorkOsError {}

use std::error::Error as StdError;
use std::fmt::Display;

#[derive(Debug)]
pub struct ErrorDetail {
    pub title: String,
    pub detail: String,
}

impl ErrorDetail {
    pub fn new(title: &str, detail: &str) -> Self {
        Self {
            title: title.to_string(),
            detail: detail.to_string(),
        }
    }
}

impl Display for ErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.title, self.detail)
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidValueError(ErrorDetail),
    GitCommandError(ErrorDetail),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl StdError for Error {}

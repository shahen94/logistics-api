use std::{fmt::Display};

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub url: Option<String>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        let url = err.url().map(|url| url.to_string());
        Error {
            message: err.to_string(),
            url,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

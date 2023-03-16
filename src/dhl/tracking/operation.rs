#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub url: Option<String>,
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

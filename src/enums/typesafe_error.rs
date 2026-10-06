use std::fmt;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum TypesafeError {
    Http(reqwest::Error),
    Api {status: StatusCode, body: String},
    Validation(String),
}
impl fmt::Display for TypesafeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {e}"),
            Self::Api { status, body } => write!(f, "API error: {status}: {body}"),
            Self::Validation(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}
impl From<reqwest::Error> for TypesafeError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}
impl std::error::Error for TypesafeError {}
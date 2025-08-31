use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TankilleError {
    Reqwest(reqwest::Error),
    NotAuthenticated,
}

impl fmt::Display for TankilleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TankilleError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            TankilleError::NotAuthenticated => write!(f, "Not authenticated"),
        }
    }
}

impl Error for TankilleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TankilleError::Reqwest(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for TankilleError {
    fn from(err: reqwest::Error) -> Self {
        TankilleError::Reqwest(err)
    }
}

type Result<T> = std::result::Result<T, TankilleError>;

pub mod client;

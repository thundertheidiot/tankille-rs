use serde::Deserialize;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: u8,
    pub message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, message: {}", self.code, self.message)
    }
}

#[derive(Debug)]
pub enum TankilleError {
    Reqwest(reqwest::Error),
    ApiError(ApiError),
    NotAuthenticated,
}

impl fmt::Display for TankilleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TankilleError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            TankilleError::ApiError(e) => write!(f, "Api error response: {}", e),
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

impl From<ApiError> for TankilleError {
    fn from(err: ApiError) -> Self {
        TankilleError::ApiError(err)
    }
}

type Result<T> = std::result::Result<T, TankilleError>;

pub mod client;

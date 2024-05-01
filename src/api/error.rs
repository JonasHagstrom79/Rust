use anyhow::Error;
use std::fmt;

pub struct AppError(pub Error);

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AppError {}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        AppError(anyhow::anyhow!(err.to_string()))
    }
}

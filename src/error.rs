#[derive(Debug)]
#[allow(dead_code)]
pub enum ServerError {
    IoError(std::io::Error),
    HeaderFormat(String),
}
use ServerError::*;

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        IoError(e)
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError(e) => write!(f, "io error: {}", e),
            HeaderFormat(s) => write!(f, "{}: wrong header format", s),
        }
    }
}

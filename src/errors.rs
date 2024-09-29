use std::fmt;
use atom_syndication;
use rss;
use tokio;
use xml; 

#[derive(Debug)]
pub enum AppError {
    NetworkError(reqwest::Error),
    ParseError(String),
    DateParseError(chrono::ParseError),
    IoError(std::io::Error),
    UnknownFeedType(String),
    AsyncRuntimeError(tokio::task::JoinError),
    XmlError(xml::reader::Error), 
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NetworkError(e) => write!(f, "Network error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),
            AppError::DateParseError(e) => write!(f, "Date parse error: {}", e),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::UnknownFeedType(t) => write!(f, "Unknown feed type: {}", t),
            AppError::AsyncRuntimeError(e) => write!(f, "Async runtime error: {}", e),
            AppError::XmlError(e) => write!(f, "XML error: {}", e), 
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(err)
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        AppError::DateParseError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl From<atom_syndication::Error> for AppError {
    fn from(err: atom_syndication::Error) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl From<rss::Error> for AppError {
    fn from(err: rss::Error) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        AppError::AsyncRuntimeError(err)
    }
}

impl From<xml::reader::Error> for AppError {
    fn from(err: xml::reader::Error) -> Self {
        AppError::XmlError(err)
    }
}
use std::io;
use std::fmt;

use reqwest;
use serde_json;

use client::Status;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AuthyError {
    BadRequest(Status),            // 400
    UnauthorizedKey(Status),       // 401
    UserNotFound(Status),          // 404
    TooManyRequests(Status),       // 429
    ServiceUnavailable,            // 503

    IoError(String),
    JsonParseError(String),
    RequestError(String),
    InvalidServerResponse,
}


impl fmt::Display for AuthyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AuthyError::*;

        match *self {
            BadRequest(ref s) => write!(f, "Bad request: {}", s.message),
            UnauthorizedKey(ref s) => write!(f, "Unauthorized API Key: {}", s.message),
            UserNotFound(ref s) => write!(f, "User not found: {}", s.message),
            TooManyRequests(ref s) => write!(f, "Too many requests: {}", s.message),
            ServiceUnavailable => write!(f, "Service is unavailable"),
            IoError(ref s) => write!(f, "IO Error: {}", s),
            JsonParseError(ref s) => write!(f, "Json parsing error: {}", s),
            RequestError(ref s) => write!(f, "Request error: {}", s),
            InvalidServerResponse => write!(f, "Server returned an invalid response"),
        }
    }
}

impl From<reqwest::Error> for AuthyError {
    fn from(e: reqwest::Error) -> Self {
        AuthyError::RequestError(e.to_string())
    }
}

impl From<serde_json::Error> for AuthyError {
    fn from(e: serde_json::Error) -> Self {
        AuthyError::JsonParseError(e.to_string())
    }
}

impl From<io::Error> for AuthyError {
    fn from(e: io::Error) -> Self {
        AuthyError::IoError(e.to_string())
    }
}

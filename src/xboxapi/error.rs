// Copyright (c) 2016 P.Y. Laligand

use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

use hyper::error::Error as NetworkError;
use serde_json::error::Error as JsonError;

/// Standard error for the xboxapi module.
#[derive(Debug)]
pub enum XboxError {
    Network(NetworkError),
    Json(JsonError),
    Io(IoError),
    Described(String),
}

impl XboxError {
    pub fn new(message: String) -> XboxError {
        XboxError::Described(message)
    }

    pub fn from(message: &str) -> XboxError {
        XboxError::new(message.to_owned())
    }
}

impl Error for XboxError {
    fn description(&self) -> &str {
        match *self {
            XboxError::Network(ref e) => e.description(),
            XboxError::Json(ref e) => e.description(),
            XboxError::Io(ref e) => e.description(),
            XboxError::Described(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            XboxError::Network(ref e) => Some(e),
            XboxError::Json(ref e) => Some(e),
            XboxError::Io(ref e) => Some(e),
            XboxError::Described(_) => None
        }
    }
}

impl fmt::Display for XboxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XboxError::Network(ref e) => write!(f, "Network error: {}", e),
            XboxError::Json(ref e) => write!(f, "JSON error: {}", e),
            XboxError::Io(ref e) => write!(f, "IO error: {}", e),
            XboxError::Described(ref s) => write!(f, "{}", s)
        }
    }
}

impl From<NetworkError> for XboxError {
    fn from(err: NetworkError) -> XboxError {
        XboxError::Network(err)
    }
}

impl From<JsonError> for XboxError {
    fn from(err: JsonError) -> XboxError {
        XboxError::Json(err)
    }
}

impl From<IoError> for XboxError {
    fn from(err: IoError) -> XboxError {
        XboxError::Io(err)
    }
}

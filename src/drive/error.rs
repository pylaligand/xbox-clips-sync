// Copyright (c) 2016 P.Y. Laligand

use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

use google_drive3::Error as GoogleError;
use serde_json::error::Error as JsonError;

/// Standard error for the drive module.
#[derive(Debug)]
pub enum DriveError {
    Io(IoError),
    Json(JsonError),
    Drive(GoogleError),
    Described(String),
}

impl Error for DriveError {
    fn description(&self) -> &str {
        match *self {
            DriveError::Io(ref e) => e.description(),
            DriveError::Json(ref e) => e.description(),
            DriveError::Drive(ref e) => e.description(),
            DriveError::Described(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DriveError::Io(ref e) => Some(e),
            DriveError::Json(ref e) => Some(e),
            DriveError::Drive(ref e) => Some(e),
            DriveError::Described(_) => None
        }
    }
}

impl fmt::Display for DriveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DriveError::Io(ref e) => write!(f, "IO error: {}", e),
            DriveError::Json(ref e) => write!(f, "JSON error: {}", e),
            DriveError::Drive(ref e) => write!(f, "Google Drive error: {}", e),
            DriveError::Described(ref s) => write!(f, "{}", s)
        }
    }
}

impl From<IoError> for DriveError {
    fn from(err: IoError) -> DriveError {
        DriveError::Io(err)
    }
}

impl From<JsonError> for DriveError {
    fn from(err: JsonError) -> DriveError {
        DriveError::Json(err)
    }
}

impl From<GoogleError> for DriveError {
    fn from(err: GoogleError) -> DriveError {
        DriveError::Drive(err)
    }
}

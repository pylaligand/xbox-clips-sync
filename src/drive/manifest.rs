// Copyright (c) 2016 P.Y. Laligand

use chrono::{DateTime, UTC};
use std::fmt;

/// Contents of the manifest file on Drive.
#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub timestamp: DateTime<UTC>,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest { timestamp: UTC::now() }
    }
}

impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Manifest[{}]", self.timestamp)
    }
}

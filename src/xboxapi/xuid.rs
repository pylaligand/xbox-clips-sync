// Copyright (c) 2016 P.Y. Laligand

use std::fmt;

/// Identifies a user with the xboxapi.com service.
pub struct Xuid (pub u64);

impl fmt::Display for Xuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Xuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

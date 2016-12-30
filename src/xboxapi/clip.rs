// Copyright (c) 2016 P.Y. Laligand

use std::fmt;

/// An Xbox video clip.
pub struct Clip {
    pub id: String,
    pub url: String,
    pub date: String,
}

impl fmt::Display for Clip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} / {}", self.id, self.date)
    }
}

impl fmt::Debug for Clip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

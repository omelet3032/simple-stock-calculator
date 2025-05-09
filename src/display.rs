use std::fmt;

use crate::types::{Leverage, Position};

impl fmt::Display for Leverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Leverage::Daily2x => write!(f, "2X"),
            Leverage::Daily3x => write!(f, "3X"),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Long => write!(f, "Long"),
            Position::Short => write!(f, "Short"),
        }
    }
}
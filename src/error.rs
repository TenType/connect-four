use crate::WIDTH;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
    ColumnFull,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match *self {
            OutOfBounds => write!(f, "Column must be in between 0 to {}", WIDTH - 1),
            ColumnFull => write!(f, "Cannot play into a full column"),
        }
    }
}

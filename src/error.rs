use crate::WIDTH;
use std::fmt;

/// A list of possible errors in a game.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// A move could not be played because the column was out of bounds.
    OutOfBounds,
    /// A move could not be played because the column was full.
    ColumnFull,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match *self {
            OutOfBounds => write!(
                f,
                "column must be in the range 0..={} (inclusive)",
                WIDTH - 1
            ),
            ColumnFull => write!(f, "cannot play into a full column"),
        }
    }
}

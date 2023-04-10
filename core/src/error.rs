use std::fmt;

/// A list of possible errors in a game.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The specified column was full.
    ColumnFull,
    /// The specified column was out of bounds or could not be parsed.
    InvalidColumn,
    /// The game is over and no more moves can be played.
    GameOver,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match *self {
            ColumnFull => write!(f, "cannot play into a full column"),
            InvalidColumn => write!(f, "column is out of bounds or cannot be parsed"),
            GameOver => write!(f, "moves cannot be played after the game ends"),
        }
    }
}

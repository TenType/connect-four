use std::fmt;

/// A list of possible errors caused by playing a move in a game.
#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    /// The specified column was full.
    ColumnFull,
    /// The specified column was out of bounds or could not be parsed.
    InvalidColumn,
    /// The game is over and no more moves can be played.
    GameOver,
}

impl std::error::Error for MoveError {}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MoveError::*;
        match *self {
            ColumnFull => write!(f, "cannot play into a full column"),
            InvalidColumn => write!(f, "column is out of bounds or cannot be parsed"),
            GameOver => write!(f, "moves cannot be played after the game ends"),
        }
    }
}

//! A library for playing, solving, and analyzing the game of [Connect Four](https://en.wikipedia.org/wiki/Connect_Four).

pub mod bitboard;

mod error;
pub use error::Error;

pub mod game;
pub use game::{Game, Status};

pub mod solver;
pub use solver::Solver;

/// The number of rows in a standard board.
pub const HEIGHT: usize = 6;

/// The number of columns in a standard board.
pub const WIDTH: usize = 7;

/// The number of players in a game.
pub const NUM_PLAYERS: usize = 2;

/// Represents a single player.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    P1,
    P2,
}

use std::{fmt, ops::Not};

impl Not for Player {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        if self == Self::P1 {
            Self::P2
        } else {
            Self::P1
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Self::P1 {
            write!(f, "X")
        } else {
            write!(f, "O")
        }
    }
}

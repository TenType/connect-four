//! A library for playing, solving, and analyzing the game of [Connect Four](https://en.wikipedia.org/wiki/Connect_Four).

mod bitboard;

mod board;
use board::*;

mod error;
pub use error::*;

mod game;
pub use game::*;

mod engine;
pub use engine::*;

mod cache;
pub use cache::*;

/// The number of rows in a standard board.
pub const HEIGHT: u8 = 6;

/// The number of columns in a standard board.
pub const WIDTH: u8 = 7;

/// The number of tiles in a standard board.
pub const AREA: u8 = WIDTH * HEIGHT;

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
            if cfg!(test) {
                write!(f, "X")
            } else {
                write!(f, "\x1b[1;31mX\x1b[0m")
            }
        } else if cfg!(test) {
            write!(f, "O")
        } else {
            write!(f, "\x1b[1;33mO\x1b[0m")
        }
    }
}

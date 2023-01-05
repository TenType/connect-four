pub mod bitboard;

mod error;
pub use error::*;

mod game;
pub use game::*;

mod solver;
pub use solver::*;

pub const HEIGHT: usize = 6;
pub const WIDTH: usize = 7;
pub const NUM_PLAYERS: usize = 2;

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

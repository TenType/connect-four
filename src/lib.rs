mod error;
pub use error::*;

mod game;
pub use game::*;

pub const HEIGHT: usize = 6;
pub const WIDTH: usize = 7;
pub const NUM_PLAYERS: usize = 2;

#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    P1,
    P2,
}

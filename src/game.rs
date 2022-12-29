use crate::{Error, Player, HEIGHT, NUM_PLAYERS, WIDTH};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Draw,
    Ongoing,
    Win(Player),
}

#[derive(Clone)]
pub struct Game {
    boards: [u64; NUM_PLAYERS],
    heights: [u64; WIDTH],
    moves: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            boards: [0, 0],
            heights: core::array::from_fn(|i| (WIDTH * i) as u64),
            moves: 0,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                let index = col * WIDTH + row;
                let piece = if self.get_bit(0, index) != 0 {
                    1
                } else if self.get_bit(1, index) != 0 {
                    2
                } else {
                    0
                };

                write!(f, "{} ", piece)?;
            }
            if row != 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Game {
    #[allow(clippy::unusual_byte_groupings)]
    const TOP_MASK: u64 = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&mut self, col: usize) -> Result<Status, Error> {
        self.can_play(col)?;

        self.unchecked_play(col);

        Ok(self.status())
    }

    pub(crate) fn unchecked_play(&mut self, col: usize) {
        self.boards[self.moves % NUM_PLAYERS] |= 1 << self.heights[col];
        self.heights[col] += 1;
        self.moves += 1;
    }

    pub fn play_moves(&mut self, moves: &[usize]) -> Result<Status, Error> {
        if let Some((last, elements)) = moves.split_last() {
            for col in elements {
                self.play(*col)?;
            }
            let status = self.play(*last)?;
            Ok(status)
        } else {
            panic!("Cannot play moves from an empty slice");
        }
    }

    pub fn can_play(&self, col: usize) -> Result<(), Error> {
        if !self.is_inside(col) {
            Err(Error::OutOfBounds)
        } else if !self.is_unfilled(col) {
            Err(Error::ColumnFull)
        } else {
            Ok(())
        }
    }

    fn is_inside(&self, col: usize) -> bool {
        col < WIDTH
    }

    pub(crate) fn is_unfilled(&self, col: usize) -> bool {
        let new_board = self.boards[self.moves % NUM_PLAYERS] | (1 << self.heights[col]);
        new_board & Self::TOP_MASK == 0
    }

    pub fn status(&self) -> Status {
        if self.is_draw() {
            Status::Draw
        } else if self.has_won() {
            let player = if (self.moves + 1) % NUM_PLAYERS == 0 {
                Player::P1
            } else {
                Player::P2
            };
            Status::Win(player)
        } else {
            Status::Ongoing
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.is_draw() || self.has_won()
    }

    pub(crate) fn is_draw(&self) -> bool {
        self.moves >= WIDTH * HEIGHT
    }

    pub fn has_won(&self) -> bool {
        self.check_win(self.boards[(self.moves + 1) % NUM_PLAYERS])
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        self.check_win(self.boards[self.moves % NUM_PLAYERS] | (1 << self.heights[col]))
    }

    fn check_win(&self, board: u64) -> bool {
        // Descending diagonal \
        let x = board & (board >> HEIGHT);
        if x & (x >> (2 * HEIGHT)) != 0 {
            return true;
        }

        // Horizontal -
        let x = board & (board >> (HEIGHT + 1));
        if x & (x >> (2 * (HEIGHT + 1))) != 0 {
            return true;
        }

        // Ascending diagonal /
        let x = board & (board >> (HEIGHT + 2));
        if x & (x >> (2 * (HEIGHT + 2))) != 0 {
            return true;
        }

        // Vertical |
        let x = board & (board >> 1);
        x & (x >> 2) != 0
    }

    pub fn moves(&self) -> usize {
        self.moves
    }

    pub fn turn(&self) -> Player {
        if self.moves % NUM_PLAYERS == 0 {
            Player::P1
        } else {
            Player::P2
        }
    }

    fn get_bit(&self, player: usize, index: usize) -> u64 {
        self.boards[player] & (1 << index)
    }
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.boards, [0, 0]);
        assert_eq!(game.heights, [0, 7, 14, 21, 28, 35, 42]);
        assert_eq!(game.moves, 0);
    }

    #[test]
    fn play_one() -> Result<(), Error> {
        let mut game = Game::new();
        game.play(3)?;
        assert_eq!(
            game.boards,
            [
                0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000,
                0
            ]
        );
        assert_eq!(game.heights, [0, 7, 14, 22, 28, 35, 42]);
        assert_eq!(game.moves, 1);
        Ok(())
    }

    #[test]
    fn play_multiple() -> Result<(), Error> {
        let mut game = Game::new();
        game.play_moves(&[3, 3, 3, 3])?;
        assert_eq!(
            game.boards,
            [
                0b_0000000_0000000_0000000_0000101_0000000_0000000_0000000,
                0b_0000000_0000000_0000000_0001010_0000000_0000000_0000000,
            ]
        );
        assert_eq!(game.heights, [0, 7, 14, 25, 28, 35, 42]);
        assert_eq!(game.moves, 4);
        Ok(())
    }

    #[test]
    fn out_of_bounds() {
        let mut game = Game::new();
        let err = game.play(7).unwrap_err();
        assert_eq!(err, Error::OutOfBounds);
    }

    #[test]
    fn full_column() {
        let mut game = Game::new();
        let err = game.play_moves(&[0, 0, 0, 0, 0, 0, 0]).unwrap_err();
        assert_eq!(err, Error::ColumnFull);
    }

    #[test]
    fn no_win_overflow() -> Result<(), Error> {
        let mut game = Game::new();
        assert!(!game.is_game_over());
        assert_eq!(game.status(), Status::Ongoing);

        let status = game.play_moves(&[0, 1, 0, 1, 0, 1, 1, 2, 1, 2, 1])?;
        assert!(!game.is_game_over());
        assert_eq!(status, Status::Ongoing);
        Ok(())
    }

    #[test]
    fn horizontal_win() -> Result<(), Error> {
        let mut game = Game::new();
        let status = game.play_moves(&[0, 0, 1, 1, 2, 2])?;
        assert!(!game.is_game_over());
        assert_eq!(status, Status::Ongoing);

        let status = game.play(3)?;
        assert!(game.is_game_over());
        assert_eq!(status, Status::Win(Player::P1));
        Ok(())
    }

    #[test]
    fn vertical_win() -> Result<(), Error> {
        let mut game = Game::new();
        let status = game.play_moves(&[0, 1, 0, 1, 0, 1])?;
        assert!(!game.is_game_over());
        assert_eq!(status, Status::Ongoing);

        let status = game.play(0)?;
        assert!(game.is_game_over());
        assert_eq!(status, Status::Win(Player::P1));
        Ok(())
    }

    #[test]
    fn asc_diagonal_win() -> Result<(), Error> {
        let mut game = Game::new();
        let status = game.play_moves(&[3, 0, 1, 1, 2, 3, 2, 2, 3])?;
        assert!(!game.is_game_over());
        assert_eq!(status, Status::Ongoing);

        let status = game.play(3)?;
        assert!(game.is_game_over());
        assert_eq!(status, Status::Win(Player::P2));
        Ok(())
    }

    #[test]
    fn desc_diagonal_win() -> Result<(), Error> {
        let mut game = Game::new();
        let status = game.play_moves(&[3, 6, 5, 5, 4, 3, 4, 4, 3])?;
        assert!(!game.is_game_over());
        assert_eq!(status, Status::Ongoing);

        let status = game.play(3)?;
        assert!(game.is_game_over());
        assert_eq!(status, Status::Win(Player::P2));
        Ok(())
    }

    #[test]
    fn draw() -> Result<(), Error> {
        let mut game = Game::new();
        let status = game.play_moves(&[
            0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 4, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4,
            4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6,
        ])?;
        assert!(!game.is_game_over());
        assert_eq!(status, Status::Ongoing);

        let status = game.play(6)?;
        assert!(game.is_game_over());
        assert_eq!(status, Status::Draw);
        Ok(())
    }
}

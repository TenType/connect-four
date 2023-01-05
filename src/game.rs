use crate::{
    bitboard::{self, Bitboard},
    Error, Player, HEIGHT, NUM_PLAYERS, WIDTH,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Draw,
    Ongoing,
    Win(Player),
}

#[derive(Clone, Default)]
pub struct Game {
    player_board: Bitboard,
    pieces_board: Bitboard,
    moves: usize,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let turn = self.turn();
        let current_player = self.player_board;
        let opponent = self.player_board ^ self.pieces_board;

        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                let index = col * WIDTH + row;
                if current_player & (1 << index) != 0 {
                    write!(f, "{turn}")?;
                } else if opponent & (1 << index) != 0 {
                    write!(f, "{}", !turn)?;
                } else {
                    write!(f, ".")?;
                };

                write!(f, " ")?;
            }
            if row != 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&mut self, col: usize) -> Result<Status, Error> {
        self.can_play(col)?;

        self.unchecked_play(col);

        Ok(self.status())
    }

    pub(crate) fn unchecked_play(&mut self, col: usize) {
        self.player_board ^= self.pieces_board;
        self.pieces_board |= self.pieces_board + bitboard::bottom_piece_mask(col);
        self.moves += 1;
    }

    pub fn play_moves(&mut self, moves: &[usize]) -> Result<Status, Error> {
        let (last, elements) = moves.split_last().expect("slice should not be empty");

        for col in elements {
            self.play(*col)?;
        }
        let status = self.play(*last)?;
        Ok(status)
    }

    #[cfg(test)]
    pub(crate) fn unchecked_play_moves(&mut self, moves: &[usize]) {
        for col in moves {
            self.unchecked_play(*col);
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
        (self.pieces_board & bitboard::top_piece_mask(col)) == 0
    }

    pub fn status(&self) -> Status {
        if self.is_draw() {
            Status::Draw
        } else if let Some(player) = self.winner() {
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

    pub fn winner(&self) -> Option<Player> {
        if self.check_win(self.player_board) {
            Some(self.turn())
        } else if self.check_win(self.player_board ^ self.pieces_board) {
            Some(!self.turn())
        } else {
            None
        }
    }

    pub fn has_won(&self) -> bool {
        self.winner().is_some()
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        let board = self.player_board
            | ((self.pieces_board + bitboard::bottom_piece_mask(col)) & bitboard::column_mask(col));
        self.check_win(board)
    }

    fn check_win(&self, board: Bitboard) -> bool {
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

    pub(crate) fn key(&self) -> Bitboard {
        self.player_board + self.pieces_board
    }
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.player_board, 0);
        assert_eq!(game.pieces_board, 0);
        assert_eq!(game.moves, 0);
    }

    #[test]
    fn play_one() -> Result<(), Error> {
        let mut game = Game::new();
        game.play(3)?;
        assert_eq!(
            game.player_board,
            0b_0000000_0000000_0000000_0000000_0000000_0000000_0000000
        );
        assert_eq!(
            game.pieces_board,
            0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000
        );
        assert_eq!(game.moves, 1);
        Ok(())
    }

    #[test]
    fn play_multiple() -> Result<(), Error> {
        let mut game = Game::new();
        game.play_moves(&[3, 3, 3, 3])?;
        assert_eq!(
            game.player_board,
            0b_0000000_0000000_0000000_0000101_0000000_0000000_0000000
        );
        assert_eq!(
            game.pieces_board,
            0b_0000000_0000000_0000000_0001111_0000000_0000000_0000000
        );
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

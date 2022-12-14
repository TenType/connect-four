//! Functionality for creating and playing the game of Connect Four.

use crate::{
    bitboard::{self, Bitboard},
    Error, Player, HEIGHT, NUM_PLAYERS, WIDTH,
};
use std::fmt;

/// Represents the state of a game.
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    /// The game has ended in a draw.
    Draw,
    /// The game is still ongoing.
    Ongoing,
    /// The game has ended with a winner represented by [`Player`].
    Win(Player),
}

/// Represents a Connect Four game.
#[derive(Clone, Default)]
pub struct Game {
    /// A bitboard representing the pieces belonging to the current player.
    player_board: Bitboard,
    /// A bitboard representing all the pieces played in the game.
    pieces_board: Bitboard,
    /// The number of moves made in the game.
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
    /// Creates a new game with an empty board.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    /// let game = Game::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Plays the current player's piece in the given 0-indexed column, returning the new status of the game.
    ///
    /// # Errors
    /// Returns an error if the move cannot be played:
    /// * [`Error::OutOfBounds`] if the column is out of bounds
    /// * [`Error::ColumnFull`] if the column is full
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// let result = game.play(3);
    /// assert!(result.is_ok());
    ///
    /// let result = game.play(7); // out of bounds
    /// assert!(result.is_err());
    /// ```
    pub fn play(&mut self, col: usize) -> Result<Status, Error> {
        self.can_play(col)?;

        self.unchecked_play(col);

        Ok(self.status())
    }

    /// Plays the current player's piece in the given 0-indexed column, without checking if the move can be played.
    ///
    /// # Warning
    /// Playing into a column that is out of bounds or full may result in unexpected behavior.
    ///
    /// # Panics
    /// Panics if `column` overflows a bitboard in debug mode.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// game.unchecked_play(3);
    ///
    /// // game.unchecked_play(7);
    /// // ^^ out of bounds: unexpected behavior!
    /// ```
    pub fn unchecked_play(&mut self, col: usize) {
        self.play_board(self.pieces_board + bitboard::bottom_piece_mask(col));
    }

    /// Plays the current player's piece given a move represented as a bitboard.
    pub(crate) fn play_board(&mut self, move_board: Bitboard) {
        self.player_board ^= self.pieces_board;
        self.pieces_board |= move_board;
        self.moves += 1;
    }

    /// Plays a sequence of moves from a slice of 0-indexed columns, returning the new status of the game.
    ///
    /// # Errors
    /// Returns an error at the first move that cannot be played:
    /// * [`Error::OutOfBounds`] if the column is out of bounds
    /// * [`Error::ColumnFull`] if the column is full
    ///
    /// # Panics
    /// Panics if `moves` is empty.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// let result = game.play_moves(&[3, 2, 3]);
    /// assert!(result.is_ok());
    ///
    /// let result = game.play_moves(&[3, 3, 3, 3, 3]); // overflowing column
    /// assert!(result.is_err());
    /// ```
    pub fn play_moves(&mut self, moves: &[usize]) -> Result<Status, Error> {
        let (last, elements) = moves.split_last().expect("slice should not be empty");

        for col in elements {
            self.play(*col)?;
        }
        let status = self.play(*last)?;
        Ok(status)
    }

    /// Plays a sequence of moves from a slice of 0-indexed columns, without checking if moves can be played.
    ///
    /// # Warning
    /// A slice that plays into a column that is out of bounds or full may result in unexpected behavior.
    ///
    /// # Panics
    /// Panics if any item in `moves` overflows a bitboard in debug mode.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// game.unchecked_play_moves(&[3, 2, 3]);
    ///
    /// // game.unchecked_play_moves(&[3, 3, 3, 3, 3]);
    /// // ^^ overflowing column: unexpected behavior!
    /// ```
    pub fn unchecked_play_moves(&mut self, moves: &[usize]) {
        for col in moves {
            self.unchecked_play(*col);
        }
    }

    /// Returns `Ok(())` if the given 0-indexed column can be played in the game board.
    ///
    /// # Errors
    /// Returns an error if the move cannot be played:
    /// * [`Error::OutOfBounds`] if the column is out of bounds
    /// * [`Error::ColumnFull`] if the column is full
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert!(game.can_play(3).is_ok());
    ///
    /// game.play(3)?;
    /// assert!(game.can_play(3).is_ok());
    ///
    /// game.play_moves(&[3, 3, 3, 3, 3])?;
    /// assert!(game.can_play(3).is_err()); // column is full
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn can_play(&self, col: usize) -> Result<(), Error> {
        if !self.is_inside(col) {
            Err(Error::OutOfBounds)
        } else if !self.is_unfilled(col) {
            Err(Error::ColumnFull)
        } else {
            Ok(())
        }
    }

    /// Checks if the given 0-indexed column is inside the game board.
    fn is_inside(&self, col: usize) -> bool {
        // No need to check for 0 < col because col is unsigned
        col < WIDTH
    }

    /// Checks if the given 0-indexed column is not full, assuming that `column` is inside the game board.
    pub(crate) fn is_unfilled(&self, col: usize) -> bool {
        (self.pieces_board & bitboard::top_piece_mask(col)) == 0
    }

    /// Returns a [`Status`] representing the current state of the game.
    /// * [`Ongoing`](Status::Ongoing) if the game is still ongoing
    /// * [`Draw`](Status::Draw) if the game ended with a draw
    /// * [`Win(player)`](Status::Win) if the game ended with a winner, represented by a [`Player`]
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Player, Status};
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.status(), Status::Ongoing);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.status(), Status::Ongoing);
    ///
    /// game.play_moves(&[2, 3, 2, 3, 2, 3])?;
    /// assert_eq!(game.status(), Status::Win(Player::P1));
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    ///
    /// Note that [`self.play()`] calls this method and also returns the status:
    /// ```
    /// use connect_four_engine::{Game, Status};
    ///
    /// let mut game = Game::new();
    /// let status = game.play(3)?;
    /// assert_eq!(status, Status::Ongoing);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn status(&self) -> Status {
        if self.is_draw() {
            Status::Draw
        } else if let Some(player) = self.winner() {
            Status::Win(player)
        } else {
            Status::Ongoing
        }
    }

    /// Checks if the game is over.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert!(!game.is_game_over());
    ///
    /// game.play(3)?;
    /// assert!(!game.is_game_over());
    ///
    /// game.play_moves(&[2, 3, 2, 3, 2, 3])?;
    /// assert!(game.is_game_over());
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn is_game_over(&self) -> bool {
        self.is_draw() || self.has_won()
    }

    /// Checks if the game is a draw and no more moves can be played.
    pub(crate) fn is_draw(&self) -> bool {
        self.moves >= WIDTH * HEIGHT
    }

    /// Returns the winner of the game or [`None`] if the game is a draw or still ongoing.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert!(game.winner().is_none());
    ///
    /// game.play(3)?;
    /// assert!(game.winner().is_none());
    ///
    /// game.play_moves(&[2, 3, 2, 3, 2, 3])?;
    /// assert!(game.winner().is_some());
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn winner(&self) -> Option<Player> {
        if self.check_win(self.player_board) {
            Some(self.turn())
        } else if self.check_win(self.player_board ^ self.pieces_board) {
            Some(!self.turn())
        } else {
            None
        }
    }

    /// Checks if the game has ended with a winner.
    fn has_won(&self) -> bool {
        self.winner().is_some()
    }

    /// Checks if a given bitboard has a line of four `1`s.
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

    /// Returns a bitboard of the playable moves that do not give the opponent an immediate win.
    pub(crate) fn possible_non_losing_moves(&self) -> Bitboard {
        let mut possible_moves = self.possible_moves();
        let opponent_win = self.opponent_winning_moves();
        let forced_moves = possible_moves & opponent_win;

        if forced_moves != 0 {
            if forced_moves & (forced_moves - 1) != 0 {
                return 0;
            }
            possible_moves = forced_moves;
        }

        possible_moves & !(opponent_win >> 1)
    }

    /// Returns a bitboard of available moves.
    fn possible_moves(&self) -> Bitboard {
        (self.pieces_board + bitboard::BOTTOM_ROW_MASK) & bitboard::FULL_BOARD_MASK
    }

    /// Returns the number of winning moves the current player has after playing a given move (represented as a bitboard).
    pub(crate) fn num_of_winning_moves_after_play(&self, move_board: Bitboard) -> u32 {
        self.winning_moves(self.player_board | move_board)
            .count_ones()
    }

    /// Returns a bitboard of the opponent's winning moves.
    fn opponent_winning_moves(&self) -> Bitboard {
        self.winning_moves(self.player_board ^ self.pieces_board)
    }

    /// Finds the winning moves of a bitboard, returning the tiles as a new bitboard.
    fn winning_moves(&self, board: Bitboard) -> Bitboard {
        // Vertical |
        let mut x = (board << 1) & (board << 2) & (board << 3);

        // Horizontal -
        let mut y = (board << (HEIGHT + 1)) & (board << (2 * (HEIGHT + 1)));
        x |= y & (board << (3 * (HEIGHT + 1)));
        x |= y & (board >> (HEIGHT + 1));

        y = (board >> (HEIGHT + 1)) & (board >> (2 * (HEIGHT + 1)));
        x |= y & (board >> (3 * (HEIGHT + 1)));
        x |= y & (board << (HEIGHT + 1));

        // Ascending diagonal /
        y = (board << HEIGHT) & (board << (2 * HEIGHT));
        x |= y & (board << (3 * (HEIGHT)));
        x |= y & (board >> (HEIGHT));

        y = (board >> (HEIGHT)) & (board >> (2 * HEIGHT));
        x |= y & (board >> (3 * (HEIGHT)));
        x |= y & (board << (HEIGHT));

        // Descending diagonal \
        y = (board << (HEIGHT + 2)) & (board << (2 * (HEIGHT + 2)));
        x |= y & (board << (3 * (HEIGHT + 2)));
        x |= y & (board >> (HEIGHT + 2));

        y = (board >> (HEIGHT + 2)) & (board >> (2 * (HEIGHT + 2)));
        x |= y & (board >> (3 * (HEIGHT + 2)));
        x |= y & (board << (HEIGHT + 2));

        x & (self.pieces_board ^ bitboard::FULL_BOARD_MASK)
    }

    /// Returns the number of moves made in the game.
    ///
    /// # Examples
    ///
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.moves(), 0);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.moves(), 1);
    ///
    /// game.play_moves(&[0, 4, 6, 3])?;
    /// assert_eq!(game.moves(), 5);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn moves(&self) -> usize {
        self.moves
    }

    /// Returns the [`Player`] whose turn it currently is.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Player};
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.turn(), Player::P1);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.turn(), Player::P2);
    ///
    /// game.play(1)?;
    /// assert_eq!(game.turn(), Player::P1);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn turn(&self) -> Player {
        if self.moves % NUM_PLAYERS == 0 {
            Player::P1
        } else {
            Player::P2
        }
    }

    /// Returns a unique key for the current game state for use in the transposition table.
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

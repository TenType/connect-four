//! Functionality for creating and playing the game of Connect Four.

use crate::{bitboard, Error, Player, AREA, HEIGHT, NUM_PLAYERS, WIDTH};
use std::{array, collections::HashSet, fmt};

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

enum WinDirection {
    AscendingDiagonal,
    DescendingDiagonal,
    Horizontal,
    Vertical,
}

/// Represents a Connect Four game.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Game {
    /// A bitboard representing the pieces belonging to the current player.
    player_board: u64,
    /// A bitboard representing all the pieces played in the game.
    pieces_board: u64,
    /// The number of moves made in the game.
    moves: u8,
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

    /// Plays the current player's piece in the given 0-indexed column.
    ///
    /// # Errors
    /// Returns an [`Error`] if the move cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Error, Game};
    ///
    /// let mut game = Game::new();
    ///
    /// let result = game.play(3);
    /// assert_eq!(result, Ok(()));
    ///
    /// let result = game.play(7); // out of bounds
    /// assert_eq!(result, Err(Error::InvalidColumn));
    /// ```
    pub fn play(&mut self, col: u8) -> Result<(), Error> {
        self.can_play(col)?;

        self.play_unchecked(col);

        Ok(())
    }

    /// Plays the current player's piece in the given 0-indexed column, without checking if the move can be played.
    ///
    /// # Warning
    /// Playing into a column that is out of bounds or full may result in incorrect behavior.
    ///
    /// # Panics
    /// Panics if `column` overflows a bitboard in debug mode.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// game.play_unchecked(3);
    ///
    /// // game.play_unchecked(7);
    /// // ^^ out of bounds
    /// ```
    pub fn play_unchecked(&mut self, col: u8) {
        self.play_board(self.pieces_board + bitboard::bottom_piece_mask(col));
    }

    /// Plays the current player's piece given a move represented as a bitboard.
    pub(crate) fn play_board(&mut self, move_board: u64) {
        self.player_board ^= self.pieces_board;
        self.pieces_board |= move_board;
        self.moves += 1;
    }

    /// Plays a sequence of moves from a slice of 0-indexed columns.
    ///
    /// # Errors
    /// Returns an [`Error`] at the first move that cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Error, Game};
    ///
    /// let mut game = Game::new();
    /// let result = game.play_slice(&[3, 2, 3, 3, 3]);
    /// assert_eq!(result, Ok(()));
    ///
    /// let result = game.play_slice(&[3, 3, 3]); // overflowing column
    /// assert_eq!(result, Err(Error::ColumnFull));
    /// ```
    pub fn play_slice(&mut self, moves: &[u8]) -> Result<(), Error> {
        for col in moves {
            self.play(*col)?;
        }
        Ok(())
    }

    /// Plays a sequence of moves from a string of 0-indexed columns.
    ///
    /// # Errors
    /// Returns an [`Error`] at the first move that cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Error, Game};
    ///
    /// let mut game = Game::new();
    /// let result = game.play_str("323");
    /// assert_eq!(result, Ok(()));
    ///
    /// let result = game.play_str("33333"); // overflowing column
    /// assert_eq!(result, Err(Error::ColumnFull));
    ///
    /// let result = game.play_str("hello"); // invalid move string
    /// assert_eq!(result, Err(Error::InvalidColumn));
    /// ```
    pub fn play_str(&mut self, moves: &str) -> Result<(), Error> {
        for c in moves.chars() {
            let col = c
                .to_digit(10)
                .ok_or(Error::InvalidColumn)?
                .try_into()
                .unwrap();

            self.play(col)?;
        }
        Ok(())
    }

    /// Returns [`Ok(())`] if the given 0-indexed column can be played in the game board.
    ///
    /// # Errors
    /// Returns an [`Error`] if the move cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Error, Game};
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.can_play(3), Ok(()));
    ///
    /// game.play(3)?;
    /// assert_eq!(game.can_play(3), Ok(()));
    ///
    /// game.play_slice(&[3, 3, 3, 3, 3])?;
    /// assert_eq!(game.can_play(3), Err(Error::ColumnFull)); // column is full
    /// # Ok::<(), Error>(())
    /// ```
    pub fn can_play(&self, col: u8) -> Result<(), Error> {
        if self.is_game_over() {
            Err(Error::GameOver)
        } else if !self.is_inside(col) {
            Err(Error::InvalidColumn)
        } else if !self.is_unfilled(col) {
            Err(Error::ColumnFull)
        } else {
            Ok(())
        }
    }

    /// Checks if the given 0-indexed column is inside the game board.
    fn is_inside(&self, col: u8) -> bool {
        // No need to check for 0 < col because col is unsigned
        col < WIDTH
    }

    /// Checks if the given 0-indexed column is not full, assuming that `column` is inside the game board.
    pub(crate) fn is_unfilled(&self, col: u8) -> bool {
        (self.pieces_board & bitboard::top_piece_mask(col)) == 0
    }

    /// Returns a [`Status`] representing the current state of the game.
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
    /// game.play_slice(&[2, 3, 2, 3, 2, 3])?;
    /// assert_eq!(game.status(), Status::Win(Player::P1));
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn status(&self) -> Status {
        if let Some(player) = self.winner() {
            Status::Win(player)
        } else if self.has_full_board() {
            Status::Draw
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
    /// game.play_slice(&[2, 3, 2, 3, 2, 3])?;
    /// assert!(game.is_game_over());
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn is_game_over(&self) -> bool {
        self.has_full_board() || self.has_won()
    }

    /// Checks if the board is full and no more moves can be played.
    pub(crate) fn has_full_board(&self) -> bool {
        self.moves >= AREA
    }

    /// Returns the winner of the game or [`None`] if the game is a draw or still ongoing.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Player};
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.winner(), None);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.winner(), None);
    ///
    /// game.play_slice(&[2, 3, 2, 3, 2, 3])?;
    /// assert_eq!(game.winner(), Some(Player::P1));
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn winner(&self) -> Option<Player> {
        if self.is_winning_board(self.opponent_board()) {
            Some(!self.turn())
        } else {
            None
        }
    }

    /// Checks if the game has ended with a winner.
    fn has_won(&self) -> bool {
        self.winner().is_some()
    }

    /// Returns a bitboard of a given player's pieces.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Player};
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.bitboard(Player::P1), 0);
    /// assert_eq!(game.bitboard(Player::P2), 0);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.bitboard(Player::P1), 0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000);
    /// assert_eq!(game.bitboard(Player::P2), 0);
    ///
    /// game.play(0)?;
    /// assert_eq!(game.bitboard(Player::P1), 0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000);
    /// assert_eq!(game.bitboard(Player::P2), 0b_0000000_0000000_0000000_0000000_0000000_0000000_0000001);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn bitboard(&self, player: Player) -> u64 {
        if player == self.turn() {
            self.player_board
        } else {
            self.opponent_board()
        }
    }

    /// Returns a bitboard representing the pieces belonging to the opposing player.
    fn opponent_board(&self) -> u64 {
        self.player_board ^ self.pieces_board
    }

    /// Checks if a given bitboard has a line of four `1`s.
    fn is_winning_board(&self, board: u64) -> bool {
        self.check_win(board).is_some()
    }

    fn check_win(&self, board: u64) -> Option<(u64, WinDirection)> {
        use WinDirection::*;

        // Ascending diagonal /
        let x = board & (board >> (HEIGHT + 2));
        let new_board = x & (x >> ((HEIGHT + 2) * 2));
        if new_board != 0 {
            return Some((new_board, AscendingDiagonal));
        }

        // Descending diagonal \
        let x = board & (board >> HEIGHT);
        let new_board = x & (x >> (HEIGHT * 2));
        if new_board != 0 {
            return Some((new_board, DescendingDiagonal));
        }

        // Horizontal -
        let x = board & (board >> (HEIGHT + 1));
        let new_board = x & (x >> ((HEIGHT + 1) * 2));
        if new_board != 0 {
            return Some((new_board, Horizontal));
        }

        // Vertical |
        let x = board & (board >> 1);
        let new_board = x & (x >> 2);
        if new_board != 0 {
            return Some((new_board, Vertical));
        }
        None
    }

    /// Returns an array containing the `(x, y)` coordinates of four pieces that form a winning line horizontally, vertically, or diagonally. If no line exists (there is no winner), then [`None`] is returned.
    ///
    /// The order of coordinates is sorted. If there are multiple winning lines, then any one of the lines can be returned.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.winning_coordinates(), None);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.winning_coordinates(), None);
    ///
    /// game.play_slice(&[2, 3, 2, 3, 2, 3])?;
    /// assert_eq!(game.winning_coordinates(), Some([(3, 0), (3, 1), (3, 2), (3, 3)]));
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn winning_coordinates(&self) -> Option<[(u8, u8); 4]> {
        let (board, direction) = self.check_win(self.opponent_board())?;

        let index = u8::try_from(board.trailing_zeros()).unwrap();
        let start_col = index / (HEIGHT + 1);
        let start_row = index % (HEIGHT + 1);

        use WinDirection::*;
        Some(match direction {
            AscendingDiagonal => array::from_fn(|i| {
                let i = u8::try_from(i).unwrap();
                (start_col + i, start_row + i)
            }),
            DescendingDiagonal => array::from_fn(|i| {
                let i = u8::try_from(i).unwrap();
                (start_col + i, start_row - i)
            }),
            Horizontal => array::from_fn(|i| {
                let i = u8::try_from(i).unwrap();
                (start_col + i, start_row)
            }),
            Vertical => array::from_fn(|i| {
                let i = u8::try_from(i).unwrap();
                (start_col, start_row + i)
            }),
        })
    }

    /// Returns a bitboard of the playable moves that do not give the opponent an immediate win.
    /// If there are no possible moves that allow the current player to survive, then 0 is returned.
    pub(crate) fn non_losing_moves(&self) -> u64 {
        let possible_moves = self.possible_moves();
        let opponent_win = self.winning_board(self.opponent_board());
        let forced_moves = possible_moves & opponent_win;

        if forced_moves != 0 {
            if forced_moves & (forced_moves - 1) != 0 {
                // Opponent has more than one winning move and cannot be stopped
                0
            } else {
                // Opponent has exactly one winning move that can be blocked
                forced_moves & !(opponent_win >> 1)
            }
        } else {
            // Avoid playing below where an opponent can win
            possible_moves & !(opponent_win >> 1)
        }
    }

    /// Returns a bitboard of available moves.
    fn possible_moves(&self) -> u64 {
        (self.pieces_board + bitboard::BOTTOM_ROW_MASK) & bitboard::FULL_BOARD_MASK
    }

    /// Returns the number of winning moves the current player has after playing a given move.
    pub(crate) fn count_winning_moves(&self, move_board: u64) -> u32 {
        self.winning_board(self.player_board | move_board)
            .count_ones()
    }

    /// Returns a bitboard of tiles that can be played to win the game.
    fn winning_board(&self, board: u64) -> u64 {
        // Vertical |
        let mut x = (board << 1) & (board << 2) & (board << 3);

        // Ascending diagonal /
        let y = (board << HEIGHT) & (board << (2 * HEIGHT));
        x |= y & (board << (3 * (HEIGHT)));
        x |= y & (board >> (HEIGHT));

        let y = (board >> (HEIGHT)) & (board >> (2 * HEIGHT));
        x |= y & (board >> (3 * (HEIGHT)));
        x |= y & (board << (HEIGHT));

        // Horizontal -
        let y = (board << (HEIGHT + 1)) & (board << (2 * (HEIGHT + 1)));
        x |= y & (board << (3 * (HEIGHT + 1)));
        x |= y & (board >> (HEIGHT + 1));

        let y = (board >> (HEIGHT + 1)) & (board >> (2 * (HEIGHT + 1)));
        x |= y & (board >> (3 * (HEIGHT + 1)));
        x |= y & (board << (HEIGHT + 1));

        // Descending diagonal \
        let y = (board << (HEIGHT + 2)) & (board << (2 * (HEIGHT + 2)));
        x |= y & (board << (3 * (HEIGHT + 2)));
        x |= y & (board >> (HEIGHT + 2));

        let y = (board >> (HEIGHT + 2)) & (board >> (2 * (HEIGHT + 2)));
        x |= y & (board >> (3 * (HEIGHT + 2)));
        x |= y & (board << (HEIGHT + 2));

        x & (self.pieces_board ^ bitboard::FULL_BOARD_MASK)
    }

    /// Returns the number of moves made in the game.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.moves(), 0);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.moves(), 1);
    ///
    /// game.play_slice(&[0, 4, 6, 3])?;
    /// assert_eq!(game.moves(), 5);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn moves(&self) -> u8 {
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
    pub fn key(&self) -> u64 {
        self.player_board + self.pieces_board
    }

    /// Returns a symmetric base 3 key for the current game state.
    pub fn key3(&self) -> u64 {
        let key_forward = (0..WIDTH).fold(0, |key, col| self.partial_key3(key, col));

        let key_backward = (0..WIDTH)
            .rev()
            .fold(0, |key, col| self.partial_key3(key, col));

        if key_forward < key_backward {
            key_forward / 3
        } else {
            key_backward / 3
        }
    }

    fn partial_key3(&self, mut key: u64, col: u8) -> u64 {
        let mut mask = bitboard::bottom_piece_mask(col);
        while (self.pieces_board & mask) != 0 {
            key *= 3;
            if (self.player_board & mask) == 0 {
                key += 2;
            } else {
                key += 1;
            }
            mask <<= 1;
        }
        key *= 3;
        key
    }

    /// Returns the number of unique game positions at a specific depth.
    ///
    /// # Warning
    /// Running this at a large depth (>14) is computationally expensive.
    ///
    /// # Panics
    /// Panics if given a depth larger than [`WIDTH`] * [`HEIGHT`].
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// assert_eq!(Game::perft(0), 1);
    /// assert_eq!(Game::perft(1), 7);
    /// assert_eq!(Game::perft(2), 49);
    /// assert_eq!(Game::perft(3), 238);
    /// ```
    ///
    /// Passing a large depth, causing a panic:
    /// ```should_panic
    /// use connect_four_engine::Game;
    /// Game::perft(43); // this panics
    /// ```
    pub fn perft(depth: u8) -> u64 {
        assert!(depth <= AREA, "perft: depth is too high (maximum {})", AREA);
        let game = Self::new();
        Self::count_nodes(game, depth, &mut HashSet::new())
    }

    /// Helper function for perft.
    fn count_nodes(game: Game, depth: u8, seen: &mut HashSet<u64>) -> u64 {
        seen.insert(game.key());

        if depth == 0 {
            return 1;
        }

        if game.is_game_over() {
            return 0;
        }

        let mut nodes = 0;

        for i in 0..WIDTH {
            if game.is_unfilled(i) {
                let mut new_game = game;
                new_game.play_unchecked(i);
                if !seen.contains(&new_game.key()) {
                    nodes += Self::count_nodes(new_game, depth - 1, seen);
                }
            }
        }

        nodes
    }

    /// Returns the [`Player`] who owns the piece at `(x, y)`, or [`None`] if the tile is empty.
    ///
    /// # Panics
    /// Panics if given a coordinate that is out of bounds (x >= [`WIDTH`] or y >= [`HEIGHT`]).
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Player};
    ///
    /// let mut game = Game::new();
    /// game.play(0)?;
    /// game.play(1)?;
    ///
    /// assert_eq!(game.at(0, 0), Some(Player::P1));
    /// assert_eq!(game.at(1, 0), Some(Player::P2));
    /// assert_eq!(game.at(2, 0), None);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    ///
    /// Passing a coordinate that is out of bounds, causing a panic:
    /// ```should_panic
    /// # use connect_four_engine::Game;
    /// # let game = Game::new();
    /// let _ = game.at(7, 0); // this panics
    /// ```
    pub fn at(&self, x: u8, y: u8) -> Option<Player> {
        assert!(x < WIDTH, "at: x is out of bounds (maximum {WIDTH})");
        assert!(y < HEIGHT, "at: y is out of bounds (maximum {HEIGHT})");

        let turn = self.turn();
        let offset = x * WIDTH + y;
        let mask = 1 << offset;

        if self.player_board & mask != 0 {
            Some(turn)
        } else if self.opponent_board() & mask != 0 {
            Some(!turn)
        } else {
            None
        }
    }

    /// Returns the current state of the game as a 2D array where each element is [`Some(Player)`] if that player owns a piece at the location or [`None`] if the tile is empty.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Player};
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.board()[0][3], None);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.board()[0][3], Some(Player::P1));
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    /// The 2D array is in row-major order:
    /// ```
    /// # use connect_four_engine::Game;
    /// # let mut game = Game::new();
    /// # game.play_slice(&[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 4, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6])?;
    /// # let x = 0;
    /// # let y = 5;
    /// let a = game.board()[y][x];
    /// # let x = 0;
    /// # let y = 5;
    /// let b = game.at(x, y);
    /// assert_eq!(a, b);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn board(&self) -> [[Option<Player>; WIDTH as usize]; HEIGHT as usize] {
        array::from_fn(|y| {
            array::from_fn(|x| self.at(x.try_into().unwrap(), y.try_into().unwrap()))
        })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = self.board();
        board.reverse();

        fn fmt_tile(tile: Option<Player>) -> String {
            tile.map_or("_".into(), |player| player.to_string())
        }

        let rows = board.map(|row| row.map(fmt_tile).join(" "));

        write!(f, "{}", rows.join("\n"))
    }
}

impl fmt::Binary for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", Player::P1)?;
        writeln!(f, "{}", bitboard::format(self.bitboard(Player::P1)))?;
        writeln!(f)?;
        writeln!(f, "{:?}", Player::P2)?;
        write!(f, "{}", bitboard::format(self.bitboard(Player::P2)))
    }
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    use std::ops::RangeBounds;

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.player_board, 0);
        assert_eq!(game.pieces_board, 0);
        assert_eq!(game.moves, 0);
        assert!(!game.is_game_over());
        assert_eq!(game.status(), Status::Ongoing);
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
        let mut game1 = Game::new();
        game1.play_slice(&[3, 3, 3, 3])?;

        let mut game2 = Game::new();
        game2.play_str("3333")?;

        assert_eq!(
            game1.player_board,
            0b_0000000_0000000_0000000_0000101_0000000_0000000_0000000
        );
        assert_eq!(
            game1.pieces_board,
            0b_0000000_0000000_0000000_0001111_0000000_0000000_0000000
        );
        assert_eq!(game1.moves, 4);
        assert_eq!(game1, game2);
        Ok(())
    }

    #[test]
    fn out_of_bounds() {
        let mut game = Game::new();
        let result = game.play(7);
        assert_eq!(result, Err(Error::InvalidColumn));
    }

    #[test]
    fn full_column() {
        let mut game = Game::new();
        let result = game.play_slice(&[0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, Err(Error::ColumnFull));
    }

    #[test]
    fn no_win_overflow() -> Result<(), Error> {
        // _ X _ _ _ _ _
        // _ X _ _ _ _ _
        // _ X _ _ _ _ _
        // X O _ _ _ _ _
        // X O O _ _ _ _
        // X O O _ _ _ _
        let mut game = Game::new();
        game.play_slice(&[0, 1, 0, 1, 0, 1, 1, 2, 1, 2, 1])?;

        assert!(!game.is_game_over());
        assert_eq!(game.status(), Status::Ongoing);
        assert_eq!(game.winning_coordinates(), None);
        assert_eq!(game.play(0), Ok(()));
        Ok(())
    }

    fn test_end_game(
        moves: &[u8],
        status: Status,
        win_coords: Option<[(u8, u8); 4]>,
    ) -> Result<(), Error> {
        let Some((last, primary)) = moves.split_last() else {
            panic!("moves slice should have more than 1 move");
        };

        let mut game = Game::new();
        game.play_slice(primary)?;
        assert!(!game.is_game_over());
        assert_eq!(game.status(), Status::Ongoing);

        game.play(*last)?;
        assert!(game.is_game_over());
        assert_eq!(game.status(), status);
        assert_eq!(game.winning_coordinates(), win_coords);
        assert_eq!(game.play(0), Err(Error::GameOver));
        Ok(())
    }

    #[test]
    fn horizontal_win() -> Result<(), Error> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // O O O _ _ _ _
        // X X X X _ _ _
        test_end_game(
            &[0, 0, 1, 1, 2, 2, 3],
            Status::Win(Player::P1),
            Some([(0, 0), (1, 0), (2, 0), (3, 0)]),
        )
    }

    #[test]
    fn vertical_win() -> Result<(), Error> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // X _ _ _ _ _ _
        // X O _ _ _ _ _
        // X O _ _ _ _ _
        // X O _ _ _ _ _
        test_end_game(
            &[0, 1, 0, 1, 0, 1, 0],
            Status::Win(Player::P1),
            Some([(0, 0), (0, 1), (0, 2), (0, 3)]),
        )
    }

    #[test]
    fn ascending_diagonal_win() -> Result<(), Error> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ O _ _ _
        // _ _ O X _ _ _
        // _ O X O _ _ _
        // O X X X _ _ _
        test_end_game(
            &[3, 0, 1, 1, 2, 3, 2, 2, 3, 3],
            Status::Win(Player::P2),
            Some([(0, 0), (1, 1), (2, 2), (3, 3)]),
        )
    }

    #[test]
    fn descending_diagonal_win() -> Result<(), Error> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ O _ _ _
        // _ _ _ X O _ _
        // _ _ _ O X O _
        // _ _ _ X X X O
        test_end_game(
            &[3, 6, 5, 5, 4, 3, 4, 4, 3, 3],
            Status::Win(Player::P2),
            Some([(3, 3), (4, 2), (5, 1), (6, 0)]),
        )
    }

    #[test]
    fn multiple_wins() -> Result<(), Error> {
        // _ _ _ _ _ _ _
        // O O _ _ _ _ O
        // X X X X X X X
        // O O X X X O O
        // O X O X O X O
        // X O O X O O X
        test_end_game(
            &[
                0, 1, 1, 5, 6, 0, 5, 6, 3, 6, 6, 0, 0, 1, 1, 5, 5, 2, 3, 4, 3, 2, 2, 4, 4, 0, 2, 6,
                4, 1, 3,
            ],
            Status::Win(Player::P1),
            // currently chooses ascending diagonal over other directions, but the implementation is subject to change in the future
            Some([(0, 0), (1, 1), (2, 2), (3, 3)]),
        )
    }

    #[test]
    fn last_win() -> Result<(), Error> {
        // O O O O X X O
        // X O X X X O X
        // O X O O O X O
        // X O X X X O X
        // O X O O O X O
        // X O X X X O X
        test_end_game(
            &[
                0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 0, 3, 3, 3, 3, 3, 1, 4, 4, 4, 4, 4, 2,
                6, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 4, 3,
            ],
            Status::Win(Player::P2),
            Some([(0, 5), (1, 5), (2, 5), (3, 5)]),
        )
    }

    #[test]
    fn draw() -> Result<(), Error> {
        // O O O X O O O
        // X X X O X X X
        // O O O X O O O
        // X X X O X X X
        // O O O X O O O
        // X X X O X X X
        test_end_game(
            &[
                0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 4, 3, 3, 3, 3, 3, 3, 4, 4, 4,
                4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6,
            ],
            Status::Draw,
            None,
        )
    }

    #[test]
    fn format_string() -> Result<(), Error> {
        // _ _ _ _ _ _ X
        // _ _ _ _ _ X O
        // _ _ _ _ O O X
        // _ _ _ O X X O
        // _ _ X X O O X
        // _ X O O X X O
        let mut game = Game::new();
        game.play_slice(&[
            1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6,
        ])?;

        assert_eq!(game.to_string(), "_ _ _ _ _ _ X\n_ _ _ _ _ X O\n_ _ _ _ O O X\n_ _ _ O X X O\n_ _ X X O O X\n_ X O O X X O");
        Ok(())
    }

    fn test_perft_file<T>(depth: T)
    where
        T: RangeBounds<u8>,
    {
        let file = File::open("./test_data/perft.txt").unwrap();
        let reader = BufReader::new(file);

        for (i, text) in reader.lines().enumerate() {
            let i = i.try_into().unwrap();

            if !depth.contains(&i) {
                continue;
            }

            let expected: u64 = text.unwrap().parse().unwrap();
            let actual = Game::perft(i);
            assert_eq!(
                expected, actual,
                "perft({i}) expected = {expected}, actual = {actual}"
            );
        }
    }

    #[test]
    fn perft_shallow() {
        test_perft_file(..7);
    }

    #[test]
    #[ignore = "too slow"]
    fn perft_deep() {
        test_perft_file(7..21);
    }

    #[test]
    #[ignore = "too slow"]
    fn perft_deeper() {
        test_perft_file(21..35);
    }

    #[test]
    #[ignore = "too slow"]
    fn perft_deepest() {
        test_perft_file(35..42);
    }

    #[test]
    #[ignore = "too slow"]
    fn perft_max() {
        test_perft_file(42..);
    }
}

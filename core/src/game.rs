//! Functionality for creating and playing the game of Connect Four.

use crate::{bitboard, Board, MoveError, Player, WinDirection, AREA, HEIGHT, WIDTH};
use std::{array, collections::HashSet, fmt, str::FromStr};

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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Game {
    pub(crate) board: Board,
    /// A vector of all the moves played in the game.
    moves: Vec<u8>,
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

    /// Creates a new game from a string of 1-indexed columns.
    ///
    /// # Errors
    /// Returns a [`MoveError`] if any move cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    /// let game = Game::from_str("32164625")?;
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    #[allow(clippy::should_implement_trait)]
    // Implemented here so FromStr does not have to be imported to use directly
    pub fn from_str(s: &str) -> Result<Self, MoveError> {
        let mut game = Self::new();
        game.play_str(s)?;
        Ok(game)
    }

    /// Plays the current player's piece in the given 0-indexed column.
    ///
    /// # Errors
    /// Returns a [`MoveError`] if the move cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, MoveError};
    ///
    /// let mut game = Game::new();
    ///
    /// let result = game.play(3);
    /// assert_eq!(result, Ok(()));
    ///
    /// let result = game.play(7); // out of bounds
    /// assert_eq!(result, Err(MoveError::InvalidColumn));
    /// ```
    pub fn play(&mut self, col: u8) -> Result<(), MoveError> {
        self.can_play(col)?;
        self.board.play_unchecked(col);
        self.moves.push(col);
        Ok(())
    }

    /// Plays a sequence of moves from a slice of 0-indexed columns.
    ///
    /// # Errors
    /// Returns a [`MoveError`] at the first move that cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, MoveError};
    ///
    /// let mut game = Game::new();
    /// let result = game.play_slice(&[3, 2, 3, 3, 3]);
    /// assert_eq!(result, Ok(()));
    ///
    /// let result = game.play_slice(&[3, 3, 3]); // overflowing column
    /// assert_eq!(result, Err(MoveError::ColumnFull));
    /// ```
    pub fn play_slice(&mut self, moves: &[u8]) -> Result<(), MoveError> {
        for col in moves {
            self.play(*col)?;
        }
        Ok(())
    }

    /// Plays a sequence of moves from a string of 1-indexed columns.
    ///
    /// # Errors
    /// Returns a [`MoveError`] at the first move that cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, MoveError};
    ///
    /// let mut game = Game::new();
    /// let result = game.play_str("434");
    /// assert_eq!(result, Ok(()));
    ///
    /// let result = game.play_str("44444"); // overflowing column
    /// assert_eq!(result, Err(MoveError::ColumnFull));
    ///
    /// let result = game.play_str("0123"); // invalid move string (0-indexed)
    /// assert_eq!(result, Err(MoveError::InvalidColumn));
    ///
    /// let result = game.play_str("hello"); // invalid move string
    /// assert_eq!(result, Err(MoveError::InvalidColumn));
    /// ```
    pub fn play_str(&mut self, moves: &str) -> Result<(), MoveError> {
        fn char_to_col(c: char) -> Option<u8> {
            let n = c.to_digit(10)?;
            let n = u8::try_from(n).unwrap();
            n.checked_sub(1)
        }

        for c in moves.chars() {
            let col = char_to_col(c).ok_or(MoveError::InvalidColumn)?;
            self.play(col)?;
        }
        Ok(())
    }

    /// Undoes the last move played and returns the 0-indexed column. Returns [`None`] if no moves have been made.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// game.play_slice(&[0, 1, 2, 3])?;
    ///
    /// assert_eq!(game.undo(), Some(3));
    /// assert_eq!(game.at(3, 0), None);
    /// assert_eq!(game.moves(), &[0, 1, 2]);
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn undo(&mut self) -> Option<u8> {
        let col = self.moves.pop()?;
        self.board.undo_unchecked(col);
        Some(col)
    }

    /// Checks if a piece can be played in a given 0-indexed column, returning the number of pieces in the column.
    ///
    /// # Errors
    /// Returns a [`MoveError`] if the move cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, MoveError};
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.can_play(3), Ok(0));
    ///
    /// game.play(3)?;
    /// assert_eq!(game.can_play(3), Ok(1));
    ///
    /// game.play_slice(&[3, 3, 3, 3, 3])?;
    /// assert_eq!(game.can_play(3), Err(MoveError::ColumnFull)); // column is full
    /// # Ok::<(), MoveError>(())
    /// ```
    pub fn can_play(&self, col: u8) -> Result<u8, MoveError> {
        if self.is_over() {
            Err(MoveError::GameOver)
        } else if col >= WIDTH {
            Err(MoveError::InvalidColumn)
        } else if !self.board.is_open(col) {
            Err(MoveError::ColumnFull)
        } else {
            Ok(self.board.pieces_in_col(col))
        }
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
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn turn(&self) -> Player {
        if self.num_moves() % 2 == 0 {
            Player::P1
        } else {
            Player::P2
        }
    }

    /// Returns the number of moves made in the game.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.num_moves(), 0);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.num_moves(), 1);
    ///
    /// game.play_slice(&[0, 4, 6, 3])?;
    /// assert_eq!(game.num_moves(), 5);
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn num_moves(&self) -> u8 {
        self.board.num_moves()
    }

    /// Returns a slice of all the moves played in the game as 0-indexed columns.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.moves(), &[]);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.moves(), &[3]);
    ///
    /// game.play_slice(&[0, 4, 6, 3])?;
    /// assert_eq!(game.moves(), &[3, 0, 4, 6, 3]);
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn moves(&self) -> &[u8] {
        &self.moves
    }

    /// Returns a string of all the moves played in the game as 1-indexed columns.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.moves_str(), "");
    ///
    /// game.play_str("1234567")?;
    /// assert_eq!(game.moves_str(), "1234567");
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn moves_str(&self) -> String {
        let mut s = String::with_capacity(self.num_moves().into());
        for col in self.moves() {
            s.push(char::from_digit((col + 1).into(), 10).unwrap());
        }
        s
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
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn status(&self) -> Status {
        if self.board.has_opponent_won() {
            Status::Win(!self.turn())
        } else if self.board.is_full() {
            Status::Draw
        } else {
            Status::Ongoing
        }
    }

    /// Checks if the game is over and no more moves can be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::Game;
    ///
    /// let mut game = Game::new();
    /// assert!(!game.is_over());
    ///
    /// game.play(3)?;
    /// assert!(!game.is_over());
    ///
    /// game.play_slice(&[2, 3, 2, 3, 2, 3])?;
    /// assert!(game.is_over());
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn is_over(&self) -> bool {
        self.board.is_terminal()
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
    /// assert_eq!(game.win_coords(), None);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.win_coords(), None);
    ///
    /// game.play_slice(&[2, 3, 2, 3, 2, 3])?;
    /// assert_eq!(game.win_coords(), Some([(3, 0), (3, 1), (3, 2), (3, 3)]));
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn win_coords(&self) -> Option<[(u8, u8); 4]> {
        let (bitboard, direction) = self.board.opponent_winning_bb()?;

        let index = u8::try_from(bitboard.trailing_zeros()).unwrap();
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
    /// # Ok::<(), connect_four_engine::MoveError>(())
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

        if self.board.player_bb() & mask != 0 {
            Some(turn)
        } else if self.board.opponent_bb() & mask != 0 {
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
    /// assert_eq!(game.matrix()[0][3], None);
    ///
    /// game.play(3)?;
    /// assert_eq!(game.matrix()[0][3], Some(Player::P1));
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    /// The 2D array is in row-major order:
    /// ```
    /// # use connect_four_engine::Game;
    /// # let game = Game::from_str("111112222233333144444255555376666667777754")?;
    /// # let x = 0;
    /// # let y = 5;
    /// let a = game.matrix()[y][x];
    /// # let x = 0;
    /// # let y = 5;
    /// let b = game.at(x, y);
    /// assert_eq!(a, b);
    /// # Ok::<(), connect_four_engine::MoveError>(())
    /// ```
    pub fn matrix(&self) -> [[Option<Player>; WIDTH as usize]; HEIGHT as usize] {
        array::from_fn(|y| {
            array::from_fn(|x| self.at(x.try_into().unwrap(), y.try_into().unwrap()))
        })
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
        let board = Board::new();
        Self::count_nodes(board, depth, &mut HashSet::new())
    }

    /// Helper function for perft.
    fn count_nodes(board: Board, depth: u8, seen: &mut HashSet<u64>) -> u64 {
        seen.insert(board.key());

        if depth == 0 {
            return 1;
        }

        if board.is_terminal() {
            return 0;
        }

        let mut nodes = 0;

        for i in 0..WIDTH {
            if board.is_open(i) {
                let mut new_board = board;
                new_board.play_unchecked(i);
                if !seen.contains(&new_board.key()) {
                    nodes += Self::count_nodes(new_board, depth - 1, seen);
                }
            }
        }

        nodes
    }
}

impl FromStr for Game {
    type Err = MoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut matrix = self.matrix();
        matrix.reverse();

        fn fmt_tile(tile: Option<Player>) -> String {
            tile.map_or("_".into(), |player| player.to_string())
        }

        let rows = matrix.map(|row| row.map(fmt_tile).join(" "));

        write!(f, "{}", rows.join("\n"))
    }
}

impl fmt::Binary for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bitboards = if self.turn() == Player::P1 {
            (self.board.player_bb(), self.board.opponent_bb())
        } else {
            (self.board.opponent_bb(), self.board.player_bb())
        };

        writeln!(f, "{:?}", Player::P1)?;
        writeln!(f, "{}", bitboard::format(bitboards.0))?;
        writeln!(f)?;
        writeln!(f, "{:?}", Player::P2)?;
        write!(f, "{}", bitboard::format(bitboards.1))
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
        assert_eq!(game.board.player_bb(), 0);
        assert_eq!(game.board.occupied_bb(), 0);
        assert_eq!(game.num_moves(), 0);
        assert!(!game.is_over());
        assert_eq!(game.status(), Status::Ongoing);
    }

    #[test]
    fn play_one() -> Result<(), MoveError> {
        let mut game = Game::new();
        game.play(3)?;
        assert_eq!(
            game.board.player_bb(),
            0b_0000000_0000000_0000000_0000000_0000000_0000000_0000000
        );
        assert_eq!(
            game.board.occupied_bb(),
            0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000
        );
        assert_eq!(game.num_moves(), 1);
        Ok(())
    }

    #[test]
    fn play_multiple() -> Result<(), MoveError> {
        let mut game1 = Game::new();
        game1.play_slice(&[3, 3, 3, 3])?;

        let mut game2 = Game::new();
        game2.play_str("4444")?;

        assert_eq!(
            game1.board.player_bb(),
            0b_0000000_0000000_0000000_0000101_0000000_0000000_0000000
        );
        assert_eq!(
            game1.board.occupied_bb(),
            0b_0000000_0000000_0000000_0001111_0000000_0000000_0000000
        );
        assert_eq!(game1.num_moves(), 4);
        assert_eq!(game1, game2);
        Ok(())
    }

    #[test]
    fn out_of_bounds() {
        let mut game = Game::new();
        let result = game.play(7);
        assert_eq!(result, Err(MoveError::InvalidColumn));
    }

    #[test]
    fn full_column() {
        let mut game = Game::new();
        let result = game.play_str("1111111");
        assert_eq!(result, Err(MoveError::ColumnFull));
    }

    #[test]
    fn no_win_overflow() -> Result<(), MoveError> {
        // _ X _ _ _ _ _
        // _ X _ _ _ _ _
        // _ X _ _ _ _ _
        // X O _ _ _ _ _
        // X O O _ _ _ _
        // X O O _ _ _ _
        let game = Game::from_str("12121223232")?;

        assert!(!game.is_over());
        assert_eq!(game.status(), Status::Ongoing);
        assert_eq!(game.win_coords(), None);
        Ok(())
    }

    fn test_end_game(
        moves: &str,
        status: Status,
        win_coords: Option<[(u8, u8); 4]>,
    ) -> Result<(), MoveError> {
        let (first, last) = moves.split_at(moves.len() - 1);

        let mut game = Game::from_str(first)?;
        assert!(!game.is_over());
        assert_eq!(game.status(), Status::Ongoing);

        game.play_str(last)?;
        assert!(game.is_over());
        assert_eq!(game.status(), status);
        assert_eq!(game.win_coords(), win_coords);
        assert_eq!(game.play(0), Err(MoveError::GameOver));
        Ok(())
    }

    #[test]
    fn horizontal_win() -> Result<(), MoveError> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // O O O _ _ _ _
        // X X X X _ _ _
        test_end_game(
            "1122334",
            Status::Win(Player::P1),
            Some([(0, 0), (1, 0), (2, 0), (3, 0)]),
        )
    }

    #[test]
    fn vertical_win() -> Result<(), MoveError> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // X _ _ _ _ _ _
        // X O _ _ _ _ _
        // X O _ _ _ _ _
        // X O _ _ _ _ _
        test_end_game(
            "1212121",
            Status::Win(Player::P1),
            Some([(0, 0), (0, 1), (0, 2), (0, 3)]),
        )
    }

    #[test]
    fn ascending_diagonal_win() -> Result<(), MoveError> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ O _ _ _
        // _ _ O X _ _ _
        // _ O X O _ _ _
        // O X X X _ _ _
        test_end_game(
            "4122343344",
            Status::Win(Player::P2),
            Some([(0, 0), (1, 1), (2, 2), (3, 3)]),
        )
    }

    #[test]
    fn descending_diagonal_win() -> Result<(), MoveError> {
        // _ _ _ _ _ _ _
        // _ _ _ _ _ _ _
        // _ _ _ O _ _ _
        // _ _ _ X O _ _
        // _ _ _ O X O _
        // _ _ _ X X X O
        test_end_game(
            "4766545544",
            Status::Win(Player::P2),
            Some([(3, 3), (4, 2), (5, 1), (6, 0)]),
        )
    }

    #[test]
    fn multiple_wins() -> Result<(), MoveError> {
        // _ _ _ _ _ _ _
        // O O _ _ _ _ O
        // X X X X X X X
        // O O X X X O O
        // O X O X O X O
        // X O O X O O X
        test_end_game(
            "1226716747711226634543355137524",
            Status::Win(Player::P1),
            // currently chooses ascending diagonal over other directions, but the implementation is subject to change in the future
            Some([(0, 0), (1, 1), (2, 2), (3, 3)]),
        )
    }

    #[test]
    fn last_win() -> Result<(), MoveError> {
        // O O O O X X O
        // X O X X X O X
        // O X O O O X O
        // X O X X X O X
        // O X O O O X O
        // X O X X X O X
        test_end_game(
            "111112222233333144444255555376666667777754",
            Status::Win(Player::P2),
            Some([(0, 5), (1, 5), (2, 5), (3, 5)]),
        )
    }

    #[test]
    fn draw() -> Result<(), MoveError> {
        // O O O X O O O
        // X X X O X X X
        // O O O X O O O
        // X X X O X X X
        // O O O X O O O
        // X X X O X X X
        test_end_game(
            "111111222222333333544444455555666666777777",
            Status::Draw,
            None,
        )
    }

    #[test]
    fn format_string() -> Result<(), MoveError> {
        // _ _ _ _ _ _ X
        // _ _ _ _ _ X O
        // _ _ _ _ O O X
        // _ _ _ O X X O
        // _ _ X X O O X
        // _ X O O X X O
        let game = Game::from_str("233444555566666777777")?;

        assert_eq!(game.to_string(), "_ _ _ _ _ _ X\n_ _ _ _ _ X O\n_ _ _ _ O O X\n_ _ _ O X X O\n_ _ X X O O X\n_ X O O X X O");
        Ok(())
    }

    #[test]
    fn undo_moves() -> Result<(), MoveError> {
        let init_game = Game::new();
        let mut game = Game::from_str("111112222233333144444255555376666667777754")?;

        for i in 0..game.num_moves() {
            assert_eq!(game.num_moves(), AREA - i);
            game.undo();
        }

        assert_eq!(init_game, game);
        assert_eq!(game.num_moves(), 0);
        assert_eq!(game.moves().len(), 0);
        assert_eq!(game.undo(), None);
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

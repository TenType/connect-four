//! Solve and analyze Connect Four games.
//!
//! # Score
//! Every game position has a score that determines the outcome of a game, assuming that both players play perfectly and optimally.
//! * A positive score signifies that the current player can win.
//!   * A position has the score of 1 if the player can win with their last piece, 2 if the player can win with their second to last piece, etc.
//! * A score of 0 signifies that the game will end in a draw.
//! * A negative score signifies that the current player will lose.
//!   * A position has the score of -1 if the player loses with their last piece, -2 if the player loses with their second to last piece, etc.

use crate::{bitboard, Cache, Game, AREA, WIDTH};

/// The minimum possible score of a game position.
pub const MIN_SCORE: i8 = -MAX_SCORE;

/// The maximum possible score of a game position.
pub const MAX_SCORE: i8 = AREA as i8 / 2 - 3;

/// The reversed column exploration order, starting from the edge columns.
const REV_MOVE_ORDER: [u8; WIDTH as usize] = {
    let mut moves = [0; WIDTH as usize];
    let mut i = 0;
    while i < WIDTH {
        let n = WIDTH - i - 1;
        moves[i as usize] = (WIDTH / 2) + (n % 2) * (n / 2 + 1) - (1 - n % 2) * (n / 2);
        i += 1;
    }
    moves
};

/// An array of moves by number of winning moves, sorted in ascending order.
///
/// # Implementation
/// An insertion sort algorithm is used because it performs well on small arrays and is online (able to sort elements as it receives them).
/// The time complexity is O(n) best case and O(n^2) worst case, and the space complexity is O(1).
#[derive(Default)]
struct MoveSorter {
    entries: [(u64, u32); WIDTH as usize],
    len: usize,
}

impl MoveSorter {
    /// Creates a new, empty move sorter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a move, represented as a bitboard, and its number of winning moves into the move sorter, ensuring that the array remains sorted.
    pub fn insert(&mut self, move_board: u64, num_winning_moves: u32) {
        let mut index = self.len;
        self.len += 1;

        while index != 0 && self.entries[index - 1].1 > num_winning_moves {
            self.entries[index] = self.entries[index - 1];
            index -= 1;
        }

        self.entries[index] = (move_board, num_winning_moves);
    }
}

impl Iterator for MoveSorter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.entries[self.len].0)
        }
    }
}

/// A solver and analyzer for the game of Connect Four.
#[derive(Default)]
pub struct Engine {
    /// The number of nodes visited.
    node_count: u64,
    /// An opening book used to cache the scores of opening positions.
    pub opening_book: Cache,
    /// A transposition table used to cache the scores of previously-computed positions.
    pub tt_cache: Cache,
}

impl Engine {
    /// Creates a new engine with empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new engine with an opening book.
    pub fn with_opening_book(opening_book: Cache) -> Self {
        Self {
            opening_book,
            ..Self::default()
        }
    }

    /// Returns the number of nodes visited in the last evaluation.
    pub fn node_count(&self) -> u64 {
        self.node_count
    }

    /// Evaluates a game position, returning its score.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Engine};
    ///
    /// let mut game = Game::new();
    /// game.play_str("32164625")?;
    ///
    /// let mut engine = Engine::new();
    /// let score = engine.evaluate(game);
    /// assert_eq!(score, 11);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn evaluate(&mut self, game: Game) -> i8 {
        self.node_count = 0;
        self.solve(game)
    }

    /// Evaluates all the possible moves of a game position, returning the scores as an array.
    /// An element of the array is [`None`] if the move cannot be played.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Engine};
    ///
    /// let mut game = Game::new();
    /// game.play_str("662222576343651642712157")?;
    ///
    /// let mut engine = Engine::new();
    /// let scores = engine.evaluate_next(game);
    /// assert_eq!(scores, [Some(1), None, Some(-8), Some(8), Some(3), Some(1), Some(7)]);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn evaluate_next(&mut self, game: Game) -> [Option<i8>; WIDTH as usize] {
        self.node_count = 0;

        let mut scores = [None; WIDTH as usize];

        for col in 0..WIDTH {
            if game.is_unfilled(col) {
                let mut new_game = game;
                new_game.play_unchecked(col);
                if new_game.has_won() {
                    scores[col as usize] = Some(new_game.position_score());
                } else {
                    scores[col as usize] = Some(-self.solve(new_game));
                }
            }
        }

        scores
    }

    /// Entry function to solve a game.
    fn solve(&mut self, game: Game) -> i8 {
        if game.can_win_next() {
            return game.position_score();
        }

        if game.moves() <= self.opening_book.max_depth() {
            if let Ok(key3) = game.key3().try_into() {
                if let Some(score) = self.opening_book.get(&key3) {
                    return score;
                }
            }
        }

        let mut max = game.position_score();
        let mut min = -max;

        while min < max {
            let mut midpoint = min + (max - min) / 2;
            if midpoint <= 0 && min / 2 < midpoint {
                midpoint = min / 2;
            } else if midpoint >= 0 && max / 2 > midpoint {
                midpoint = max / 2;
            }

            let score = self.negamax(game, midpoint, midpoint + 1);

            if score <= midpoint {
                max = score;
            } else {
                min = score;
            }
        }
        min
    }

    /// Recursively solves a game using the negamax search algorithm, returning its score.
    fn negamax(&mut self, game: Game, alpha: i8, beta: i8) -> i8 {
        self.node_count += 1;

        if game.has_full_board() {
            return 0;
        }

        let non_losing_moves = game.non_losing_moves();
        if non_losing_moves == 0 {
            return -game.position_score();
        }

        let min = -game.position_score() + 1;
        if min >= beta {
            return min;
        }

        let max = self.tt_cache.get(&game.key()).unwrap_or(-min + 1);
        if alpha >= max {
            return max;
        }

        let mut moves = MoveSorter::new();

        for col in REV_MOVE_ORDER {
            let move_board = non_losing_moves & bitboard::column_mask(col);
            if move_board != 0 {
                moves.insert(move_board, game.count_winning_moves(move_board));
            }
        }

        for move_board in moves {
            let mut new_game = game;
            new_game.play_board(move_board);

            let score = -self.negamax(new_game, -beta, -alpha);
            if score >= beta {
                return score;
            }
        }
        self.tt_cache.insert(game.key(), alpha);
        alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    fn test_file(file_name: &str) {
        let path = format!("./test_data/{file_name}.csv");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut engine = Engine::new();

        for line in reader.lines().skip(1) {
            test_line(line.unwrap(), &mut engine);
        }
    }

    fn test_line(line: String, engine: &mut Engine) {
        let items: Vec<&str> = line.split(',').collect();

        let [moves, expected] = items[..] else {
            panic!("file line should have moves and score separated by a comma");
        };

        let expected: i8 = expected.parse().unwrap();
        assert_eval(engine, moves, expected);
    }

    fn assert_eval(engine: &mut Engine, moves: &str, expected: i8) {
        let mut game = Game::new();
        game.play_str(moves).expect("invalid move string");

        let actual = engine.evaluate(game);

        assert_eq!(
            expected, actual,
            "input = {moves}, expected = {expected}, actual = {actual}"
        );
    }

    #[test]
    fn begin_easy() {
        test_file("begin_easy");
    }

    #[test]
    #[ignore = "too slow"]
    fn begin_medium() {
        test_file("begin_medium");
    }

    #[test]
    #[ignore = "too slow"]
    fn begin_hard() {
        test_file("begin_hard");
    }

    #[test]
    fn middle_easy() {
        test_file("middle_easy");
    }

    #[test]
    fn middle_medium() {
        test_file("middle_medium");
    }

    #[test]
    fn end_easy() {
        test_file("end_easy");
    }

    #[test]
    fn last_move() {
        assert_eval(&mut Engine::new(), "112233", 18);
    }
}

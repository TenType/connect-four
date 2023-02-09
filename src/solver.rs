//! Solve and analyze Connect Four games.
//!
//! # Score
//! Every game position has a score that determines the outcome of a game, assuming that both players play perfectly and optimally.
//! * A positive score signifies that the current player can win.
//!   * A position has the score of 1 if the player can win with their last piece, 2 if the player can win with their second to last piece, etc.
//! * A score of 0 signifies that the game will end in a draw.
//! * A negative score signifies that the current player will lose.
//!   * A position has the score of -1 if the player loses with their last piece, -2 if the player loses with their second to last piece, etc.

use std::collections::HashMap;

use crate::{
    bitboard::{self, Bitboard},
    Game, HEIGHT, WIDTH,
};

/// The size of the score of a game position.
pub type Score = i8;

/// The minimum possible score of a game position.
pub const MIN_SCORE: Score = -((WIDTH * HEIGHT) as Score) / 2 + 3;

/// The maximum possible score of a game position.
pub const MAX_SCORE: Score = ((WIDTH * HEIGHT) as Score + 1) / 2 - 3;

/// The column exploration order, starting from the centermost columns.
const MOVE_ORDER: [usize; WIDTH] = {
    let mut moves = [0; WIDTH];
    let mut i = 0;
    while i < WIDTH {
        moves[i] = (WIDTH / 2) + (i % 2) * (i / 2 + 1) - (1 - i % 2) * (i / 2);
        i += 1;
    }
    moves
};

/// The reversed column exploration order, starting from the edge columns.
const REV_MOVE_ORDER: [usize; WIDTH] = {
    let mut moves = [0; WIDTH];
    let mut i = 0;
    while i < WIDTH {
        moves[i] = MOVE_ORDER[WIDTH - i - 1];
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
    entries: [(Bitboard, u32); WIDTH],
    len: usize,
}

impl MoveSorter {
    /// Creates a new, empty move sorter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a move, represented as a bitboard, and its number of winning moves into the move sorter, ensuring that the array remains sorted.
    pub fn insert(&mut self, move_board: Bitboard, num_winning_moves: u32) {
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
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.entries[self.len].0)
        }
    }
}

/// Represents the solver for a Connect Four game.
pub struct Solver {
    /// The number of nodes visited.
    node_count: usize,
    /// A transposition table used to cache the scores of previously-computed positions.
    trans_table: HashMap<Bitboard, Score>,
}

impl Solver {
    /// Solve a game, returning its score.
    ///
    /// # Examples
    /// ```
    /// use connect_four_engine::{Game, Solver};
    ///
    /// let mut game = Game::new();
    /// game.play_slice(&[2, 1, 0, 5, 3, 5, 1, 4])?;
    ///
    /// let score = Solver::solve(game);
    /// assert_eq!(score, 11);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn solve(game: Game) -> Score {
        let mut solver = Solver {
            node_count: 0,
            trans_table: HashMap::new(),
        };

        let mut min = -((WIDTH * HEIGHT - game.moves()) as Score) / 2;
        let mut max = (WIDTH * HEIGHT - game.moves()) as Score / 2;

        while min < max {
            let mut midpoint = min + (max - min) / 2;
            if midpoint <= 0 && min / 2 < midpoint {
                midpoint = min / 2;
            } else if midpoint >= 0 && max / 2 > midpoint {
                midpoint = max / 2;
            }

            let score = solver.negamax(game.clone(), midpoint, midpoint + 1);

            if score <= midpoint {
                max = score;
            } else {
                min = score;
            }
        }
        min
    }

    /// Recursively solves a game using the negamax search algorithm, returning its score.
    fn negamax(&mut self, game: Game, mut alpha: Score, mut beta: Score) -> Score {
        self.node_count += 1;

        let non_losing_moves = game.possible_non_losing_moves();
        if non_losing_moves == 0 {
            return -((WIDTH * HEIGHT - game.moves()) as Score) / 2;
        }

        if game.is_draw() {
            return 0;
        }

        let min = -((WIDTH * HEIGHT - 2 - game.moves()) as Score) / 2;
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        let mut max = ((WIDTH * HEIGHT - 1 - game.moves()) / 2) as Score;
        if let Some(score) = self.trans_table.get(&game.key()) {
            max = *score + MIN_SCORE - 1;
        }

        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }

        let mut moves = MoveSorter::new();

        for col in REV_MOVE_ORDER {
            let move_board = non_losing_moves & bitboard::column_mask(col);
            if move_board != 0 {
                moves.insert(move_board, game.num_of_winning_moves_after_play(move_board));
            }
        }

        for move_board in moves {
            let mut new_game = game.clone();
            new_game.play_board(move_board);

            let score = -self.negamax(new_game, -beta, -alpha);
            if score >= beta {
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }
        self.trans_table.insert(game.key(), alpha - MIN_SCORE + 1);
        alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    fn test_file(file_name: &str) {
        let path = format!("./test_data/{file_name}.txt");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            test_line(line.unwrap());
        }
    }

    fn test_line(line: String) {
        let items: Vec<&str> = line.split(' ').take(2).collect();

        let [moves, expected] = items[..] else {
            panic!("file line should have two strings separated by a space");
        };

        let expected: Score = expected.parse().unwrap();

        let mut game = Game::new();
        game.play_str(moves).expect("invalid move string");

        let actual = Solver::solve(game);

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
}

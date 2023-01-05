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

use crate::{bitboard::Bitboard, Game, HEIGHT, WIDTH};

/// The size of the score of a game position.
pub type Score = i8;

/// The minimum possible score of a game position.
pub const MIN_SCORE: Score = -((WIDTH * HEIGHT) as Score) / 2 + 3;

/// The maximum possible score of a game position.
pub const MAX_SCORE: Score = ((WIDTH * HEIGHT) as Score + 1) / 2 - 3;

/// Represents the solver for a Connect Four game.
pub struct Solver {
    /// The column exploration order, starting from the centermost columns.
    move_order: [usize; WIDTH],
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
    /// game.play_moves(&[2, 1, 0, 5, 3, 5, 1, 4])?;
    ///
    /// let score = Solver::solve(game);
    /// assert_eq!(score, 11);
    /// # Ok::<(), connect_four_engine::Error>(())
    /// ```
    pub fn solve(game: Game) -> Score {
        let mut solver = Solver {
            move_order: core::array::from_fn(|i| {
                (WIDTH / 2) + (i % 2) * (i / 2 + 1) - (1 - i % 2) * (i / 2)
            }),
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

        if game.is_draw() {
            return 0;
        }

        for col in 0..WIDTH {
            if game.is_unfilled(col) && game.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1 - game.moves()) / 2) as Score;
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

        for col in self.move_order {
            if game.is_unfilled(col) {
                let mut new_game = game.clone();
                new_game.unchecked_play(col);
                let score = -self.negamax(new_game, -beta, -alpha);
                if score >= beta {
                    return score;
                }
                if score > alpha {
                    alpha = score;
                }
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

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let items: Vec<&str> = line.split(' ').take(2).collect();

            if let [move_str, expected_str] = items[..] {
                let moves: Vec<usize> = move_str
                    .chars()
                    .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                    .collect();

                let mut game = Game::new();
                game.unchecked_play_moves(&moves);

                let actual = Solver::solve(game);
                let expected: Score = expected_str.parse().unwrap();

                assert_eq!(
                    expected, actual,
                    "input = {moves:?}, expected = {expected}, actual = {actual}"
                );
            } else {
                panic!("File line should have 2 items");
            }
        }
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

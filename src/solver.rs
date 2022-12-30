use crate::{Game, HEIGHT, WIDTH};

pub type Score = i8;

pub struct Solver {
    move_order: [usize; WIDTH],
    node_count: usize,
}

impl Solver {
    pub fn solve(game: Game) -> Score {
        let mut solver = Solver {
            move_order: core::array::from_fn(|i| {
                (WIDTH / 2) + (i % 2) * (i / 2 + 1) - (1 - i % 2) * (i / 2)
            }),
            node_count: 0,
        };

        solver.negamax(
            game,
            -((WIDTH * HEIGHT / 2) as Score),
            (WIDTH * HEIGHT / 2) as Score,
        )
    }

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

        let max = ((WIDTH * HEIGHT - 1 - game.moves()) / 2) as Score;

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

        alpha
    }
}

use crate::{Game, HEIGHT, WIDTH};

pub type Score = i8;

pub fn solve(game: Game) -> Score {
    negamax(
        game,
        -((WIDTH * HEIGHT / 2) as Score),
        (WIDTH * HEIGHT / 2) as Score,
    )
}

fn negamax(game: Game, mut alpha: Score, mut beta: Score) -> Score {
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

    for col in 0..WIDTH {
        if game.is_unfilled(col) {
            let mut new_game = game.clone();
            new_game.unchecked_play(col);
            let score = -negamax(new_game, -beta, -alpha);
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

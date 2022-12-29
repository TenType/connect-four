use crate::{Game, HEIGHT, WIDTH};

pub type Score = i8;

pub fn solve(game: Game) -> Score {
    negamax(game)
}

fn negamax(game: Game) -> Score {
    if game.is_draw() {
        return 0;
    }

    for col in 0..WIDTH {
        if game.is_unfilled(col) && game.is_winning_move(col) {
            return ((WIDTH * HEIGHT + 1 - game.moves()) / 2) as Score;
        }
    }

    let mut best_score = -((WIDTH * HEIGHT) as Score);

    for col in 0..WIDTH {
        if game.is_unfilled(col) {
            let mut new_game = game.clone();
            new_game.unchecked_play(col);
            let score = -negamax(new_game);
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

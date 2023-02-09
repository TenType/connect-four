use connect_four_engine::{Game, Solver};
use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let mut game = Game::new();
        game.play_str(input.trim()).expect("invalid move string");

        let score = Solver::solve(game);
        println!("{score}");
    }
}

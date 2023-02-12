use connect_four_engine::{Engine, Game};
use std::{
    io::{self, Write},
    time::Instant,
};

fn main() {
    let mut engine = Engine::new();

    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let mut game = Game::new();
        game.play_str(input.trim()).expect("invalid move string");

        println!("{game}");

        engine.reset_node_count();

        let now = Instant::now();
        let score = engine.evaluate(game);
        let time = now.elapsed();

        println!(
            "Evaluated {score} in {:.3?} with {} nodes ({} in cache)",
            time,
            engine.node_count(),
            engine.cache.len(),
        );
    }
}

use connect_four_engine::{Cache, Engine, Game};
use std::io::{self, Write};
use std::time::Instant;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Use opening book if provided
    let mut engine = if let Some(path) = args.get(1) {
        let bytes = fs::read(path).expect("cannot read file");
        let size = bytes.len() as f64 / 1024.0 / 1024.0;
        let cache = Cache::from_bytes(bytes).expect("file should have correct bytes format");
        println!(
            "Using opening book with {} entries ({size:.2} MB)",
            cache.len()
        );
        Engine::with_opening_book(cache)
    } else {
        println!("No opening book, provide an file path to use one");
        Engine::new()
    };

    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let mut game = Game::new();
        let input = input.trim();

        if !input.is_empty() {
            if let Err(e) = game.play_str(input) {
                println!("{e}");
                continue;
            }
        }

        println!("{game}");

        let now = Instant::now();
        let score = engine.evaluate(game);
        let time = now.elapsed();

        println!(
            "Evaluated {score} in {time:.3?} with {} nodes ({} in tt_cache)",
            engine.node_count(),
            engine.tt_cache.len(),
        );
    }
}

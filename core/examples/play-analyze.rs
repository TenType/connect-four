use connect_four_engine::{Cache, Engine, Game, Outcome, Rating};
use std::cmp::Ordering;
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

    let mut game = Game::new();
    loop {
        println!("{game}");
        if let Some(outcome) = game.outcome() {
            match outcome {
                Outcome::Win(player) => println!("Player {:?} won!", player as u8 + 1),
                Outcome::Draw => println!("Draw game!"),
            }
            println!("{}", game.moves_str());
            return;
        }

        let now = Instant::now();
        let analysis = engine.analyze(&game);
        let time = now.elapsed();

        let prediction = analysis.prediction();

        for (i, (s, p)) in analysis.scores.iter().zip(prediction).enumerate() {
            if let Some(score) = s {
                match score.cmp(&0) {
                    Ordering::Less => print!("\x1b[1;31m{score:3}"),
                    Ordering::Equal => print!("\x1b[1;37m ±0"),
                    Ordering::Greater => print!("\x1b[1;32m{score:+3}"),
                }
                print!(" in {:?}", p.unwrap().1);
            } else {
                print!("~~~");
            }
            print!("\x1b[0m");
            if i < analysis.scores.len() - 1 {
                print!(" | ");
            }
        }
        println!();

        let ratings = analysis.ratings();

        for (i, rating) in ratings.iter().enumerate() {
            if let Some(r) = rating {
                match r {
                    Rating::Best => print!("\x1b[1;32m !!"),
                    Rating::Good => print!("\x1b[1;32m OK"),
                    Rating::Inaccuracy => print!("\x1b[1;34m ?!"),
                    Rating::Mistake => print!("\x1b[1;33m ? "),
                    Rating::Blunder => print!("\x1b[1;31m ??"),
                }
            } else {
                print!("~~~");
            }
            print!("\x1b[0m");
            if i < analysis.scores.len() - 1 {
                print!(" | ");
            }
        }
        println!();

        println!(
            "\x1b[30mAnalyzed in {time:.3?} with {} nodes ({} in tt_cache)\x1b[0m",
            engine.node_count(),
            engine.tt_cache.len(),
        );

        loop {
            let mut input = String::new();

            print!("{0:?} {0} > ", game.turn());
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            if input.is_empty() {
                break;
            }

            match input {
                "c" | "clear" => {
                    game = Game::new();
                    println!("Cleared");
                    break;
                }
                "m" | "moves" => {
                    let moves = game.moves_str();
                    if moves.is_empty() {
                        eprintln!("No moves made yet");
                    } else {
                        println!("{moves}");
                    }
                }
                "u" | "undo" => {
                    if let Some(col) = game.undo() {
                        println!("{game}");
                        println!("Undid column {col}");

                        let moves = game.moves_str();
                        if !moves.is_empty() {
                            println!("Moves left: {moves}");
                        }
                    } else {
                        eprintln!("No moves to undo");
                    }
                }
                _ => {
                    if let Err(e) = game.play_str(input) {
                        eprintln!("{e}");
                    } else {
                        let first_char = input.chars().next().expect("move should exist");
                        let digit = first_char.to_digit(10).expect("move should be valid");
                        let last_move = u8::try_from(digit).unwrap() - 1;

                        let mut best_moves = analysis.best_moves();
                        match ratings[last_move as usize].expect("column should not be full") {
                            Rating::Best => {
                                print!("\x1b[1;32mBest move!");
                                best_moves.retain(|x| x != &last_move);
                                if !best_moves.is_empty() {
                                    print!(" Alternatives: {}", best_moves_str(best_moves));
                                }
                                println!("\x1b[0m");
                                break;
                            }
                            Rating::Good => print!("\x1b[1;32mGood move."),
                            Rating::Inaccuracy => print!("\x1b[1;34mInaccuracy."),
                            Rating::Mistake => print!("\x1b[1;33mMistake."),
                            Rating::Blunder => print!("\x1b[1;31mBlunder."),
                        }
                        print!(" Best was column {}.", best_moves_str(best_moves));
                        println!("\x1b[0m");
                        break;
                    }
                }
            }
        }
    }
}

fn best_moves_str(best_moves: Vec<u8>) -> String {
    best_moves
        .iter()
        .map(|x| (x + 1).to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

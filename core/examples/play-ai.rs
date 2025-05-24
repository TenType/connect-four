mod common;

use connect_four_engine::{Agent, AgentDifficulty, Cache, Engine, Game, Outcome, Player};
use std::io::{self, Write};
use std::time::Instant;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let ai_difficulty = if let Some(s) = args.get(1) {
        let s = s.to_lowercase();
        match s.as_str() {
            "1" | "random" => AgentDifficulty::Random,
            "2" | "easy" => AgentDifficulty::Easy,
            "3" | "moderate" => AgentDifficulty::Moderate,
            "4" | "advanced" => AgentDifficulty::Advanced,
            "5" | "perfect" => AgentDifficulty::Perfect,
            _ => {
                panic!("invalid AI difficulty argument, must be a number 1-5 or a difficulty name")
            }
        }
    } else {
        AgentDifficulty::Perfect
    };

    // Use opening book if provided
    let mut engine = if let Some(path) = args.get(2) {
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

    let human_player = match args.get(3) {
        Some(s) if s == "1" => Player::P1,
        Some(s) if s == "2" => Player::P2,
        None => Player::P1,
        _ => panic!("invalid human player argument, must either be 1 or 2"),
    };

    let feedback = match args.get(4) {
        Some(s) if s == "0" => 0,
        Some(s) if s == "1" => 1,
        Some(s) if s == "2" => 2,
        None => 0,
        _ => panic!("invalid feedback argument, must either be 0, 1, or 2"),
    };

    let mut game = Game::new();
    let mut agent = Agent::new(ai_difficulty);

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

        if feedback >= 2 {
            common::print_next_analysis(&analysis);
        }

        println!(
            "\x1b[30mAnalyzed in {time:.3?} with {} nodes ({} in tt_cache)\x1b[0m",
            engine.node_count(),
            engine.tt_cache.len(),
        );

        loop {
            let mut input = String::new();
            let turn = game.turn();

            print!("{0:?} {0} > ", turn);
            if turn != human_player {
                let chosen_move = agent.choose_move(&analysis);
                game.play(chosen_move).expect("ai move should be valid");
                println!(
                    "{} \x1b[1;36m(AI: {ai_difficulty:?})\x1b[0m",
                    chosen_move + 1
                );
                break;
            }
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            if input.is_empty() {
                break;
            }

            match input {
                "a" | "analyze" => {
                    common::print_next_analysis(&analysis);
                }
                "c" | "clear" => {
                    game = Game::new();
                    println!("Cleared");
                    break;
                }
                "e" | "eval" | "evaluate" => {
                    let now = Instant::now();
                    let score = engine.evaluate(&game);
                    let time = now.elapsed();

                    println!(
                        "Evaluated {score:+} in {time:.3?} with {} nodes ({} in tt_cache)",
                        engine.node_count(),
                        engine.tt_cache.len(),
                    );
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
                        println!("Undid column {col}");
                        if let Some(col) = game.undo() {
                            println!("Undid column {col}");
                            println!("{game}");
                        } else {
                            eprintln!("No moves to undo")
                        }

                        let moves = game.moves_str();
                        if !moves.is_empty() {
                            println!("Moves left: {moves}");
                        }
                    } else {
                        eprintln!("No moves to undo");
                    }
                }
                _ => {
                    if input.len() != 1 {
                        eprintln!("Only one move is allowed at a time");
                    } else if let Err(e) = game.play_str(input) {
                        eprintln!("{e}");
                    } else if turn == human_player {
                        if feedback >= 1 {
                            let first_char = input.chars().next().expect("move should exist");
                            let digit = first_char.to_digit(10).expect("move should be valid");
                            let last_move = u8::try_from(digit).unwrap() - 1;
                            common::print_move_rating(&analysis, last_move);
                        }
                        break;
                    }
                }
            }
        }
    }
}

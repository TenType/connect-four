use connect_four_engine::{
    Agent, AgentDifficulty, Analysis, Cache, Engine, Game, Outcome, Player, Rating,
};
use std::cmp::Ordering;
use std::io::{self, Write};
use std::time::Instant;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytes = fs::read("database/opening_book.bin").expect("cannot read file");
    let cache = Cache::from_bytes(bytes).expect("file should have correct bytes format");
    let mut engine = Engine::with_opening_book(cache);

    let feedback = match args.get(1) {
        Some(s) if s == "0" => 0,
        Some(s) if s == "1" => 1,
        Some(s) if s == "2" => 2,
        None => 0,
        _ => panic!("invalid feedback argument: must be 0 (no feedback), 1 (show rating after every move), or 2 (all feedback)"),
    };

    let ai_player = match args.get(2) {
        Some(s) if s == "0" => None,
        Some(s) if s == "1" => Some(Player::P1),
        Some(s) if s == "2" => Some(Player::P2),
        None => None,
        _ => panic!("invalid AI player argument: must be 0 (no AI), 1 (AI goes first), or 2 (AI goes second)"),
    };

    let ai_difficulty = if let Some(s) = args.get(3) {
        let s = s.to_lowercase();
        match s.as_str() {
            "0" | "random" => AgentDifficulty::Random,
            "1" | "easy" => AgentDifficulty::Easy,
            "2" | "medium" => AgentDifficulty::Medium,
            "3" | "hard" => AgentDifficulty::Hard,
            "4" | "nearperfect" => AgentDifficulty::NearPerfect,
            "5" | "perfect" => AgentDifficulty::Perfect,
            _ => {
                panic!("invalid AI difficulty argument: must be a number 0-5 or a difficulty name")
            }
        }
    } else {
        AgentDifficulty::Perfect
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

        println!(
            "\x1b[30mAnalyzed in {time:.3?} with {} nodes ({} in tt_cache)\x1b[0m",
            engine.node_count(),
            engine.tt_cache.len(),
        );

        if feedback >= 2 {
            print_next_analysis(&analysis);
        }

        loop {
            let mut input = String::new();
            let turn = game.turn();

            print!("{0:?} {0} > ", turn);
            if let Some(p) = ai_player {
                if turn == p {
                    let chosen_move = agent.choose_move(&analysis);
                    game.play(chosen_move).expect("ai move should be valid");
                    println!(
                        "{} \x1b[1;36m(AI: {ai_difficulty:?})\x1b[0m",
                        chosen_move + 1
                    );
                    break;
                }
            }
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            if input.is_empty() {
                break;
            }

            match input {
                "a" | "analyze" => {
                    print_next_analysis(&analysis);
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
                        if ai_player.is_some() {
                            if let Some(col) = game.undo() {
                                println!("Undid column {col}");
                            } else {
                                eprintln!("No moves to undo")
                            }
                        }
                        println!("{game}");
                        let moves = game.moves_str();
                        if !moves.is_empty() {
                            println!("Moves left: {moves}");
                        }
                    } else {
                        eprintln!("No moves to undo");
                    }
                }
                _ => {
                    if ai_player.is_some() && input.len() != 1 {
                        eprintln!("Only one move is allowed at a time in AI mode");
                    } else if let Err(e) = game.play_str(input) {
                        eprintln!("{e}");
                    } else {
                        if feedback >= 1 {
                            let first_char = input.chars().next().expect("move should exist");
                            let last_move =
                                first_char.to_digit(10).expect("move should be a digit") as u8 - 1;
                            print_move_rating(&analysis, last_move);
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn print_next_analysis(analysis: &Analysis) {
    let predictions = analysis.predictions();

    println!("{:=<43}", "");

    print!("| ");
    for (i, (s, p)) in analysis.scores.iter().zip(predictions).enumerate() {
        if let Some(score) = s {
            match score.cmp(&0) {
                Ordering::Less => print!("\x1b[1;31mL"),
                Ordering::Equal => print!("\x1b[1;37mD"),
                Ordering::Greater => print!("\x1b[1;32mW"),
            }
            print!("{:2}", p.unwrap().1);
        } else {
            print!("===");
        }
        print!("\x1b[0m");
        if i < analysis.scores.len() - 1 {
            print!(" | ");
        }
    }
    println!(" |");

    print!("| ");
    for (i, s) in analysis.scores.iter().enumerate() {
        if let Some(score) = s {
            match score.cmp(&0) {
                Ordering::Less => print!("\x1b[1;31m{score:3}"),
                Ordering::Equal => print!("\x1b[1;37m ±0"),
                Ordering::Greater => print!("\x1b[1;32m{score:+3}"),
            }
        } else {
            print!("===");
        }
        print!("\x1b[0m");
        if i < analysis.scores.len() - 1 {
            print!(" | ");
        }
    }
    println!(" |");

    print!("| ");
    for (i, s) in analysis.scores.iter().enumerate() {
        if let Some(score) = s {
            let rel_score = analysis.amplified_score(*score);
            match rel_score.cmp(&0) {
                Ordering::Less => print!("\x1b[1;31m{rel_score:3}"),
                Ordering::Equal => print!("\x1b[1;37m ±0"),
                Ordering::Greater => print!("\x1b[1;32m{rel_score:+3}"),
            }
        } else {
            print!("===");
        }
        print!("\x1b[0m");
        if i < analysis.scores.len() - 1 {
            print!(" | ");
        }
    }
    println!(" |");

    let ratings = analysis.ratings();

    print!("| ");
    for (i, rating) in ratings.iter().enumerate() {
        if let Some(r) = rating {
            let color = match r {
                Rating::Best | Rating::Good => "\x1b[1;32m",
                Rating::Inaccuracy => "\x1b[1;34m",
                Rating::Mistake => "\x1b[1;33m",
                Rating::Blunder => "\x1b[1;31m",
            };
            print!("{color}{:>3}\x1b[0m", r.to_string());
        } else {
            print!("===");
        }
        if i < analysis.scores.len() - 1 {
            print!(" | ");
        }
    }
    println!(" |");
    println!("{:=<43}", "");
}

fn print_move_rating(analysis: &Analysis, col: u8) {
    let ratings = analysis.ratings();
    let mut best_moves = analysis.best_moves();
    match ratings[col as usize].expect("column should not be full") {
        Rating::Best => {
            print!("\x1b[1;32mBest move!");
            best_moves.retain(|x| x != &col);
            if !best_moves.is_empty() {
                print!(" Alternatives: {}", format_moves_str(best_moves));
            }
            println!("\x1b[0m");
            return;
        }
        Rating::Good => print!("\x1b[1;32mGood move."),
        Rating::Inaccuracy => print!("\x1b[1;34mInaccuracy."),
        Rating::Mistake => print!("\x1b[1;33mMistake."),
        Rating::Blunder => print!("\x1b[1;31mBlunder."),
    }
    print!(" Best was column {}.", format_moves_str(best_moves));
    println!("\x1b[0m");
}

fn format_moves_str(moves: Vec<u8>) -> String {
    let moves: Vec<String> = moves.iter().map(|x| (x + 1).to_string()).collect();
    match moves.len() {
        0 => String::new(),
        1 => moves[0].clone(),
        2 => format!("{} or {}", moves[0], moves[1]),
        _ => {
            let all_but_last = &moves[..moves.len() - 1];
            let last = &moves[moves.len() - 1];
            format!("{}, or {}", all_but_last.join(", "), last)
        }
    }
}

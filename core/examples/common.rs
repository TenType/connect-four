use connect_four_engine::{Analysis, Rating};
use std::cmp::Ordering;

pub fn print_next_analysis(analysis: &Analysis) {
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

pub fn print_move_rating(analysis: &Analysis, col: u8) {
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

#[allow(dead_code)]
fn main() {
    panic!("this file should not be executed directly");
}

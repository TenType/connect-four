use connect_four_engine::{Game, Outcome};
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();

    loop {
        let mut input = String::new();

        print!("{:?} > ", game.turn());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "u" || input == "undo" {
            game.undo();
        } else if let Err(e) = game.play_str(input) {
            eprintln!("{e}");
            continue;
        }

        println!("{game}");

        if let Some(outcome) = game.outcome() {
            match outcome {
                Outcome::Win(player) => return println!("Player {:?} won!", player as u8 + 1),
                Outcome::Draw => return println!("Draw game!"),
            }
        }
    }
}

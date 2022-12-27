use connect_four_engine::{Game, Status};
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();

    loop {
        let mut input = String::new();

        print!("{:?} > ", game.turn());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let n: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Must be a nonnegative number");
                continue;
            }
        };

        match game.play(n) {
            Ok(status) => {
                println!("{game}");

                use Status::*;
                match status {
                    Win(player) => return println!("Player {:?} won!", player as usize + 1),
                    Draw => return println!("Draw game!"),
                    Ongoing => (),
                }
            }
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        }
    }
}

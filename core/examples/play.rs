use connect_four_engine::{Game, Status};
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();

    loop {
        let mut input = String::new();

        print!("{:?} > ", game.turn());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let n: u8 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Must be a nonnegative number");
                continue;
            }
        };

        if let Err(e) = game.play(n) {
            eprintln!("{e}");
            continue;
        }

        println!("{game}");

        use Status::*;
        match game.status() {
            Win(player) => return println!("Player {:?} won!", player as u8 + 1),
            Draw => return println!("Draw game!"),
            Ongoing => (),
        }
    }
}

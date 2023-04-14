use connect_four_engine::{Cache, Engine, Game, Player, Status};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct App {
    engine: Engine,
    game: Game,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> App {
        let cache = Cache::from_bytes(bytes).expect("file should have correct bytes format");

        App {
            engine: Engine::with_opening_book(cache),
            game: Game::new(),
        }
    }

    pub fn play(&mut self, col: u8) -> u8 {
        let row = self.game.can_play(col).unwrap_or(u8::MAX);
        let _ = self.game.play(col);
        row
    }

    pub fn is_game_over(&self) -> bool {
        self.game.is_over()
    }

    pub fn first_player_turn(&self) -> bool {
        self.game.turn() == Player::P1
    }

    pub fn winner(&self) -> u8 {
        match self.game.status() {
            Status::Win(Player::P1) => 1,
            Status::Win(Player::P2) => 2,
            _ => 0,
        }
    }

    pub fn is_draw(&self) -> bool {
        self.game.status() == Status::Draw
    }

    pub fn evaluate(&mut self) -> i8 {
        self.engine.evaluate(&self.game)
    }
}

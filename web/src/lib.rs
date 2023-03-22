use connect_four_engine::{Game, Player, Status, HEIGHT};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = Game)]
pub struct GameWrapper(Game);

impl Default for GameWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_class = Game)]
impl GameWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameWrapper {
        GameWrapper(Game::new())
    }

    pub fn can_play(&self, col: u8) -> bool {
        self.0.can_play(col).is_ok()
    }

    pub fn play(&mut self, col: u8) {
        self.0.play(col).expect("column should be valid")
    }

    pub fn is_game_over(&self) -> bool {
        self.0.is_game_over()
    }

    pub fn available_row(&self, col: u8) -> u8 {
        for row in 0..HEIGHT {
            if self.0.at(col, row).is_none() {
                return row;
            }
        }
        panic!("no pieces in column {col}")
    }

    pub fn first_player_turn(&self) -> bool {
        self.0.turn() == Player::P1
    }

    pub fn winner(&self) -> u8 {
        match self.0.winner() {
            None => 0,
            Some(Player::P1) => 1,
            Some(Player::P2) => 2,
        }
    }

    pub fn is_draw(&self) -> bool {
        self.0.status() == Status::Draw
    }
}

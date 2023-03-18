mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {name}!"));
}

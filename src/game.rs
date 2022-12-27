use crate::{NUM_PLAYERS, WIDTH};

pub struct Game {
    boards: [u64; NUM_PLAYERS],
    heights: [u64; WIDTH],
    moves: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            boards: [0, 0],
            heights: core::array::from_fn(|i| (WIDTH * i) as u64),
            moves: 0,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&mut self, col: usize) {
        self.boards[self.moves % NUM_PLAYERS] |= 1 << self.heights[col];
        self.heights[col] += 1;
        self.moves += 1;
    }

    pub fn play_moves(&mut self, moves: &[usize]) {
        for col in moves {
            self.play(*col);
        }
    }

    pub fn moves(&self) -> usize {
        self.moves
    }
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.boards, [0, 0]);
        assert_eq!(game.heights, [0, 7, 14, 21, 28, 35, 42]);
        assert_eq!(game.moves, 0);
    }

    #[test]
    fn play_one() {
        let mut game = Game::new();
        game.play(3);
        assert_eq!(
            game.boards,
            [
                0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000,
                0
            ]
        );
        assert_eq!(game.heights, [0, 7, 14, 22, 28, 35, 42]);
        assert_eq!(game.moves, 1);
    }

    #[test]
    fn play_multiple() {
        let mut game = Game::new();
        game.play_moves(&[3, 3, 3, 3]);
        assert_eq!(
            game.boards,
            [
                0b_0000000_0000000_0000000_0000101_0000000_0000000_0000000,
                0b_0000000_0000000_0000000_0001010_0000000_0000000_0000000,
            ]
        );
        assert_eq!(game.heights, [0, 7, 14, 25, 28, 35, 42]);
        assert_eq!(game.moves, 4);
    }
}

use crate::{Error, NUM_PLAYERS, WIDTH};

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
    #[allow(clippy::unusual_byte_groupings)]
    const TOP_MASK: u64 = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&mut self, col: usize) -> Result<(), Error> {
        self.can_play(col)?;

        self.boards[self.moves % NUM_PLAYERS] |= 1 << self.heights[col];
        self.heights[col] += 1;
        self.moves += 1;

        Ok(())
    }

    pub fn play_moves(&mut self, moves: &[usize]) -> Result<(), Error> {
        for col in moves {
            self.play(*col)?;
        }
        Ok(())
    }

    pub fn can_play(&self, col: usize) -> Result<(), Error> {
        if !self.is_inside(col) {
            Err(Error::OutOfBounds)
        } else if !self.is_unfilled(col) {
            Err(Error::ColumnFull)
        } else {
            Ok(())
        }
    }

    fn is_inside(&self, col: usize) -> bool {
        col < WIDTH
    }

    fn is_unfilled(&self, col: usize) -> bool {
        let new_board = self.boards[self.moves % NUM_PLAYERS] | (1 << self.heights[col]);
        new_board & Self::TOP_MASK == 0
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
    fn play_one() -> Result<(), Error> {
        let mut game = Game::new();
        game.play(3)?;
        assert_eq!(
            game.boards,
            [
                0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000,
                0
            ]
        );
        assert_eq!(game.heights, [0, 7, 14, 22, 28, 35, 42]);
        assert_eq!(game.moves, 1);
        Ok(())
    }

    #[test]
    fn play_multiple() -> Result<(), Error> {
        let mut game = Game::new();
        game.play_moves(&[3, 3, 3, 3])?;
        assert_eq!(
            game.boards,
            [
                0b_0000000_0000000_0000000_0000101_0000000_0000000_0000000,
                0b_0000000_0000000_0000000_0001010_0000000_0000000_0000000,
            ]
        );
        assert_eq!(game.heights, [0, 7, 14, 25, 28, 35, 42]);
        assert_eq!(game.moves, 4);
        Ok(())
    }

    #[test]
    fn out_of_bounds() {
        let mut game = Game::new();
        let result = game.play(7);
        assert_eq!(result, Err(Error::OutOfBounds));
    }

    #[test]
    fn full_column() {
        let mut game = Game::new();
        let result = game.play_moves(&[0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, Err(Error::ColumnFull));
    }
}

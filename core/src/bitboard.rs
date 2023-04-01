//! The bitboard type, utility masks, and formatting functions.
//!
//! # Configuration
//! In a standard 7x6 board, a bitboard represents the following configuration:
//! ```text
//! .  .  .  .  .  .  .
//! 5 12 19 26 33 40 47
//! 4 11 18 25 32 39 46
//! 3 10 17 24 31 38 45
//! 2  9 16 23 30 37 44
//! 1  8 15 22 29 36 43
//! 0  7 14 21 28 35 42
//! ```
//! 0 is the right-most bit, and 48 is the left-most bit.
//! There is an extra sentinel row of 0s at the top of the bitboard that denotes the separation of columns.

use crate::{AREA, HEIGHT, WIDTH};

/// Formats a bitboard into a [`String`].
/// **Note:** The top sentinel row, which does not contain any pieces, is omitted.
pub(crate) fn format(board: u64) -> String {
    let mut text = String::with_capacity((AREA * 2).into());
    for row in (0..HEIGHT).rev() {
        for col in 0..WIDTH {
            let index = col * WIDTH + row;
            let piece = if (board & (1 << index)) != 0 {
                '1'
            } else {
                '0'
            };

            text.push(piece);
            if col != WIDTH - 1 {
                text.push(' ');
            }
        }
        if row != 0 {
            text.push('\n');
        }
    }
    text
}

/// A mask representing the bottom row of tiles.
pub(crate) const BOTTOM_ROW_MASK: u64 = {
    let mut mask = 0;
    let mut col = 0;
    while col < WIDTH {
        mask |= bottom_piece_mask(col);
        col += 1;
    }
    mask
};

/// A mask representing all the tiles in a board.
pub(crate) const FULL_BOARD_MASK: u64 = BOTTOM_ROW_MASK * FIRST_COLUMN_MASK;

/// Returns a mask representing the top piece in the given 0-indexed column.
pub(crate) const fn top_piece_mask(col: u8) -> u64 {
    1 << (bottom_index(col) + HEIGHT - 1)
}

/// Returns a mask representing the bottom piece in the given 0-indexed column.
pub(crate) const fn bottom_piece_mask(col: u8) -> u64 {
    1 << bottom_index(col)
}

/// Returns a mask representing the tiles in the given 0-indexed column.
pub(crate) const fn column_mask(col: u8) -> u64 {
    FIRST_COLUMN_MASK << bottom_index(col)
}

/// A mask representing the tiles in the first column.
const FIRST_COLUMN_MASK: u64 = (1 << HEIGHT) - 1;

/// Returns the index of the bottom tile of a column.
const fn bottom_index(col: u8) -> u8 {
    col * (HEIGHT + 1)
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;

    #[test]
    fn format_string() {
        // 0 1 1 1 0 1 1
        // 1 1 1 0 1 1 0
        // 1 1 0 0 1 0 0
        // 0 0 1 0 0 1 1
        // 0 1 1 0 1 1 1
        // 1 1 0 1 1 1 0
        let expected = "0 1 1 1 0 1 1\n1 1 1 0 1 1 0\n1 1 0 0 1 0 0\n0 0 1 0 0 1 1\n0 1 1 0 1 1 1\n1 1 0 1 1 1 0";
        let actual = format(0b_0100110_0110111_0011011_0100001_0110110_0111011_0011001);
        assert_eq!(expected, actual);
    }

    #[test]
    fn masks() {
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 1 1 1 1 1 1 1
        assert_eq!(
            BOTTOM_ROW_MASK,
            0b_0000001_0000001_0000001_0000001_0000001_0000001_0000001
        );

        // 1 1 1 1 1 1 1
        // 1 1 1 1 1 1 1
        // 1 1 1 1 1 1 1
        // 1 1 1 1 1 1 1
        // 1 1 1 1 1 1 1
        // 1 1 1 1 1 1 1
        assert_eq!(
            FULL_BOARD_MASK,
            0b_0111111_0111111_0111111_0111111_0111111_0111111_0111111
        );

        // 0 0 0 1 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        assert_eq!(
            top_piece_mask(3),
            0b_0000000_0000000_0000000_0100000_0000000_0000000_0000000
        );

        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 1 0 0 0
        assert_eq!(
            bottom_piece_mask(3),
            0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000
        );

        // 0 0 0 1 0 0 0
        // 0 0 0 1 0 0 0
        // 0 0 0 1 0 0 0
        // 0 0 0 1 0 0 0
        // 0 0 0 1 0 0 0
        // 0 0 0 1 0 0 0
        assert_eq!(
            column_mask(3),
            0b_0000000_0000000_0000000_0111111_0000000_0000000_0000000
        );
    }
}

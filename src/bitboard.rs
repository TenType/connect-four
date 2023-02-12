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

use crate::{HEIGHT, WIDTH};

/// Formats a bitboard into a [`String`].
///
/// **Note:** The top sentinel row, which does not contain any pieces, is omitted.
///
/// # Examples
/// ```
/// use connect_four_engine::bitboard;
/// bitboard::format(0b_0000000_0000000_0000000_0001111_0000000_0000000_0000000);
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// ```
pub fn format(board: u64) -> String {
    let mut text = String::new();
    for row in (0..HEIGHT).rev() {
        for col in 0..WIDTH {
            let index = col * WIDTH + row;
            let piece = if (board & (1 << index)) != 0 {
                '1'
            } else {
                '0'
            };

            text.push(piece);
            text.push(' ');
        }
        if row != 0 {
            text.push('\n');
        }
    }
    text
}

/// Prints a formatted bitboard.
///
/// **Note:** The top sentinel row, which does not contain any pieces, is omitted.
///
/// # Examples
/// ```
/// use connect_four_engine::bitboard;
/// bitboard::print(0b_0000000_0000000_0000000_0001111_0000000_0000000_0000000);
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// ```
///
/// This is equivalent to:
/// ```
/// # use connect_four_engine::bitboard;
/// println!("{}", bitboard::format(0b_0000000_0000000_0000000_0001111_0000000_0000000_0000000));
/// ```
pub fn print(board: u64) {
    println!("{}", format(board));
}

/// Returns a mask representing the top piece in the given 0-indexed column.
///
/// # Examples
/// ```
/// use connect_four_engine::bitboard::top_piece_mask;
/// assert_eq!(top_piece_mask(3), 0b_0000000_0000000_0000000_0100000_0000000_0000000_0000000);
/// // 0 0 0 1 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// ```
pub const fn top_piece_mask(col: usize) -> u64 {
    1 << (bottom_index(col) + HEIGHT - 1)
}

/// Returns a mask representing the bottom piece in the given 0-indexed column.
///
/// # Examples
/// ```
/// use connect_four_engine::bitboard::bottom_piece_mask;
/// assert_eq!(bottom_piece_mask(3), 0b_0000000_0000000_0000000_0000001_0000000_0000000_0000000);
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0
/// // 0 0 0 1 0 0 0
/// ```
pub const fn bottom_piece_mask(col: usize) -> u64 {
    1 << bottom_index(col)
}

/// Returns a mask representing the tiles in the given 0-indexed column.
///
/// # Examples
/// ```
/// use connect_four_engine::bitboard::column_mask;
/// assert_eq!(column_mask(3), 0b_0000000_0000000_0000000_0111111_0000000_0000000_0000000);
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// // 0 0 0 1 0 0 0
/// ```
pub const fn column_mask(col: usize) -> u64 {
    FIRST_COLUMN_MASK << bottom_index(col)
}

/// A mask representing the tiles in the first column.
///
/// ```text
/// 1 0 0 0 0 0 0
/// 1 0 0 0 0 0 0
/// 1 0 0 0 0 0 0
/// 1 0 0 0 0 0 0
/// 1 0 0 0 0 0 0
/// 1 0 0 0 0 0 0
/// ```
const FIRST_COLUMN_MASK: u64 = (1 << HEIGHT) - 1;

/// Returns the index of the bottom tile of a column.
const fn bottom_index(col: usize) -> usize {
    col * (HEIGHT + 1)
}

/// A mask representing the bottom row of tiles.
///
/// ```text
/// 0 0 0 0 0 0 0
/// 0 0 0 0 0 0 0
/// 0 0 0 0 0 0 0
/// 0 0 0 0 0 0 0
/// 0 0 0 0 0 0 0
/// 1 1 1 1 1 1 1
/// ```
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
///
/// ```text
/// 1 1 1 1 1 1 1
/// 1 1 1 1 1 1 1
/// 1 1 1 1 1 1 1
/// 1 1 1 1 1 1 1
/// 1 1 1 1 1 1 1
/// 1 1 1 1 1 1 1
/// ```
pub(crate) const FULL_BOARD_MASK: u64 = BOTTOM_ROW_MASK * FIRST_COLUMN_MASK;

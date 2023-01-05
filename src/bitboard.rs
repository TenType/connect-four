use crate::{HEIGHT, WIDTH};

pub type Bitboard = u64;

pub fn format(board: Bitboard) -> String {
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

pub fn print(board: Bitboard) {
    println!("{}", format(board));
}

pub(crate) const fn top_piece_mask(col: usize) -> Bitboard {
    1 << (bottom_index(col) + HEIGHT - 1)
}

pub(crate) const fn bottom_piece_mask(col: usize) -> Bitboard {
    1 << bottom_index(col)
}

pub(crate) const fn column_mask(col: usize) -> Bitboard {
    FIRST_COLUMN_MASK << bottom_index(col)
}

const FIRST_COLUMN_MASK: Bitboard = (1 << HEIGHT) - 1;

const fn bottom_index(col: usize) -> usize {
    col * (HEIGHT + 1)
}

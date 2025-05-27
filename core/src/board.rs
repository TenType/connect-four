use crate::{bitboard, Game, AREA, HEIGHT, WIDTH};

pub(crate) enum WinDirection {
    AscendingDiagonal,
    DescendingDiagonal,
    Horizontal,
    Vertical,
}

/// Internal representation of a Connect Four board. In the [`Engine`], this serves as a node in the game tree.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Board {
    /// A bitboard representing the pieces belonging to the current player.
    player_bb: u64,
    /// A bitboard representing all the pieces played in the game.
    occupied_bb: u64,
    /// The number of moves made in the game.
    num_moves: u8,
}

impl Board {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Returns a bitboard representing the pieces belonging to the current player.
    pub(crate) fn player_bb(&self) -> u64 {
        self.player_bb
    }

    /// Returns a bitboard representing the pieces belonging to the opposing player.
    pub(crate) fn opponent_bb(&self) -> u64 {
        self.player_bb ^ self.occupied_bb
    }

    /// Returns a bitboard representing all the pieces.
    #[cfg(test)]
    pub(crate) fn occupied_bb(&self) -> u64 {
        self.occupied_bb
    }

    /// Plays the current player's piece in the given 0-indexed column without checking if the move can be played.
    pub(crate) fn play_unchecked(&mut self, col: u8) {
        self.play_bb(self.occupied_bb + bitboard::bottom_piece_mask(col));
    }

    /// Plays the current player's piece given a move represented as a bitboard.
    pub(crate) fn play_bb(&mut self, move_bb: u64) {
        self.player_bb ^= self.occupied_bb;
        self.occupied_bb |= move_bb;
        self.num_moves += 1;
    }

    /// Removes the topmost piece in the given 0-indexed column.
    pub(crate) fn undo_unchecked(&mut self, col: u8) {
        let move_bb = (self.occupied_bb + bitboard::bottom_piece_mask(col)) >> 1;
        self.occupied_bb ^= move_bb & bitboard::column_mask(col);
        self.player_bb ^= self.occupied_bb;
        self.num_moves -= 1;
    }

    /// Checks if the given 0-indexed column is not full, assuming that `col` is inside the game board.
    pub(crate) fn is_open(&self, col: u8) -> bool {
        (self.occupied_bb & bitboard::top_piece_mask(col)) == 0
    }

    /// Returns the number of pieces in a 0-indexed column.
    pub(crate) fn pieces_in_col(&self, col: u8) -> u8 {
        (self.occupied_bb & bitboard::column_mask(col))
            .count_ones()
            .try_into()
            .unwrap()
    }

    /// Checks if the board is full and no more moves can be played.
    pub(crate) fn is_full(&self) -> bool {
        self.num_moves >= AREA
    }

    pub(crate) fn is_terminal(&self) -> bool {
        self.is_full() || self.has_opponent_won()
    }

    pub(crate) fn has_opponent_won(&self) -> bool {
        self.opponent_winning_bb().is_some()
    }

    pub(crate) fn opponent_winning_bb(&self) -> Option<(u64, WinDirection)> {
        self.check_win(self.opponent_bb())
    }

    pub(crate) fn check_win(&self, bitboard: u64) -> Option<(u64, WinDirection)> {
        use WinDirection::*;

        // Ascending diagonal /
        let x = bitboard & (bitboard >> (HEIGHT + 2));
        let new_bitboard = x & (x >> ((HEIGHT + 2) * 2));
        if new_bitboard != 0 {
            return Some((new_bitboard, AscendingDiagonal));
        }

        // Descending diagonal \
        let x = bitboard & (bitboard >> HEIGHT);
        let new_bitboard = x & (x >> (HEIGHT * 2));
        if new_bitboard != 0 {
            return Some((new_bitboard, DescendingDiagonal));
        }

        // Horizontal -
        let x = bitboard & (bitboard >> (HEIGHT + 1));
        let new_bitboard = x & (x >> ((HEIGHT + 1) * 2));
        if new_bitboard != 0 {
            return Some((new_bitboard, Horizontal));
        }

        // Vertical |
        let x = bitboard & (bitboard >> 1);
        let new_bitboard = x & (x >> 2);
        if new_bitboard != 0 {
            return Some((new_bitboard, Vertical));
        }
        None
    }

    /// Returns a bitboard of the playable moves that do not give the opponent an immediate win.
    /// If there are no possible moves that allow the current player to survive, then 0 is returned.
    pub(crate) fn non_losing_moves_bb(&self) -> u64 {
        let possible_moves = self.possible_bb();
        let opponent_win = self.winning_bb(self.opponent_bb());
        let forced_moves = possible_moves & opponent_win;

        if forced_moves != 0 {
            if forced_moves & (forced_moves - 1) != 0 {
                // Opponent has more than one winning move and cannot be stopped
                0
            } else {
                // Opponent has exactly one winning move that can be blocked
                forced_moves & !(opponent_win >> 1)
            }
        } else {
            // Avoid playing below where an opponent can win
            possible_moves & !(opponent_win >> 1)
        }
    }

    /// Returns a bitboard of available moves.
    fn possible_bb(&self) -> u64 {
        (self.occupied_bb + bitboard::BOTTOM_ROW_MASK) & bitboard::FULL_BOARD_MASK
    }

    /// Checks whether the current player can win with their next move.
    pub(crate) fn can_win_next(&self) -> bool {
        self.winning_bb(self.player_bb) & self.possible_bb() != 0
    }

    /// Checks whether the current player can win by playing into a 0-indexed column.
    pub(crate) fn is_winning_move(&self, col: u8) -> bool {
        self.winning_bb(self.player_bb) & self.possible_bb() & bitboard::column_mask(col) != 0
    }

    /// Returns the number of winning moves the current player has after playing a given move.
    pub(crate) fn count_winning_moves(&self, move_bb: u64) -> u32 {
        self.winning_bb(self.player_bb | move_bb).count_ones()
    }

    /// Returns a bitboard of tiles that can be played to win the game.
    fn winning_bb(&self, bitboard: u64) -> u64 {
        // Vertical |
        let mut x = (bitboard << 1) & (bitboard << 2) & (bitboard << 3);

        // Ascending diagonal /
        let y = (bitboard << HEIGHT) & (bitboard << (2 * HEIGHT));
        x |= y & (bitboard << (3 * (HEIGHT)));
        x |= y & (bitboard >> (HEIGHT));

        let y = (bitboard >> (HEIGHT)) & (bitboard >> (2 * HEIGHT));
        x |= y & (bitboard >> (3 * (HEIGHT)));
        x |= y & (bitboard << (HEIGHT));

        // Horizontal -
        let y = (bitboard << (HEIGHT + 1)) & (bitboard << (2 * (HEIGHT + 1)));
        x |= y & (bitboard << (3 * (HEIGHT + 1)));
        x |= y & (bitboard >> (HEIGHT + 1));

        let y = (bitboard >> (HEIGHT + 1)) & (bitboard >> (2 * (HEIGHT + 1)));
        x |= y & (bitboard >> (3 * (HEIGHT + 1)));
        x |= y & (bitboard << (HEIGHT + 1));

        // Descending diagonal \
        let y = (bitboard << (HEIGHT + 2)) & (bitboard << (2 * (HEIGHT + 2)));
        x |= y & (bitboard << (3 * (HEIGHT + 2)));
        x |= y & (bitboard >> (HEIGHT + 2));

        let y = (bitboard >> (HEIGHT + 2)) & (bitboard >> (2 * (HEIGHT + 2)));
        x |= y & (bitboard >> (3 * (HEIGHT + 2)));
        x |= y & (bitboard << (HEIGHT + 2));

        x & (self.occupied_bb ^ bitboard::FULL_BOARD_MASK)
    }

    /// Mirrors the board so its columns are reflected horizontally (left-to-right).
    pub(crate) fn mirror(&mut self) {
        self.player_bb = bitboard::mirror(self.player_bb);
        self.occupied_bb = bitboard::mirror(self.occupied_bb);
    }

    pub(crate) fn position_score(&self, win_this_turn: bool) -> i8 {
        if win_this_turn {
            (AREA - self.num_moves + 1) as i8 / 2
        } else {
            (AREA - self.num_moves) as i8 / 2
        }
    }

    /// Returns a unique key for the current game state for use in the transposition table.
    pub(crate) fn key(&self) -> u64 {
        self.player_bb + self.occupied_bb
    }

    /// Returns a symmetric base 3 key for the current game state.
    pub(crate) fn key3(&self) -> u128 {
        let key_forward = (0..WIDTH).fold(0, |key, col| self.partial_key3(key, col));

        let key_backward = (0..WIDTH)
            .rev()
            .fold(0, |key, col| self.partial_key3(key, col));

        if key_forward < key_backward {
            key_forward / 3
        } else {
            key_backward / 3
        }
    }

    fn partial_key3(&self, mut key: u128, col: u8) -> u128 {
        let mut mask = bitboard::bottom_piece_mask(col);
        while (self.occupied_bb & mask) != 0 {
            key *= 3;
            if (self.player_bb & mask) == 0 {
                key += 2;
            } else {
                key += 1;
            }
            mask <<= 1;
        }
        key *= 3;
        key
    }

    pub(crate) fn num_moves(&self) -> u8 {
        self.num_moves
    }
}

impl From<&Game> for Board {
    fn from(game: &Game) -> Self {
        game.board
    }
}

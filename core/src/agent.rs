//! AI agents to play moves in Connect Four games.

use crate::{Analysis, Outcome, Rating, WIDTH};
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug)]
pub enum AgentDifficulty {
    Random,
    Easy,
    Medium,
    Hard,
    NearPerfect,
    Perfect,
}

pub struct Agent {
    difficulty: AgentDifficulty,
    rng: ThreadRng,
}

impl Agent {
    pub fn new(difficulty: AgentDifficulty) -> Self {
        Self {
            difficulty,
            rng: rand::rng(),
        }
    }

    pub fn choose_move(&mut self, analysis: &Analysis) -> u8 {
        use crate::Rating::*;
        use AgentDifficulty::*;
        let mut possible_moves = match self.difficulty {
            Random => Vec::new(),
            Easy => self.filter_moves_by_lookahead(analysis, 1),
            Medium => self.filter_moves_by_rating(analysis, Mistake),
            Hard => self.filter_moves_by_rating(analysis, Inaccuracy),
            NearPerfect => self.filter_moves_by_rating(analysis, Good),
            Perfect => self.filter_moves_by_rating(analysis, Best),
        };
        if possible_moves.is_empty() {
            for (s, col) in analysis.scores.iter().zip(0..WIDTH) {
                if s.is_some() {
                    possible_moves.push(col);
                }
            }
        }
        possible_moves[self.rng.random_range(0..possible_moves.len())]
    }

    fn filter_moves_by_rating(&mut self, analysis: &Analysis, worst_rating: Rating) -> Vec<u8> {
        let ratings = analysis.ratings();
        let mut possible_moves = Vec::new();
        for (possible_rating, col) in ratings.iter().zip(0..WIDTH) {
            if let Some(rating) = possible_rating {
                if rating >= &worst_rating {
                    possible_moves.push(col);
                }
            }
        }
        possible_moves
    }

    fn filter_moves_by_lookahead(&mut self, analysis: &Analysis, lookahead: u8) -> Vec<u8> {
        let predictions = analysis.predictions();

        let mut possible_moves = Vec::new();
        for (p, col) in predictions.iter().zip(0..WIDTH) {
            if let Some(prediction) = p {
                // Can win in `lookahead` moves or less
                if prediction.0 == Outcome::Win(analysis.player) && prediction.1 <= lookahead {
                    possible_moves.push(col);
                }
            }
        }
        if !possible_moves.is_empty() {
            return possible_moves;
        }
        for (p, col) in predictions.iter().zip(0..WIDTH) {
            if let Some(prediction) = p {
                // Exclude moves that lose in `lookahead` moves or less
                if prediction.0 != Outcome::Win(!analysis.player) || prediction.1 > lookahead {
                    possible_moves.push(col);
                }
            }
        }
        possible_moves
    }
}

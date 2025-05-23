//! AI agents to play moves in Connect Four games.

use crate::{Analysis, WIDTH};
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug)]
pub enum AgentDifficulty {
    Random,
    Easy,
    Moderate,
    Advanced,
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
        let worst_rating = match self.difficulty {
            Random => Blunder,
            Easy => Mistake,
            Moderate => Inaccuracy,
            Advanced => Good,
            Perfect => Best,
        };

        let ratings = analysis.ratings();
        let mut possible_moves = Vec::new();
        for (possible_rating, col) in ratings.iter().zip(0..WIDTH) {
            if let Some(rating) = possible_rating {
                if rating >= &worst_rating {
                    possible_moves.push(col);
                }
            }
        }
        possible_moves[self.rng.random_range(0..possible_moves.len())]
    }
}

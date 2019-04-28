use std::io;
use std::time::Duration;

use shakmaty::{fen::Fen, uci::Uci, Color, Move, MoveList, Outcome, Position, Role, Setup};

/// this example requires a newtype due to orphan rules, as both shakmaty::Chess and rubot::Game
/// are from outside of this example.
#[derive(Debug, Clone, Default)]
pub struct Chess(shakmaty::Chess);

impl rubot::Game for Chess {
    type Player = Color;
    type Action = Move;
    type Actions = MoveList;
    type Fitness = i32;

    fn actions(&self, player: &Self::Player) -> (bool, Self::Actions) {
        (*player == self.0.turn(), self.0.legals())
    }

    fn execute(&mut self, action: &Self::Action, player: &Self::Player) -> Self::Fitness {
        self.0.play_unchecked(action);

        if let Some(outcome) = self.0.outcome() {
            match outcome {
                Outcome::Draw => 0,
                Outcome::Decisive { winner } => {
                    if winner == *player {
                        std::i32::MAX
                    } else {
                        std::i32::MIN
                    }
                }
            }
        } else {
            let mut fitness = 0;
            for (_square, piece) in self.0.board().pieces() {
                // values based on https://medium.freecodecamp.org/simple-chess-ai-step-by-step-1d55a9266977
                let value = match piece.role {
                    Role::Pawn => 10,
                    Role::Knight => 30,
                    Role::Bishop => 30,
                    Role::Rook => 50,
                    Role::Queen => 90,
                    Role::King => 900,
                };

                if piece.color == *player {
                    fitness += value;
                } else {
                    fitness -= value;
                }
            }
            fitness
        }
    }
}

impl Chess {
    /// panics if `fen` is not valid
    pub fn from_fen(fen: &str) -> Self {
        Chess(shakmaty::Chess::from_setup(&Fen::from_ascii(fen.as_bytes()).unwrap()).unwrap())
    }
}

use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn random() -> Move {
        match rand::thread_rng().gen_range(0..3) {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissors,
        }    
    }

    fn beats(&self, other: Move) -> bool {
        (*self == Move::Rock && other == Move::Scissors)
            || (*self == Move::Paper && other == Move::Rock)
            || (*self == Move::Scissors && other == Move::Paper)    
    }
}

struct Strategy {
    history: Vec<Move>,
}

impl Strategy {
    fn new() -> Strategy {
        Strategy { history: Vec::new() }    
    }
}

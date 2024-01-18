use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

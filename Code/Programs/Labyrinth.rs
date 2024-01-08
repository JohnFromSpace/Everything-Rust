use rand::Rng;
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Open,
    Start,
    End,
    Player,
}

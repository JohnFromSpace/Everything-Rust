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

struct Labyrinth {
    grid: Vec<Vec<Cell>>,
    player_position: (usize, usize),
    end_position: (usize, usize),
}

impl Labyrinth {
    fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();

         // Generate a random labyrinth
        let mut grid = vec![vec![Cell::Wall; size]; size];

        // Place the player at the start
        let start_position = (rng.gen_range(0..size), rng.gen_range(0..size));
        grid[start_position.0][start_position.1] = Cell::Start;
        
    }
}

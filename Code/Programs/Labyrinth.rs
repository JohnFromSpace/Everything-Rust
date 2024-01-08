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

        // Place the end point
        let end_position = (rng.gen_range(0..size), rng.gen_range(0..size));
        grid[end_position.0][end_position.1] = Cell::End;

        // Make sure the start and end points are not the same
        while start_position == end_position {
            let new_end_position = (rng.gen_range(0..size), rng.gen_range(0..size));
            grid[new_end_position.0][new_end_position.1] = Cell::End;
        }

        Labyrinth {
            grid,
            player_position: start_position,
            end_position,
        }
    }

    fn print(&self) {
         for row in &self.grid {
            for cell in row {
                match cell {
                    Cell::Wall => print!("# "),
                    Cell::Open => print!(". "),
                    Cell::Start => print!("S "),
                    Cell::End => print!("E "),
                    Cell::Player => print!("P "),
                }
            }
            println!();
        }   
    }

    fn move_player(&mut self, direction: &str) -> bool {
        let (mut new_row, mut new_col) = self.player_position;  

        match direction {
            "up" => new_row -= 1,
            "down" => new_row += 1,
            "left" => new_col -= 1,
            "right" => new_col += 1,
            _ => return false,
        }

         if new_row < self.grid.len() && new_col < self.grid[0].len() && self.grid[new_row][new_col] != Cell::Wall {
            // Move the player
            self.grid[self.player_position.0][self.player_position.1] = Cell::Open;
            self.player_position = (new_row, new_col);
            self.grid[self.player_position.0][self.player_position.1] = Cell::Player;
             
            true
        } else {
            false
        }   
    }

    fn is_player_at_end(&self) -> bool {
        
    }
}

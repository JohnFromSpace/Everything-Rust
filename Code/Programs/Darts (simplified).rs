use rand::Rng;
use std::io;

const DAYS_IN_A_WEEK: usize = 7;
const WEEKS_IN_A_YEAR: usize = 52;

#[derive(Debug)]
struct Player {
    name: String,
    proficiency: f64,
    cumulative_score: usize,
}

impl Player {
    fn new(name: &str, proficiency: f64) -> Self {
        Player {
            name: name.to_string(),
            proficiency,
            cumulative_score: 0,
        }
    }

    fn throw_dart(&self) -> usize {
        
    }
}

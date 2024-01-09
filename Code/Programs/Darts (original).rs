use rand::Rng;
use std::collections::{HashMap, BTreeMap};
use std::io;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    proficiency: f64,
    total_score: usize,
    average_score: f64,
}

impl Player {
    fn new(name: &str, proficiency: f64) -> Self {
        Player {
            name: name.to_string(),
            proficiency,
            total_score: 0,
            average_score: 0.0,
        }    
    }    
}

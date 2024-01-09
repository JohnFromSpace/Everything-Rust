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

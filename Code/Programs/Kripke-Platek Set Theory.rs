use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum VonNeumann {
    Set(HashSet<VonNeumann>),
    Successor(Box<VonNeumann>),
}

// A structure to represent a model in Kripke-Platek set theory
#[derive(Debug)]
struct KripkePlatekModel {
    universe: HashSet<VonNeumann>,
    admissible_sets: HashSet<HashSet<VonNeumann>>,
}

impl KripkePlaket {
    
}

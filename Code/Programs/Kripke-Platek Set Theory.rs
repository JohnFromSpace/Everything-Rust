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
    fn new() -> Self {
        KripkePlatekModel {
            universe: HashSet::new(),
            admissible_sets: HashSet::new(),
        }     
    } 

    // Axiom of extensionality: For all sets x and y, x = y if and only if for all z, z is in x if and only if z is in y.
    fn axiom_of_extensionality(&self, set1: &HashSet<VonNeumann>, set2: &HashSet<VonNeumann>) -> bool {
        set1 == set2
    }
}

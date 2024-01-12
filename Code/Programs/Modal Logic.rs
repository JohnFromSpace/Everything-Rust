use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Proposition {
    Atom(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ModalOperator {
    Box,     // Necessity
    Diamond, // Possibility
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Agent {
    Agent1,
    Agent2,
    // Add more agents as needed
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TemporalOperator {
    Future, // F
    Past,   // P
}


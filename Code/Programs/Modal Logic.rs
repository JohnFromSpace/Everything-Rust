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

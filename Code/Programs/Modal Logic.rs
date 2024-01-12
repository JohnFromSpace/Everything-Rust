use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Proposition {
    Atom(String),
}

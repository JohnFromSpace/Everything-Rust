use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum VonNeumann {
    Set(HashSet<VonNeumann>),
    Successor(Box<VonNeumann>),
}

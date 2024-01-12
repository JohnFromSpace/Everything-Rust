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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DeonticOperator {
    Obligation, // O
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum AlethicFormula {
    Atom(Proposition),
    Not(Box<AlethicFormula>),
    And(Box<AlethicFormula>, Box<AlethicFormula>),
    Or(Box<AlethicFormula>, Box<AlethicFormula>),
    Modal(ModalOperator, Box<AlethicFormula>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum EpistemicFormula {
    Atom(Proposition),
    Not(Box<EpistemicFormula>),
    And(Box<EpistemicFormula>, Box<EpistemicFormula>),
    Or(Box<EpistemicFormula>, Box<EpistemicFormula>),
    Modal(ModalOperator, Box<EpistemicFormula>, Agent),
}

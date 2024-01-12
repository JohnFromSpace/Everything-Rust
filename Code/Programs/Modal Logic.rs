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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DoxasticFormula {
    Atom(Proposition),
    Not(Box<DoxasticFormula>),
    And(Box<DoxasticFormula>, Box<DoxasticFormula>),
    Or(Box<DoxasticFormula>, Box<DoxasticFormula>),
    Modal(ModalOperator, Box<DoxasticFormula>, Agent),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TemporalFormula {
    Atom(Proposition),
    Not(Box<TemporalFormula>),
    And(Box<TemporalFormula>, Box<TemporalFormula>),
    Or(Box<TemporalFormula>, Box<TemporalFormula>),
    Modal(TemporalOperator, Box<TemporalFormula>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DeonticFormula {
    Atom(Proposition),
    Not(Box<DeonticFormula>),
    And(Box<DeonticFormula>, Box<DeonticFormula>),
    Or(Box<DeonticFormula>, Box<DeonticFormula>),
    Modal(DeonticOperator, Box<DeonticFormula>),
}

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

#[derive(Debug)]
struct KripkeFrame {
    states: HashSet<String>,
    accessibility: HashMap<String, HashSet<String>>,
}

#[derive(Debug)]
struct KripkeModel {
    frames: Vec<KripkeFrame>,
}

impl KripkeModel {
    fn new() -> Self {
        KripkeModel { frames: Vec::new() }    
    }

     fn evaluate_alethic_formula_at_state(&self, frame: &KripkeFrame, state: &str, formula: &AlethicFormula) -> bool {
         match formula {
             AlethicFormula::Atom(prop) => frame.states.contains(&prop.to_string()),
             AlethicFormula::Not(sub_formula) => !self.evaluate_alethic_formula_at_state(frame, state, sub_formula),
             AlethicFormula::And(sub_formula1, sub_formula2) => {
                 self.evaluate_alethic_formula_at_state(frame, state, sub_formula1)
                 && self.evaluate_alethic_formula_at_state(frame, state, sub_formula2)
            }
            AlethicFormula::Or(sub_formula1, sub_formula2) => {
                self.evaluate_alethic_formula_at_state(frame, state, sub_formula1)
                    || self.evaluate_alethic_formula_at_state(frame, state, sub_formula2)
            }
            AlethicFormula::Modal(operator, sub_formula) => match operator {
                ModalOperator::Box => {
                     if let Some(accessible_states) = frame.accessibility.get(state) {
                        accessible_states
                            .iter()
                            .all(|next_state| self.evaluate_alethic_formula_at_state(frame, next_state, sub_formula))
                    } else {
                        false
                    }
                }

                 ModalOperator::Diamond => {
                     if let Some(accessible_states) = frame.accessibility.get(state) {
                        accessible_states
                            .iter()
                            .any(|next_state| self.evaluate_alethic_formula_at_state(frame, next_state, sub_formula))
                    } else {
                        false
                    }
                }
            }
         }
     }

    fn evaluate_alethic_formula(&self, formula: &AlethicFormula) -> bool {
         for frame in &self.frames {
            for state in &frame.states {
                if !self.evaluate_alethic_formula_at_state(frame, state, formula) {
                    return false;
                }
            }
        }
        true    
    }

     fn evaluate_epistemic_formula_at_state(
        &self,
        frame: &KripkeFrame,
        state: &str,
        formula: &EpistemicFormula,
        agent: &Agent,
    ) -> bool {
        match formula {
            EpistemicFormula::Atom(prop) => frame.states.contains(&prop.to_string()),
            EpistemicFormula::Not(sub_formula) => {
                 !self.evaluate_epistemic_formula_at_state(frame, state, sub_formula, agent)    
            }
            EpistemicFormula::And(sub_formula1, sub_formula2) => {
                 self.evaluate_epistemic_formula_at_state(frame, state, sub_formula1, agent)
                    && self.evaluate_epistemic_formula_at_state(frame, state, sub_formula2, agent)     
            }
            EpistemicFormula::Or(sub_formula1, sub_formula2) => {
                self.evaluate_epistemic_formula_at_state(frame, state, sub_formula1, agent)
                    || self.evaluate_epistemic_formula_at_state(frame, state, sub_formula2, agent)    
            }
            EpistemicFormula::Modal(operator, sub_formula, formula_agent) => {
                if formula_agent == agent {
                    match operator {
                        ModalOperator::Box => {
                            if let Some(accessible_states) = frame.accessibility.get(state) {
                                accessible_states.iter().all(|next_state| {
                                    self.evaluate_epistemic_formula_at_state(
                                        frame,
                                        next_state,
                                        sub_formula,
                                        formula_agent,
                                    )
                                })
                            } else {
                                false
                            }
                        }    
                    }   
                }
            }
        }     
    }
}

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
                        ModalOperator::Diamond => {
                            if let Some(accessible_states) = frame.accessibility.get(state) {
                                accessible_states.iter().any(|next_state| {
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
                } else {
                    // The formula is not about the current agent, so it is vacuously true
                    true
                }
            }
        }     
    }
    
    fn evaluate_epistemic_formula(&self, formula: &EpistemicFormula, agent: &Agent) -> bool {
        for frame in &self.frames {
            for state in &frame.states {
                if !self.evaluate_epistemic_formula_at_state(frame, state, formula, agent) {
                    return false;
                }
            }
        }
        true   
    }

    fn evaluate_doxastic_formula_at_state(
        &self,
        frame: &KripkeFrame,
        state: &str,
        formula: &DoxasticFormula,
        agent: &Agent,
    ) -> bool {
        match formula {
            DoxasticFormula::Atom(prop) => frame.states.contains(&prop.to_string()),
            DoxasticFormula::Not(sub_formula) => {
                !self.evaluate_doxastic_formula_at_state(frame, state, sub_formula, agent)    
            }
            DoxasticFormula::And(sub_formula1, sub_formula2) => {
                self.evaluate_doxastic_formula_at_state(frame, state, sub_formula1, agent)
                    && self.evaluate_doxastic_formula_at_state(frame, state, sub_formula2, agent)    
            }
            DoxasticFormula::Or(sub_formula1, sub_formula2) => {
                self.evaluate_doxastic_formula_at_state(frame, state, sub_formula1, agent)
                    || self.evaluate_doxastic_formula_at_state(frame, state, sub_formula2, agent)        
            }
            DoxasticFormula::Modal(operator, sub_formula, formula_agent) => {
                 if formula_agent == agent {
                    match operator {
                        ModalOperator::Box => {
                            if let Some(accessible_states) = frame.accessibility.get(state) {
                                accessible_states.iter().all(|next_state| {
                                    self.evaluate_doxastic_formula_at_state(
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
                        ModalOperator::Diamond => {
                            if let Some(accessible_states) = frame.accessibility.get(state) {
                                accessible_states.iter().any(|next_state| {
                                    self.evaluate_doxastic_formula_at_state(
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
                } else {
                    // The formula is not about the current agent, so it is vacuously true
                    true
                }
            }
        }    
    }

    fn evaluate_doxastic_formula(&self, formula: &DoxasticFormula, agent: &Agent) -> bool {
        for frame in &self.frames {
            for state in &frame.states {
                if !self.evaluate_doxastic_formula_at_state(frame, state, formula, agent) {
                    return false;
                }
            }
        }
        true    
    }

    fn evaluate_temporal_formula_at_state(
        &self,
        frame: &KripkeFrame,
        state: &str,
        formula: &TemporalFormula,
    ) -> bool {
        match formula {
            TemporalFormula::Atom(prop) => frame.states.contains(&prop.to_string()), 
            TemporalFormula::Not(sub_formula) => {
                 !self.evaluate_temporal_formula_at_state(frame, state, sub_formula)    
            }
            TemporalFormula::And(sub_formula1, sub_formula2) => {
                self.evaluate_temporal_formula_at_state(frame, state, sub_formula1)
                    && self.evaluate_temporal_formula_at_state(frame, state, sub_formula2)    
            }
            TemporalFormula::Or(sub_formula1, sub_formula2) => {
                self.evaluate_temporal_formula_at_state(frame, state, sub_formula1)
                    || self.evaluate_temporal_formula_at_state(frame, state, sub_formula2)    
            }
            TemporalFormula::Modal(operator, sub_formula) => match operator {
                TemporalOperator::Future => {
                     if let Some(accessible_states) = frame.accessibility.get(state) {
                         accessible_states
                            .iter()
                            .any(|next_state| self.evaluate_temporal_formula_at_state(frame, next_state, sub_formula))    
                     } else {
                        false
                    }    
                }
                TemporalOperator::Past => {
                    if let Some(accessible_states) = frame.accessibility.get(state) {
                        accessible_states
                            .iter()
                            .any(|next_state| self.evaluate_temporal_formula_at_state(frame, next_state, sub_formula))    
                    } else {
                        false
                    }        
                }
            },
        }
    }

    fn evaluate_temporal_formula(&self, formula: &TemporalFormula) -> bool {
        for frame in &self.frames {
            for state in &frame.states {
                if !self.evaluate_temporal_formula_at_state(frame, state, formula) {
                    return false;
                }
            }
        }
        true    
    }

    fn evaluate_deontic_formula_at_state(
        &self,
        frame: &KripkeFrame,
        state: &str,
        formula: &DeonticFormula,
    ) -> bool {
        match formula {
            DeonticFormula::Atom(prop) => frame.states.contains(&prop.to_string()),
            DeonticFormula::Not(sub_formula) => {
                !self.evaluate_deontic_formula_at_state(frame, state, sub_formula)    
            }
            DeonticFormula::And(sub_formula1, sub_formula2) => {
                self.evaluate_deontic_formula_at_state(frame, state, sub_formula1)
                    && self.evaluate_deontic_formula_at_state(frame, state, sub_formula2)    
            }
            DeonticFormula::Or(sub_formula1, sub_formula2) => {
                self.evaluate_deontic_formula_at_state(frame, state, sub_formula1)
                    || self.evaluate_deontic_formula_at_state(frame, state, sub_formula2)    
            }
            DeonticFormula::Modal(operator, sub_formula) => match operator {
                DeonticOperator::O => {
                    if let Some(accessible_states) = frame.accessibility.get(state) {
                        accessible_states
                            .iter()
                            .any(|next_state| self.evaluate_deontic_formula_at_state(frame, next_state, sub_formula))    
                    } else {
                        false
                    }   
                }
            },
        }    
    }

    fn evaluate_deontic_formula(&self, formula: &DeonticFormula) -> bool {
        for frame in &self.frames {
            for state in &frame.states {
                if !self.evaluate_deontic_formula_at_state(frame, state, formula) {
                    return false;
                }
            }
        }
        true
    }       
}

fn main() {
    // Example usage
    let mut model = KripkeModel::new(); 

    // Define states and accessibility relations for a Kripke frame
    let frame1_states: HashSet<String> = ["s1".to_string(), "s2".to_string()].iter().cloned().collect();
    let frame1_accessibility: HashMap<String, HashSet<String>> =
        [("s1".to_string(), ["s2".to_string()].iter().cloned().collect())].iter().cloned().collect();

    // Add the Kripke frame to the model
    model.frames.push(KripkeFrame {
        states: frame1_states,
        accessibility: frame1_accessibility,
    });

    // Define an Alethic logic formula: ◇(p ∧ q) → (◇p ∧ ◇q)
    let alethic_formula = AlethicFormula::Or(
    Box::new(AlethicFormula::Modal(
        ModalOperator::Diamond,
        Box::new(AlethicFormula::And(
            Box::new(AlethicFormula::Atom(Proposition::Atom("p".to_string()))),
            Box::new(AlethicFormula::Atom(Proposition::Atom("q".to_string()))),
        )),
    )),
    Box::new(AlethicFormula::And(
        Box::new(AlethicFormula::Modal(
            ModalOperator::Diamond,
            Box::new(AlethicFormula::Atom(Proposition::Atom("p".to_string()))),
        )),
        Box::new(AlethicFormula::Modal(
            ModalOperator::Diamond,
            Box::new(AlethicFormula::Atom(Proposition::Atom("q".to_string()))),
            )),
        )),
    );

    // Evaluate the Alethic formula in the Kripke model
    let result_alethic = model.evaluate_alethic_formula(&alethic_formula);
    println!("Does the Alethic logic formula hold in the Kripke model? {}", result_alethic);

    // Define an Epistemic logic formula: [K1]p → K1[p]
    let epistemic_formula = EpistemicFormula::Modal(
        ModalOperator::Box,
        Box::new(EpistemicFormula::Atom(Proposition::Atom("p".to_string()))),
        Agent::Agent1,
    );

    
}

use std::collections::HashMap;

#[derive(Debug)]
enum VonNeumann {
    Set(Vec<VonNeumann>),
}

impl VonNeumann {
    // Construct the Von Neumann ordinal for a given natural number n
    fn ordinal(n: usize) -> VonNeumann {
        let mut result = Vec::with_capacity(n + 1);
        for i in 0..=n {
            result.push(VonNeumann::ordinal(i));
        }
        VonNeumann::Set(result)
    }    

    // Take the union of two Von Neumann ordinals
    fn union(a: &VonNeumann, b: &VonNeumann) -> VonNeumann {
        match (a, b) {
            (VonNeumann::Set(set_a), VonNeumann::Set(set_b)) => {
                let mut result = set_a.clone();
                result.extend_from_slice(set_b);
                VonNeumann::Set(result)
            }
        }    
    }
}

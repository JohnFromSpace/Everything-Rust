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

     // Take the intersection of two Von Neumann ordinals
    fn intersection(a: &VonNeumann, b: &VonNeumann) -> VonNeumann {
        match (a, b) {
            (VonNeumann::Set(set_a), VonNeumann::Set(set_b)) => {
                let result: Vec<VonNeumann> = set_a
                    .iter()
                    .filter(|element| set_b.contains(element))
                    .cloned()
                    .collect();
                VonNeumann::Set(result)
            }
        }    
    }

    // Return the power set of a Von Neumann ordinal
    fn power_set(a: &VonNeumann) -> VonNeumann {
        match a {
            VonNeumann::Set(set_a) => {
                let mut result = Vec::new();
                for subset in 0..(1 << set_a.len()) {
                    let subset_elements: Vec<VonNeumann> = set_a
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| (subset >> i) & 1 == 1)
                        .map(|(_, element)| element.clone())
                        .collect();
                    result.push(VonNeumann::Set(subset_elements));
                }
                VonNeumann::Set(result)
            }
        }    
    }

    // Check if one Von Neumann ordinal is a subset of another
    fn is_subset(a: &VonNeumann, b: &VonNeumann) -> bool {
        match (a, b) {
            (VonNeumann::Set(set_a), VonNeumann::Set(set_b)) => {
                set_a.iter().all(|element| set_b.contains(element))
            }
        }    
    }
}

fn main() {
    // Example usage:
    let ordinal_3 = VonNeumann::ordinal(3);
    println!("Ordinal 3: {:?}", ordinal_3);

    let ordinal_5 = VonNeumann::ordinal(5);
    println!("Ordinal 5: {:?}", ordinal_5);

    let union_result = VonNeumann::union(&ordinal_3, &ordinal_5);
    println!("Union of Ordinal 3 and Ordinal 5: {:?}", union_result);

    let intersection_result = VonNeumann::intersection(&ordinal_3, &ordinal_5);
    println!("Intersection of Ordinal 3 and Ordinal 5: {:?}", intersection_result);
}

// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = magazine.iter().fold(HashMap::new(), |mut words, str| {
        *words.entry(str).or_insert(0) += 1;
        words
    });
    
    let note_words = note.iter().fold(HashMap::new(), |mut words, str| {
        *words.entry(str).or_insert(0) += 1;
        words
    });
    
    note_words.iter().all(&|(w, count)| magazine_words.get(w).unwrap_or(&0) >= count)
}

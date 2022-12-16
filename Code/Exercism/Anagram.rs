use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let old_word = word.to_lowercase();
    let mut new_word = old_word.chars().collect::<Vec<char>>();
    new_word.sort_unstable();
    possible_anagrams
        .iter()
        .fold(HashSet::new(), |mut acc, val| {
            let old_val = val.to_lowercase();
            let mut new_val = old_val.chars().collect::<Vec<char>>();
            new_val.sort_unstable();
            if new_word == new_val && old_word != old_val {
                acc.insert(*val);
            };
            acc
        })
}

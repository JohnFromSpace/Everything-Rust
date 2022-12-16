pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }
    
     if minefield[0].is_empty() {
        return vec![String::new()];
    }
    
    let width = minefield[0].len();
    let mut result = Vec::new();
    
    for (y, &line) in minefield.iter().enumerate() {
        
    }
    
}

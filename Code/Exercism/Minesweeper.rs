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
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                result.push('*');
                continue;
            }
            
            let mut neighbour_mines = minefield
                .iter()
                .take(y + 2)
                .skip((y as isize - 1).max(0) as usize)
                .flat_map(|&line| {
                    line.chars()
                        .take(x + 2)
                        .skip((x as isize - 1). max(0) as usize)
                })
                .filter(|&chr| chr == '*')
                .count();
     
            if neighbour_mines > 0 {
                result.push(format!("{}", neighbour_mines).chars().nth(0).unwrap());
            } 
            
            else {
                result.push(' ');
            }
    }
    
}

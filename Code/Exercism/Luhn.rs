/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }
    
    let adjust = |(i, n)| -> u32 {
        if i % 2 == 0 {
            n
        }
        else {
            let new_number = n * 2;
            if new_number >= 10 {
                new_number - 9
            }
            else {
                new_number
            }
        }
    };
    
    
}

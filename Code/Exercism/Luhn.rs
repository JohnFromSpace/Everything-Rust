/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }
    
}

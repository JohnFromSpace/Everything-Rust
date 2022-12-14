#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    fn find_window<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a.is_empty() || b.windows(a.len()).any(|v| a == v)
    }
    
    if a.len() == b.len() && a == b {
        Comparison::Equal
    }
    else if a.len() < b.len() && find_window(a, b) {
        Comparison::Sublist
    }
    else if find_window(b, a) {
        Comparison::Superlist
    }
    else {
        Comparison::Unequal
    }
}

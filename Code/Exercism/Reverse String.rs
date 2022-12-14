pub fn reverse(input: &str) -> String {
    let mut answer = String::new();
    for i in input.chars().rev() {
        answer.push(i)
    }
    answer
}

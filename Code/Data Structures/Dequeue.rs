use std::collections::VecDeque;

fn main() {
    // Create a new empty queue
    let mut queue = VecDeque::new();

    // Enqueue elements
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // Dequeue elements
    while let Some(front) = queue.pop_front() {
        println!("Dequeued: {}", front);
    }
}

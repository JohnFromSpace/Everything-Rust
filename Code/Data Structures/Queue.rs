fn main() {
    // Create a new empty queue
    let mut queue = Vec::new();

    // Enqueue elements
    queue.push(1);
    queue.push(2);
    queue.push(3);

    // Dequeue elements
    while let Some(front) = queue.get(0).cloned() {
        println!("Dequeued: {}", front);
        queue.remove(0);
    }
}

// Define a basic Stack structure
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // Create a new empty stack
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // Push an element onto the stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Pop an element from the stack
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Peek at the top element without removing it
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Get the size of the stack
    fn size(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    // Create a new stack of integers
    let mut stack = Stack::new();

    // Push elements onto the stack
    stack.push(3);
    stack.push(7);
    stack.push(11);

    // Peek at the top element
    match stack.peek() {
        Some(top) => println!("Top element: {}", top),
        None => println!("The stack is empty."),
    }

    // Pop elements from the stack
    while let Some(item) = stack.pop() {
        println!("Popped: {}", item);
    }

    // Check if the stack is empty
    println!("Is empty: {}", stack.is_empty());

    // Print the size of the stack
    println!("Stack size: {}", stack.size());
}

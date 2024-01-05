// Define a basic Node structure for the doubly linked list
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

// Define the DoublyLinkedList structure
#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> DoublyLinkedList<T> {
    // Create a new empty doubly linked list
    fn new() -> Self {
        DoublyLinkedList { head: None }
    }

    // Insert a new element at the beginning of the doubly linked list
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
            prev: None,
        });

        if let Some(mut head) = new_node.next.as_mut() {
            head.prev = Some(new_node.clone());
        }

        self.head = Some(new_node);
    }

    // Print the elements of the doubly linked list
    fn print_forward(&self) {
        let mut current = &self.head;

        print!("Doubly Linked List (Forward): ");
        while let Some(node) = current {
            print!("{} <-> ", node.data);
            current = &node.next;
        }
        println!("None");
    }

    // Print the elements of the doubly linked list in reverse order
    fn print_backward(&self) {
        let mut current = &self.head;

        // Move to the last node
        while let Some(node) = current {
            if node.next.is_none() {
                break;
            }
            current = &node.next;
        }

        print!("Doubly Linked List (Backward): ");
        while let Some(node) = current {
            print!("{} <-> ", node.data);
            current = &node.prev;
        }
        println!("None");
    }
}

fn main() {
    // Create a new doubly linked list
    let mut list = DoublyLinkedList::new();

    // Insert elements into the doubly linked list
    list.push(3);
    list.push(7);
    list.push(11);

    // Print the doubly linked list in forward and backward order
    list.print_forward();
    list.print_backward();
}

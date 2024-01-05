// Define a basic Node structure for the singly linked list
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define the LinkedList structure
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Insert a new element at the beginning of the linked list
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Print the elements of the linked list
    fn print(&self) {
        let mut current = &self.head;

        print!("Linked List: ");
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    // Create a new linked list
    let mut list = LinkedList::new();

    // Insert elements into the linked list
    list.push(3);
    list.push(7);
    list.push(11);

    // Print the linked list
    list.print();
}

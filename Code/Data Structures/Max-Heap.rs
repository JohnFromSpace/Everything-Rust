#[derive(Debug)]
struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    fn insert(&mut self, value: i32) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    fn extract_max(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let max_value = self.data[0];
            let last_index = self.data.len() - 1;

            self.data.swap(0, last_index);
            self.data.pop();

            self.heapify_down(0);

            Some(max_value)
        }
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;

            if self.data[index] > self.data[parent_index] {
                self.data.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len();

        while 2 * index + 1 < len {
            let left_child_index = 2 * index + 1;
            let right_child_index = 2 * index + 2;

            let mut largest = index;

            if self.data[left_child_index] > self.data[largest] {
                largest = left_child_index;
            }

            if right_child_index < len && self.data[right_child_index] > self.data[largest] {
                largest = right_child_index;
            }

            if largest != index {
                self.data.swap(index, largest);
                index = largest;
            } else {
                break;
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn main() {
    let mut max_heap = MaxHeap::new();

    max_heap.insert(10);
    max_heap.insert(5);
    max_heap.insert(20);
    max_heap.insert(8);
    max_heap.insert(15);

    println!("Max Heap: {:?}", max_heap);

    while let Some(max_value) = max_heap.extract_max() {
        println!("Extracted Max Value: {}", max_value);
        println!("Max Heap after extraction: {:?}", max_heap);
    }
}

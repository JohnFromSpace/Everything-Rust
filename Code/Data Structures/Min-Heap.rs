#[derive(Debug)]
struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn insert(&mut self, value: i32) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    fn extract_min(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let min_value = self.data[0];
            let last_index = self.data.len() - 1;

            self.data.swap(0, last_index);
            self.data.pop();

            self.heapify_down(0);

            Some(min_value)
        }
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;

            if self.data[index] < self.data[parent_index] {
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

            let mut smallest = index;

            if self.data[left_child_index] < self.data[smallest] {
                smallest = left_child_index;
            }

            if right_child_index < len && self.data[right_child_index] < self.data[smallest] {
                smallest = right_child_index;
            }

            if smallest != index {
                self.data.swap(index, smallest);
                index = smallest;
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
    let mut min_heap = MinHeap::new();

    min_heap.insert(10);
    min_heap.insert(5);
    min_heap.insert(20);
    min_heap.insert(8);
    min_heap.insert(15);

    println!("Min Heap: {:?}", min_heap);

    while let Some(min_value) = min_heap.extract_min() {
        println!("Extracted Min Value: {}", min_value);
        println!("Min Heap after extraction: {:?}", min_heap);
    }
}

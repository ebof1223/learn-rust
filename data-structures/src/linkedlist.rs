pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node { val, next: None }
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn push_back(&mut self, val: T) {
        let node = Some(Box::new(Node::new(val)));
        if self.head.is_none() {
            self.head = node;
        } else {
            let mut current = self.head.as_mut().unwrap();

            while let Some(ref mut next_node) = current.next {
                current = next_node;
            }
            current.next = node;
        }
    }
}

fn vec_to_linkedlist<T>(vector: Vec<T>) -> LinkedList<T> {
    let mut list: LinkedList<T> = LinkedList::new();
    let mut vector = vector.into_iter();

    while let Some(item) = vector.next() {
        list.push_back(item)
    }

    list
}

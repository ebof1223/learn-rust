pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node { val, next: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    pub fn push_back(&mut self, val: T) {
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

pub fn vec_to_linkedlist<T>(vector: Vec<T>) -> LinkedList<T> {
    let mut list: LinkedList<T> = LinkedList::new();
    let mut vector = vector.into_iter();
    while let Some(item) = vector.next() {
        list.push_back(item)
    }
    list
}

pub fn linkedlist_to_vec<T>(list: LinkedList<T>) -> Vec<T> {
    let mut vector = vec![];
    let mut node = list.head;
    while let Some(current) = node {
        vector.push(current.val);
        node = current.next;
    }
    vector
}

struct DblLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> DblLinkedList<T> {
    fn new() -> DblLinkedList<T> {
        DblLinkedList {
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, val: T) {
        let node = Box::new(Node {
            val,
            next: None,
            prev: None,
        });
        match self {
            DblLinkedList { head: None, .. } => {
                self.head = Some(node);
                todo!()
            }

            DblLinkedList {
                head: Some(node),
                tail: None,
            } => {
                todo!()
            }

            DblLinkedList { tail: Some(_), .. } => {
                todo!()
            }
        }
    }
}

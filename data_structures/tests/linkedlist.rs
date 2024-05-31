#[cfg(test)]
mod tests {

    use data_structures::linkedlist;

    #[test]
    fn linked_list_int() {
        let mut list = linkedlist::LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        let head = list.head.unwrap();
        assert_eq!(head.val, 1);
        let mut node = head.next.unwrap();
        assert_eq!(node.val, 2);
        let mut count: i32 = 3;
        while let Some(current) = node.next {
            assert_eq!(current.val, count);
            count += 1;
            node = current;
        }
    }

    #[test]
    fn vec_to_list() {
        let list = linkedlist::vec_to_linkedlist(vec![1, 2, 3, 4, 5]);

        let head = list.head.unwrap();
        assert_eq!(head.val, 1);
        let mut node = head.next.unwrap();
        assert_eq!(node.val, 2);
        let mut count: i32 = 3;
        while let Some(current) = node.next {
            assert_eq!(current.val, count);
            count += 1;
            node = current;
        }
    }
    #[test]
    fn list_to_vec() {
        let mut vector = vec![];
        let mut list = linkedlist::LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        let head = list.head.unwrap();
        vector.push(head.val);
        let mut node = head.next.unwrap();
        vector.push(node.val);

        while let Some(current) = node.next {
            vector.push(current.val);
            node = current;
        }

        let mut count = 1;
        for item in vector.into_iter() {
            assert_eq!(item, count);
            count += 1;
        }
    }
}

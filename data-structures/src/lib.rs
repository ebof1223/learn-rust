mod binary_rev;
mod linkedlist;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_int() {
        let mut list: linkedlist::LinkedList<i32> = linkedlist::LinkedList::new();
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
}

pub struct LinkedList<I> {
    pub head: Option<Box<LinkedListNode<I>>>,
}
pub struct LinkedListNode<I> {
    data: I,
    pub next: Option<Box<LinkedListNode<I>>>
}
impl<I> LinkedListNode<I> {
    pub fn get_data(&self) -> &I { &self.data }
}

impl<I: Clone> LinkedList<I> {
    pub fn new() -> Self { Self { head: None } }


    fn get_node_mut(&mut self, position: usize) -> Option<&mut LinkedListNode<I>> {
        let Some(mut node) = self.head.as_deref_mut() else { return None };
        for _ in 0..position {
            node = match &mut node.next {
                None => return None,
                Some(next) => next
            };
        }
        Some(node)
    } 

    /// Put item onto the front of the linked list
    pub fn push_front(&mut self, item: I) {
        // Take the value from head, construct new node with old head as next
        let next = self.head.take();
        self.head = Some(Box::new(LinkedListNode { data: item, next } ));
    }

    /// Insert item at position \
    /// Return error if the list is not large enough to accomodate this item
    pub fn insert_at(&mut self, item: I, position: usize) -> Result<(), ()> {
        if position == 0 {
            self.push_front(item);
            return Ok(())
        }

        // Go to position-1, take value from next, construct new node and insert
        let Some(prev) = self.get_node_mut(position-1) else { return Err(()) };

        let next = prev.next.take();
        let new = LinkedListNode { data: item, next };
        prev.next = Some(Box::new(new));
        Ok(())
    }

    /// Remove and return the node at the provided position
    pub fn remove(&mut self, position: usize) -> Option<Box<LinkedListNode<I>>> {
        if self.head.is_none() {
            return None
        }

        // Special case for the head
        if position == 0 {
            let mut removed = self.head.take().unwrap(); // unwrap is safe because above check
            self.head = removed.next.take();
            return Some(removed)
        }

        // Go to position-1, take value of next, set next to next's next, return removed data
        let Some(prev) = self.get_node_mut(position-1) else { return None };
        let Some(mut removed) = prev.next.take() else { return None };
        prev.next = removed.next.take();
        Some(removed)
    }

    /// Remove second node, swap with first node, insert first node in old place of second node
    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), ()> {
        if a == b { return Ok(()) }

        let first = a.min(b);
        let second = a.max(b);
        let Some(mut second_node) = self.remove(second) else { return Err(()) };
        
        // Swap first with second, get first
        // Special case for head
        let mut first_node = if first == 0 {
            let Some(mut head) = self.head.take() else { return Err(()) };
            second_node.next = head.next.take();
            let first_node = head;
            self.head = Some(second_node);
            first_node
        } else {
            let Some(mut node) = self.head.as_mut() else { return Err(()) };
            //fast forward node to first-1
            for _ in 0..first-1 {
                node = match &mut node.next {
                    None => return Err(()),
                    Some(next) => next
                }
            }
            //swap first with second
            let Some(mut first_node) = node.next.take() else { return Err(()) };
            second_node.next = first_node.next.take();
            node.next = Some(second_node);
            first_node
        };

        // Insert first where second was
        let Some(mut node) = self.head.as_mut() else { return Err(()) };
        //fast forward node to second-1
        for _ in 0..second-1 {
            node = match &mut node.next {
                None => return Err(()),
                Some(next) => next
            }
        }
        first_node.next = node.next.take();
        node.next = Some(first_node);
        Ok(())
    }




    /// Utility for testing so I can use vec comparison
    pub fn as_vec(&self) -> Vec<I> {
        let mut ret = vec![];
        let mut node = &self.head;
        loop {
            match &node {
                None => return ret,
                Some(n) => {
                    ret.push(n.data.clone());
                    node = &n.next;
                }
            }
        }
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_push_test() {
        let mut l = LinkedList::new();
        assert!(l.head.is_none());

        // Push a single element
        l.push_front(10);
        assert_eq!(l.as_vec(), [10]);

        // Push multiple elements
        l.push_front(-5);
        assert_eq!(l.as_vec(), [-5, 10]);

        // Push extreme values
        l.push_front(i32::MAX);
        l.push_front(i32::MIN);
        assert_eq!(l.as_vec(), [i32::MIN, i32::MAX, -5, 10]);

        // Push to a non-empty list
        l.push_front(100);
        assert_eq!(l.as_vec(), [100, i32::MIN, i32::MAX, -5, 10]);
    }

    #[test]
    fn list_insert_test() {
        let mut l = LinkedList::new();

        // Fail to insert at invalid positions
        assert!(l.insert_at(10, 1).is_err());  // Insert at index 1 when the list is empty
        assert!(l.insert_at(-5, 22).is_err()); // Index out of bounds

        // Insert at head
        assert!(l.insert_at(-6, 0).is_ok());
        assert_eq!(l.as_vec(), [-6]);

        // Insert in front of head
        assert!(l.insert_at(4, 0).is_ok());
        assert_eq!(l.as_vec(), [4, -6]);

        // Insert at valid positions
        assert!(l.insert_at(12, 1).is_ok());
        assert_eq!(l.as_vec(), [4, 12, -6]);

        // Insert past the end (should fail)
        assert!(l.insert_at(13, 4).is_err());

        // Insert at the end (valid)
        assert!(l.insert_at(-1, 3).is_ok());
        assert_eq!(l.as_vec(), [4, 12, -6, -1]);

        // Insert in the middle
        assert!(l.insert_at(2, 2).is_ok());
        assert_eq!(l.as_vec(), [4, 2, 12, -6, -1]);
    }

    #[test]
    fn list_remove_test() {
        let mut l = LinkedList::new();
        let v = [5, 4, -3, 9];
        for n in v.iter().rev() {
            l.push_front(*n);
        }
        assert_eq!(l.as_vec(), v);

        // Remove from middle
        assert_eq!(l.remove(2).map(|n| n.get_data().clone()), Some(-3));
        assert_eq!(l.as_vec(), [5, 4, 9]);

        // Remove from the end
        assert_eq!(l.remove(2).map(|n| n.get_data().clone()), Some(9));
        assert_eq!(l.as_vec(), [5, 4]);

        // Remove from the start
        assert_eq!(l.remove(0).map(|n| n.get_data().clone()), Some(5));
        assert_eq!(l.as_vec(), [4]);

        // Attempt to remove from an empty list
        assert!(l.remove(0).is_some());
        assert_eq!(l.as_vec(), []);

        // Edge case: removing from an empty list
        assert!(l.remove(0).is_none());
    }

    #[test]
    fn list_swap_test() {
        let mut l = LinkedList::new();
        let v = [5, 4, -3, 9];
        for n in v.iter().rev() {
            l.push_front(*n);
        }
        assert_eq!(l.as_vec(), v);

        // Swap valid elements
        l.swap(0, 1).expect("Failed Swap");
        assert_eq!(l.as_vec(), vec![4, 5, -3, 9]);

        l.swap(1, 0).expect("Failed Swap");
        assert_eq!(l.as_vec(), vec![5, 4, -3, 9]);

        l.swap(1, 3).expect("Failed Swap");
        assert_eq!(l.as_vec(), vec![5, 9, -3, 4]);

        l.swap(0, 3).expect("Failed Swap");
        assert_eq!(l.as_vec(), vec![4, 9, -3, 5]);

        l.swap(1, 2).expect("Failed Swap");
        assert_eq!(l.as_vec(), vec![4, -3, 9, 5]);

        // Swap invalid indices
        assert!(l.swap(0, 5).is_err());  // Out of bounds
        assert!(l.swap(5, 2).is_err());  // Out of bounds

        // Swap with itself (no-op)
        l.swap(2, 2).expect("Failed Swap");
        assert_eq!(l.as_vec(), vec![4, -3, 9, 5]);
    }

    #[test]
    fn list_edge_cases() {
        let mut l = LinkedList::new();

        // Test remove on an empty list
        assert!(l.remove(0).is_none());

        // Test swap on an empty list
        assert!(l.swap(0, 1).is_err());

        // Test inserting into an empty list
        assert!(l.insert_at(10, 0).is_ok());
        assert_eq!(l.as_vec(), [10]);

        // Test push after removal
        l.remove(0);
        assert!(l.head.is_none());
        l.push_front(20);
        assert_eq!(l.as_vec(), [20]);

        // Test insert out of bounds when the list has only one element
        assert!(l.insert_at(30, 2).is_err());  // Out of bounds

        // Test insert at valid index
        assert!(l.insert_at(15, 1).is_ok());
        assert_eq!(l.as_vec(), [20, 15]);

        // Test swap on a single-element list (no-op)
        l.swap(0, 0).expect("Failed Swap");
        assert_eq!(l.as_vec(), [20]);
    }
}

#![allow(unused)]

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Creates an empty list.
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// Add a new head in the list
    pub fn cons(&mut self, item: T) {
        let new_node = Box::new(Node{
            val: item,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// Remove the head of the list, return the head element
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    /// Return a reference to the head of the list
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.val
        })
    }

    /// Return a mutable reference to the head of the list
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.val
        })
    }

}


impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

// Iterator for a linked list
pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_list_api() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Add some elements to the list
        list.cons(1);
        list.cons(2);
        list.cons(3);

        // Check removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        let mut new_list = LinkedList::new();
        new_list.cons(1);
        assert_eq!(list, new_list);

        // Add more elements
        list.cons(4);
        list.cons(5);

        // Check removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check axhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list, LinkedList::new());
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        // Add some elements
        list.cons(1);
        list.cons(2);
        list.cons(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // Test mutability
        list.peek_mut().map(|value| {
            *value = 42;
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn test_into_iter() {
        let mut list = LinkedList::new();
        list.cons(1);
        list.cons(2);
        list.cons(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}

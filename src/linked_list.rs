#![allow(unused)]
// Reference: https://rust-unofficial.github.io/too-many-lists/second.html

use serde::ser::{Serializer, SerializeSeq};
use serde::de::{Deserializer, Visitor, SeqAccess};
use serde::{Serialize, Deserialize};

use std::fmt;
use std::marker::PhantomData;


#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

// Serialization
impl<T> Serialize for LinkedList<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

// Deserialization
struct MyListVisitor<T> {
    marker: PhantomData<fn() -> LinkedList<T>>
}

impl<T> MyListVisitor<T> {
    fn new() -> Self {
        MyListVisitor {
            marker: PhantomData
        }
    }
}

impl<'de, T> Visitor<'de> for MyListVisitor<T>
where
    T: Deserialize<'de>
{
    type Value = LinkedList<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter)
                 -> fmt:: Result {
        formatter.write_str("linked list")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>
    {
        let mut list = LinkedList::new();
        while let Some(item) = seq.next_element()? {
            list.cons(item);
        }
        Ok(list.reverse())
    }

}

impl<'de, T> Deserialize<'de> for LinkedList<T>
where
    T: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(MyListVisitor::new())
    }
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

    pub fn len(&self) -> usize {
        let mut len = 0;
        for _ in self.iter() {
            len += 1;
        }
        len
    }

    pub fn reverse(self) -> Self {
        let mut reversed = LinkedList::new();
        for item in self.into_iter() { reversed.cons(item) }
        reversed
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

// Iterators for linked lists
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

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(& self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for item in iter { list.cons(item); }
        list.reverse()
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_serialization() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let serialization = serde_json::to_string(&list).unwrap();
        assert_eq!(serialization, "[]"); // serialize like arrays

        list.cons(1);
        list.cons(2);
        list.cons(3);
        let serialization = serde_json::to_string(&list).unwrap();
        assert_eq!(serialization, "[3,2,1]"); // serialize like arrays

        // deserialize to vec
        let vec_deserialization: Vec<i32> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(vec_deserialization, vec![3,2,1]); // deserialize to vec

        let list_deserialization: LinkedList<i32> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(list_deserialization, list); // deserialize to list
    }

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

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.cons(1);
        list.cons(2);
        list.cons(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.cons(1);
        list.cons(2);
        list.cons(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);

        list.cons(1);
        list.cons(2);
        list.cons(3);

        assert_eq!(list.len(), 3);
    }
}

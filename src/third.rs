// A persistent singly-linked stack implementation.
//
// Improvements:
//     - persistent
//     - immutable
//     - generic over contained type
//
// Inspired by:
// https://rust-unofficial.github.io/too-many-lists/second.html
//
// We want to be able to do the following with out linked lists, much as we would expect to be able
// to do in functional language with garbage collection:
//
// list1 = A -> B -> C -> D
// list2 = tail(list1) = B -> C -> D
// list3 = push(list2, X) = X -> B -> C -> D
//
// and end up with memory looking like:
//
// list1 -> A ---+
//               |
//               v
// list2 ------> B -> C -> D
//               ^
//               |
// list3 -> X ---+
//
// To do this in Rust, we do reference counting using `Rc`.

use std::rc::Rc;

//////////////////////////////////////////////////////////////////////////////
// Data Structures

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

//////////////////////////////////////////////////////////////////////////////
// Implementation

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Option<List<T>> {
        self.head.as_ref().map(|rc_node| List {
            head: rc_node.next.clone(),
        })
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|rc_node| &rc_node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // If we're looking at the last ref counted pointer to this node, then we can extract
            // it using take() and drop it. Otherwise, we just stop since someone else holds a
            // valid pointer to it.
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|next_node| &**next_node);
            &node.elem
        })
    }
}

//////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);
        assert!(list.tail().is_none());

        let list2 = list.append(0).append(1).append(2);
        assert_eq!(list2.head(), Some(&2));
        assert!(!list2.tail().is_none());
        // can't directly compare tail to another list yet...
    }

    #[test]
    fn iter() {
        let list = List::new().append(0).append(1).append(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
    }

    // If the Drop impl for List is commented out above, this test will cause the stack to
    // overflow.
    #[test]
    fn test_drop() {
        let mut list = List::new();
        for i in 0..1000000 {
            list = list.append(i);
        }
        let list2 = list.append(42);
        assert_eq!(list2.head(), Some(&42));
        list = list.append(1024);
        assert_eq!(list.head(), Some(&1024));
        // list and list2 share a tail and are both dropped here
    }
}

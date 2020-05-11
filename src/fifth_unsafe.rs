// An unsafe queue implementation
//
// Features:
//     - mutable queue API
//     - fast push and pop
//     - uses raw pointers
//
// Inspired by:
// https://rust-unofficial.github.io/too-many-lists/second.html

//////////////////////////////////////////////////////////////////////////////
// Data structures
//
// Linked list layout:
// [] = stack
// () = heap
// [ptr] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (C, None)
// [ptr] ----------------------------------------^
//

use std::ptr;

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    // Push an element onto the tail of the queue.
    //
    // [ptr] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (C, None)
    // [ptr] ---------------------------------------^
    //
    // becomes
    //
    // [ptr] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (C, Some(ptr)) -> (x, None)
    // [ptr] ---------------------------------------------------------^
    //
    pub fn push(&mut self, x: T) {
        let mut new_node = Box::new(Node {
            elem: x,
            next: None, // new tail doesn't point to anything
        });

        // Take a pointer to the node in the box, coerce it to a raw pointer. Box has a
        // stable address, even when moved, so this is OK as long as we are careful not to
        // use the raw pointer after the Box is dropped.
        let raw_new_node: *mut _ = &mut *new_node;

        // Instead of take()'ing self.tail, we branch on whether it is null or not
        if self.tail.is_null() {
            // Case: empty queue. assign to self.head
            self.head = Some(new_node);
        } else {
            // Case: non-empty queue. Set next field in the node pointed to by current
            // self.tail to the new boxed node.
            unsafe {
                (*self.tail).next = Some(new_node); // UNSAFE dereference
            }
        }

        self.tail = raw_new_node;
        // When we pop, we have to make sure we null out this pointer because at that point
        // the Box could be dropped (it's out of our control).
    }

    // Pop an element off of the head of the queue.
    //
    // [ptr] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (C, None)
    // [ptr] ---------------------------------------^
    //
    // becomes
    //
    // [ptr] -> (B, Some(ptr)) -> (C, None)
    // [ptr] ---------------------^
    //
    // and A is returned (if the queue was non-empty).
    pub fn pop(&mut self) -> Option<T> {
        // self.head :: Link === Option<Box<Node>>
        // self.tail :: *mut Node
        self.head.take().map(|box_node| {
            if box_node.next.is_none() {
                self.tail = ptr::null_mut();
            }
            self.head = box_node.next;
            box_node.elem
        })
    }
}

//////////////////////////////////////////////////////////////////////////////
// Iteration

pub struct IntoIter<T>(Queue<T>);

impl<T> Queue<T> {
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

impl<T> Queue<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|ref_box_node| &**ref_box_node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|ref_node| {
            self.next = ref_node.next.as_ref().map(|ref_box_node| &**ref_box_node);
            &ref_node.elem
        })
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests

#[cfg(test)]
mod test {
    use super::Queue;
    #[test]
    fn basics() {
        let mut queue = Queue::new();

        // Check empty queue behaves right
        assert_eq!(queue.pop(), None);

        // Populate queue
        queue.push(1);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), None);

        // Check that items are popped in FIFO order
        queue.push(2);
        queue.push(3);
        queue.push(4);
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        let mut iter = queue.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        // cannot borrow from moved value!
        // queue.push(42);
    }

    #[test]
    fn iter() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        // Push some more onto the queue and iterate again
        queue.push(4);
        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }
}

// A basic singly-linked stack implementation.
// Inspired by: https://rust-unofficial.github.io/too-many-lists/first.html

use std::mem;

//////////////////////////////////////////////////////////////////////////////
// Types
//
// Linked list layout:
// [] = stack
// () = heap
// [ptr] -> (elem A, ptr) -> (elem B, ptr) -> (elem C, *null*)

// A newtype around Link to hide implementation details of `Node`
pub struct List {
    head: Link,
}

// An ADT the represents a link list Link, either the link is empty (end of the list) or it is a
// pointer to a Node.
enum Link {
    Empty,
    More(Box<Node>),
}

// A list node is an integer and a link.
// TODO: parametrize type of 'elem'.
struct Node {
    elem: i32,
    next: Link,
}

//////////////////////////////////////////////////////////////////////////////
// Implementation

impl List {
    // return a new, empty list
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // push an integer onto the stack
    pub fn push(&mut self, x: i32) {
        let new_box_node = Box::new(Node {
            elem: x,
            // disassociate the old `self.head` from `self`, returning it's value and replace it
            // with a new Link::empty indicating end of list.
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_box_node);
    }

    // pop an integer from the stack, returning either Some(value) or None if
    // the stack is empty.
    //
    // Here the head of the list is matches on and replaced with Link::Empty. In case there is an
    // element to pop, the head is reassigned to point to the next element.
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            // replaced value is same as original
            Link::Empty => Option::None,
            // replaced value is a non-empty Link that we take the next
            // pointer from
            Link::More(node) => {
                self.head = node.next;
                return Some(node.elem);
            }
        }
    }
}

// A non-recursive Drop implementation so we don't blow the stack when
// dropping large lists.
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        // `boxed_node` is matches as `mut` so that we can extract and replace its `next` field.
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

// TESTS

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();

        // test pop on empty list
        assert_eq!(list.pop(), None);

        // push items onto the list
        list.push(0);
        list.push(1);
        list.push(2);

        // check the popped items
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        // push more onto the pre-popped list
        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(0));

        // test for exhaustion
        assert_eq!(list.pop(), None);
    }

    // If the Drop impl for List is commented out above, this test will cause the stack to
    // overflow.
    #[test]
    fn test_drop() {
        let mut list = List::new();
        for i in 0..1000000 {
            list.push(i);
        }
        // list is dropped
    }
}

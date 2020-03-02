// A basic singly-linked stack implementation.
// Inspired by: https://rust-unofficial.github.io/too-many-lists/first.html

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

// Implementation

impl List {
    // return a new, empty list
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // push an integer onto the given stack
    pub fn push(&mut self, x: i32) {
        let new_box_node = Box::new(Node {
            elem: x,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_box_node);
    }

    // pop an integer from the stack, returning either Some(value) or None if
    // the stack is empty.
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
}

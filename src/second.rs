// A better singly-linked stack implementation.
//
// Improvements:
//     - better API
//     - generic over contained type
//     - uses Option instread of custom isomorphic type
//
// Inspired by:
// https://rust-unofficial.github.io/too-many-lists/second.html

//////////////////////////////////////////////////////////////////////////////
// Data structures
//
// Linked list layout:
// [] = stack
// () = heap
// [ptr] -> (elem A, ptr) -> (elem B, ptr) -> (elem C, *null*)

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

//////////////////////////////////////////////////////////////////////////////
// Implementation

impl<T> List<T> {
    // return a new, empty list
    pub fn new() -> Self {
        List { head: None }
    }

    // push an integer onto the given stack
    pub fn push(&mut self, x: T) {
        let new_box_node = Box::new(Node {
            elem: x,
            // Option::take extracts the content and replaces it with a new None
            // equivalent to mem::replace(&mut self.head, None)
            next: self.head.take(),
        });
        self.head = Some(new_box_node);
    }

    // pop an integer from the stack, returning either Some(value) or None if
    // the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        // map a lambda over the content of self.head that includes self.head in its closure
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // peek at the element at the head of the list
    pub fn peek(&self) -> Option<&T> {
        // Option::as_ref :: &Option<N> -> Option<&N>
        // map consumes the &N leaving the original Optional content alone
        self.head.as_ref().map(|node| &node.elem)
    }

    // peek at the element at the head of the list, return a mutable ref
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

// A non-recursive Drop implementation so we don't blow the stack when
// dropping large lists.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Iteration

//
// IntoIter
//
// struct has a single List<T> field
pub struct IntoIter<T>(List<T>);

// Provide List<T> with a method for converting to an iterator
impl<T> List<T> {
    // into_iter consumes `self`, returning an `IntoIter<T>`
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    // type of thing being iterator through
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

//
// Iter
//
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // self needs to live at least as long as the iter. We elide the lifetimes.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            // remember: map<U, F>(self, f: F) -> Option<U>
            // turbofish operator ::<> lets us (partially) spec the generic types
            // or, more janky is to write: &**node in place of &node below
            next: self.head.as_ref().map::<&Node<T>, _>(|node| &node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // Why `Self::Item` instead of `&T`? Because otherwise compiler can't infer the lifetime of the
    // thing inside the returned Option
    //
    // Note: we don't technically need to take() `self.next` here since & is Copy.
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // self.next = node.next.map(|nnode| &nnode);
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}

//
// IterMut
//
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    // We must take() `self.next` here because &mut is not Copy.
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|next_node| &mut **next_node);
            &mut node.elem
        })
    }
}

//////////////////////////////////////////////////////////////////////////////
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

        // How about for strings?
        let mut str_list = List::new();
        assert_eq!(list.pop(), None);
        str_list.push("a".to_string());
        str_list.push("b".to_string());
        str_list.push("c".to_string());
        assert_eq!(str_list.pop(), Some("c".to_string()));
        assert_eq!(str_list.pop(), Some("b".to_string()));
        assert_eq!(str_list.pop(), Some("a".to_string()));
        assert_eq!(str_list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        // check that the head is empty
        assert_eq!(list.peek(), None);

        // push items onto the list
        list.push(0);
        list.push(1);
        list.push(2);

        // peek at the head
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.peek_mut(), Some(&mut 0));

        // type of x is &mut i32, the closure binds the name x to this and we
        // can mutate the dereferenced value. Specifying &mut x in the closure
        // argument would bind x to an already derefed and immutable value.
        list.peek_mut().map(|x| *x = 42); // mutate value inside the Option
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    // test the IntoIter iterator
    fn into_iter() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter_mut = list.iter_mut();
        // let mut iter_mut2 = list.iter_mut();  // this fails to compile!
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), None);
    }
}

// A basic mutable queue implementation (that doesn't end up working)
//
// Features:
//     - mutable queue API
//     - fast push and pop
//
// Inspired by:
// https://rust-unofficial.github.io/too-many-lists/second.html

//////////////////////////////////////////////////////////////////////////////
// Data structures
//
// Memory layout:
//
// [] = stack
// () = heap
// [ptr] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (C, None)
// [ptr] ----------------------------------------^
//

pub struct Queue<'a, T> {
    head: Link<T>,
    tail: WeakLink<'a, T>,
}

type Link<T> = Option<Box<Node<T>>>;
type WeakLink<'a, T> = Option<&'a mut Node<T>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> Queue<'a, T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    pub fn push(&'a mut self, x: T) {
        let new_node = Box::new(Node {
            elem: x,
            next: None, // new tail doesn't point to anything
        });

        let new_tail = match self.tail.take() {
            Some(last_node) => {
                // non-empty queue case
                last_node.next = Some(new_node);
                // last_node.next :: Option<Box<Node>>
                // last_node.next.as_mut() :: Option<&mut Box<node>>
                // &mut **mr_box_node :: &mut Node
                last_node
                    .next
                    .as_mut()
                    .map(|mr_box_node| &mut **mr_box_node)
            }
            None => {
                // empty queue case
                self.head = Some(new_node);
                self.head.as_mut().map(|mr_box_node| &mut **mr_box_node)
            }
        };

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|first_node| {
            let first_node_val = *first_node;
            self.head = first_node_val.next;

            if self.head.is_none() {
                self.tail = None;
            }

            first_node_val.elem
        })
    }
}

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

        // XXX doesn't work to borrow mutable queue more than once!
        // assert_eq!(queue.pop(), Some(1));
    }
}

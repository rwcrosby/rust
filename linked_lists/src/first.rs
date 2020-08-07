// First attempt at a linked list
// from https://rust-unofficial.github.io/too-many-lists/first-layout.html

use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    //! Implement a bunch of stuff - Inner block doc

    /// Create a new list - Outer block doc
    /// # This should a heading in markdown
    /// - A list item
    /// - Another list item
    /// ```
    /// // Code line
    /// ```
    pub fn new() -> Self{
        List {
            head: Link::Empty
        }
    }


    pub fn push(&mut self, elem: i32) {
        //! Push an element onto the list - Inner block doc

        let new_node = Box::new (
            Node {
                elem,
                next: mem::replace(&mut self.head, Link::Empty),
            }
        );

        self.head = Link::More(new_node);

    }

    /// And pop something as well
    pub fn pop(&mut self) -> Option<i32> {

        match mem::replace(&mut self.head, Link::Empty) {

            Link::Empty => None,

            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }

        }

    }

}

impl Drop for List {


    /// Loop through the list clearing the next pointers
    fn drop(&mut self) {

        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // `while let` == "do this thing until this pattern doesn't match"

        while let Link::More(mut boxed_node) = cur_link {

            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);

        }
    }
}

// Second attempt at a linked list
// from https://rust-unofficial.github.io/too-many-lists/second.html

/// Type for a list node
struct Node<T> {
    elem: T,
    next: Link<T>,
}

/// Type for a link in the list
type Link<T> = Option<Box<Node<T>>>;

/// Type for a list
pub struct List<T> {
    head: Link<T>,
}

/// Type for a destructive iterator
pub struct IntoIter<T>(List<T>);

/// Type for a non-desrtuctive iterator
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// Type for a non-destructive, mutable, iterator
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    //! Second implementation

    /// Create a new list
    pub fn new() -> Self{
        List {
            head: None
        }           
    }

    /// Push an element onto the list
    pub fn push(&mut self, elem: T) {

        let new_node = Box::new (
            Node {
                elem,
                next: self.head.take(),
            }
        );

        self.head = Some(new_node);

    }

    /// And pop something as well
    pub fn pop(&mut self) -> Option<T> {

        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
            }
        )

    }

    /// Peek into the list
    pub fn peek(&self) -> Option<&T> {

        self.head.as_ref().map(|node| {
            &node.elem
            }
        )

    }

    /// Peek into the list and return a mutable value
    pub fn peek_mut(&mut self) -> Option<&mut T> {

        self.head.as_mut().map(|node| {
            &mut node.elem
            }
        )

    }

    /// Get a desrtuctive iterator
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// Get a non-destructive iterator
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    /// Get a non-destructive, mutable,  iterator
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }

}

impl<T> Drop for List<T> {

    /// Loop through the list clearing the next pointers
    fn drop(&mut self) {

        let mut cur_link = self.head.take();

        // `while let` == "do this thing until this pattern doesn't match"

        while let Some(mut boxed_node) = cur_link {

                cur_link = boxed_node.next.take();

        }
    }
}

impl<T> Iterator for IntoIter<T> {

    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }

}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }

}

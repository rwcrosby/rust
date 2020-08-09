// Third version - Persistent singly linked stack
// from https://rust-unofficial.github.io/too-many-lists/third.html

use::std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

/// Type for a non-desrtuctive iterator
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    //! Third implementation

    /// Create a new list
    pub fn new() -> Self {
        List { 
            head: None 
        }
    }

    pub fn append(&self, elem: T) -> List<T> {

        List { head: Some(
                        Rc::new(
                            Node { 
                                elem: elem,
                                next: self.head.clone(),
                                }
                            )
                        )
            }

    }
    
    /// This and the following tail2 function should be equivalent
    pub fn tail(&self) -> List<T> {

        List { head: self.head.as_ref()
                         .and_then(|node| node.next.clone()) 
        }

    }

    /// This should be equipvalent to the tail function
    pub fn tail2(&self) -> List<T> {

        List { head: match self.head.as_ref() {
                        Some(node) => node.next.clone(),
                        None => None
                    }
        }

    }
   
    pub fn head(&self) -> Option<&T> {

        self.head.as_ref().map(|node| &node.elem)

    }

    /// Get a non-destructive iterator
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
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

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Debug;


#[derive(Debug)]
pub struct DLNode<T: Clone + Debug> {
    pub value: Box<T>,
    pub previous: Option<Weak<RefCell<DLNode<T>>>>,
    pub next: Option<Rc<RefCell<DLNode<T>>>>,
}

impl<T: Clone + Debug> DLNode<T> {
    pub fn new(value: T) -> DLNode<T> 
    where
    T: Clone {
        DLNode {
            value: Box::new(value),
            previous: None,
            next: None,
        }
    }
}


use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Debug;


#[derive(Debug)]
pub struct DLNode<T: Clone + Debug> {
    pub value: T,
    pub previous: Option<Weak<RefCell<DLNode<T>>>>,
    pub next: Option<Rc<RefCell<DLNode<T>>>>,
}

impl<T: Clone + Debug> DLNode<T> {
    pub fn new(value: T) -> DLNode<T> 
    where
    T: Clone {
        DLNode {
            value,
            previous: None,
            next: None,
        }
    }
}

impl<T: Clone + Debug> Drop for DLNode<T> {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.value);
    }
}

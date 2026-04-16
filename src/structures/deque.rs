
use std::rc::{Rc};
use std::cell::RefCell;
use std::fmt::Debug;
use crate::structures::doubly_linked_node::DLNode;


#[derive(Debug, Clone)]
pub struct Deque<T: std::clone::Clone + Debug> {
    head: Option<Rc<RefCell<DLNode<T>>>>,
    tail: Option<Rc<RefCell<DLNode<T>>>>,
}

impl<T: Clone + Debug> Deque<T> {
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_tail = Rc::new(RefCell::new(DLNode::new(value)));
        if self.tail.is_some() {
            let tail = self.tail.as_ref().unwrap().clone();
            tail.borrow_mut().next = Some(new_tail.clone());
            new_tail.borrow_mut().previous = Some(Rc::downgrade(&self.tail.as_ref().unwrap()));
            self.tail = Some(new_tail); //.clone()
        } else if self.head.is_some() {
            let mut head = self.head.as_ref().unwrap().borrow_mut();
            head.next = Some(new_tail.clone());
            new_tail.borrow_mut().previous = Some(Rc::downgrade(&self.head.as_ref().unwrap()));
            self.tail = Some(new_tail.clone());
        } else {
            self.head = Some(new_tail);
        }
        if self.tail.is_some() {
            println!("append() - Number of references for tail: {}.", Rc::strong_count(&self.tail.as_ref().unwrap()));
        }
        if self.head.is_some() {
            println!("append() - Number of references for head: {}.", Rc::strong_count(&self.head.as_ref().unwrap()));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.tail.take() {
            if Rc::ptr_eq(self.head.as_ref().unwrap(), &node) {
                let value: T = node.as_ref().borrow().value.clone();
                self.tail = None;
                println!("pop() - Removed value: {:?}.", value);
                println!("pop() - Number of references for head: {}.", Rc::strong_count(&self.head.as_ref().unwrap()));
                Some(value)
            } else {
                let new_tail = node.as_ref().borrow_mut().previous.clone();
                let new_tail = new_tail.unwrap().upgrade().unwrap().clone();
                self.tail = Some(new_tail);
                let value: T = node.as_ref().borrow().value.clone();
                println!("pop() - Removed value: {:?}.", value);
                println!("pop() - Number of references for head: {}.", Rc::strong_count(&self.head.as_ref().unwrap()));
                println!("pop() - Number of references for tail: {}.", Rc::strong_count(&self.tail.as_ref().unwrap()));
                Some(value)
            }
        } else if let Some(node) = self.head.take() {
            if self.head.is_some() {
                println!("pop() - Number of references for head: {}.", Rc::strong_count(&self.head.as_ref().unwrap()));
            } else {
                println!("pop() - head is None.");
            }
            let value = node.as_ref().borrow().value.clone();
            println!("pop() - Removed value: {:?}.", value);
            self.head = None;
            Some(value)
        }
        else {
            None
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            let new_head = head.as_ref().borrow_mut().next.clone();
            if new_head.is_some() {
                self.head = new_head;
            } else {
                self.head = None;
            }
            let value = head.as_ref().borrow().value.clone();
            Some(value)
        } else {
            None
        }
    }
}

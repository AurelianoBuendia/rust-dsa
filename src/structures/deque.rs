
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
            self.tail.as_ref().unwrap().borrow_mut().next = Some(new_tail.clone());
            new_tail.borrow_mut().previous = Some(Rc::downgrade(&self.tail.as_ref().unwrap()));
            self.tail = Some(new_tail);
        } else if self.head.is_some() {
            let mut head = self.head.as_ref().unwrap().borrow_mut();
            head.next = Some(new_tail.clone());
            new_tail.borrow_mut().previous = Some(Rc::downgrade(&self.head.as_ref().unwrap()));
            self.tail = Some(new_tail);
        } else {
            self.head = Some(new_tail);
            self.tail = None;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.tail.take() {
            if Rc::ptr_eq(self.head.as_ref().unwrap(), &node.borrow().previous.as_ref().unwrap().upgrade().unwrap()) {
                let value: T = *node.as_ref().borrow().value.to_owned();
                self.tail = None;
                self.head.as_ref().unwrap().borrow_mut().next = None;
                // println!("pop() - Number of references for head: {}.", Rc::strong_count(&self.head.as_ref().unwrap()));
                Some(value)
            } else {
                let new_tail = node.as_ref().borrow_mut().previous.as_ref().unwrap().upgrade().unwrap();
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
                let value: T = *node.as_ref().borrow().value.to_owned();
                // println!("pop() - Number of references for head: {}.", Rc::strong_count(&self.head.as_ref().unwrap()));
                // println!("pop() - Number of references for tail: {}.", Rc::strong_count(&self.tail.as_ref().unwrap()));
                Some(value)
            }
        } else {
            if self.head.is_some() {
                let head = self.head.take();
                let value_box: Box<T> = head.as_ref().unwrap().borrow_mut().value.clone();
                let value = *value_box;
                self.tail = None;
                Some(value)
            } else {
                println!("pop() - head is None.");
                None
            }
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
            Some(*value)
        } else {
            None
        }
    }
}

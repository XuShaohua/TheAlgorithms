// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
pub struct Node {
    value: String,
    next: Link,
    previous: Link,
}

#[derive(Debug, Clone)]
pub struct TransactionLog {
    length: usize,
    head: Link,
    tail: Link,
}

pub struct ListIterator {
    current: Link,
}

impl Node {
    pub fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            next: None,
            previous: None,
        }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, value: String) {
        let tail = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(tail.clone());
                // Set previous pointer.
                tail.borrow_mut().previous = Some(old_tail);
            }
            None => self.head = Some(tail.clone()),
        }
        self.tail = Some(tail);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head: Rc<RefCell<Node>>| {
            if let Some(next) = head.borrow_mut().next.take() {
                // Reset previous pointer.
                next.borrow_mut().previous = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            let node: Option<RefCell<Node>> = Rc::try_unwrap(head).ok();
            node.expect("").into_inner().value
        })
    }
}

impl ListIterator {
    pub fn new(started_at: Link) -> Self {
        Self {
            current: started_at,
        }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };

        result
    }
}

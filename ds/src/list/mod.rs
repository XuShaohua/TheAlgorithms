// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

struct TransactionLog {
    length: usize,
    head: SingleLink,
    tail: SingleLink,
}

impl Node {
    pub fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next: None }))
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
            Some(old_tail) => old_tail.borrow_mut().next = Some(tail.clone()),
            None => self.head = Some(tail.clone()),
        }
        self.tail = Some(tail);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head: Rc<RefCell<Node>>| {
            if let Some(next) = head.borrow_mut().next.take() {
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

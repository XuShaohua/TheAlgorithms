// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

type ListNodePtr<T> = Option<Rc<RefCell<ListNode<T>>>>;

pub struct LinkedListV2<T> {
    length: usize,
    head: ListNodePtr<T>,
    tail: ListNodePtr<T>,
}

struct ListNode<T> {
    value: T,
    next: ListNodePtr<T>,
}

impl<T: Clone> Clone for ListNode<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl<T> ListNode<T> {
    #[must_use]
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next: None }))
    }

    #[must_use]
    pub fn with_next(value: T, next: ListNodePtr<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next }))
    }

    /// Check wether current node is the the last one in list.
    #[must_use]
    pub const fn is_last(&self) -> bool {
        self.next.is_none()
    }
}

impl<T> LinkedListV2<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    #[must_use]
    pub fn from_vec(vec: Vec<T>) -> Self {
        let length = vec.len();
        let mut tail = None;
        let mut node = None;
        for item in vec {
            let node_rc = ListNode::with_next(item, node);
            if tail.is_none() {
                tail.replace(node_rc.clone());
            }
            node = Some(node_rc);
        }
        let head = node;
        Self { length, head, tail }
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Add a new node to tail of list.
    pub fn push_back(&mut self, value: T) {
        let node = ListNode::new(value);
        match self.tail.take() {
            Some(tail) => tail.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone()),
        }
        self.tail = Some(node);
        self.length += 1;
    }

    /// Add a new node to head of list.
    pub fn push_front(&mut self, value: T) {
        let node = ListNode::new(value);
        match self.head.take() {
            Some(head) => node.borrow_mut().next = Some(head),
            None => self.tail = Some(node.clone()),
        }
        self.head = Some(node);
        self.length += 1;
    }

    /// Remove a node from head of list.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head
            .take()
            // Extract value from head if it has only one strong reference.
            .and_then(|head: Rc<RefCell<ListNode<T>>>| Rc::try_unwrap(head).ok())
            .map(|head| {
                if let Some(next) = head.borrow_mut().next.take() {
                    self.head = Some(next);
                } else {
                    // Reset tail to None if head->next is None
                    self.tail.take();
                }
                self.length -= 1;
                head.into_inner().value
            })
    }

    /// Remove a node from tail of list.
    ///
    /// Time is O(n).
    fn pop_back() -> Option<T> {
        unimplemented!()
    }

    /// Get reference of value in head node of list.
    #[must_use]
    pub fn head(&self) -> Option<&T> {
        unimplemented!()
    }

    /// Get mutable reference of value in head node of list.
    #[must_use]
    pub fn head_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Get reference of value in tail node of list.
    #[must_use]
    pub fn tail(&self) -> Option<&T> {
        unimplemented!()
    }

    /// Get mutable reference of value in tail node of list.
    #[must_use]
    pub fn tail_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }
}

// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

type ListNodePtr<T> = Option<Box<ListNode<T>>>;

pub struct LinkedListV1<T> {
    length: usize,
    head: ListNodePtr<T>,
}

pub struct ListNode<T> {
    value: T,
    next: ListNodePtr<T>,
}

impl<T> ListNode<T> {
    #[must_use]
    pub fn new(value: T) -> Box<Self> {
        Box::new(Self { value, next: None })
    }

    #[must_use]
    pub fn with_next(value: T, next: ListNodePtr<T>) -> Box<Self> {
        Box::new(Self { value, next })
    }

    #[must_use]
    pub const fn is_last(&self) -> bool {
        self.next.is_some()
    }
}

impl<T> LinkedListV1<T> {
    #[must_use]
    pub const fn new_empty() -> Self {
        Self {
            length: 0,
            head: None,
        }
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

    /// Add a new node to head of list.
    pub fn push(&mut self, value: T) {
        let old_head: ListNodePtr<T> = self.head.take();
        let head = ListNode::with_next(value, old_head);
        self.head = Some(head);
        self.length += 1;
    }

    /// Insert the value at specific position in list.
    ///
    /// Time is O(n).
    pub fn insert_at(&mut self, _value: T, pos: usize) {
        assert!(pos < self.length);
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
        /*
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
        */
    }

    /// Remove a node at position.
    pub fn remove_at(&mut self, pos: usize) {
        assert!(pos < self.length);
    }
}

impl<T> LinkedListV1<T>
where
    T: PartialEq,
{
    /// Returns position of value in list.
    ///
    /// Returns None if not found.
    pub fn find(&self, _value: &T) -> ListNodePtr<T> {
        // TODO(Shaohua): Returns a reference
        unimplemented!()
    }

    /// Delete a node from list with same value.
    pub fn remove(&mut self, _value: &T) -> Option<usize> {
        unimplemented!()
    }
}

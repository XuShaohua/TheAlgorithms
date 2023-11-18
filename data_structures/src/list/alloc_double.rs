// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Option<Box<T>>>,
}

struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    #[must_use]
    fn new(element: T) -> Self {
        Self {
            prev: None,
            next: None,
            element,
        }
    }

    fn into_inner(self: Box<Self>) -> T {
        self.element
    }
}

impl Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    /// Create an empty list.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn clear(&mut self) {
        drop(self.head.take());
        drop(self.tail.take());
        self.len = 0;
    }

    /// Concat other list to self.
    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => {
                // self is empty, just swap with other one.
                mem::swap(self, other);
            }
            Some(mut tail) => {
                if let Some(other_head) = other.head.take() {
                    // Extend head and tail of self.
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }
                    self.tail = other.tail.take();

                    self.len += other.len;
                    other.len = 0;
                }
            }
        }
    }

    pub fn push_front(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        let node_ptr = NonNull::from(Box::leak(node));
    }
}

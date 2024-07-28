// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    value: T,
}

impl<T> Default for LinkedList<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// Public functions for list.
impl<T> LinkedList<T> {
    // Construct

    /// Create an empty list.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    // Element access

    /// Access the first element.
    #[must_use]
    #[inline]
    pub fn front(&self) -> &T {
        todo!()
    }

    /// Access the first element exclusively.
    #[must_use]
    #[inline]
    pub fn front_mut(&mut self) -> &mut T {
        todo!()
    }

    /// Access the last element.
    #[must_use]
    #[inline]
    pub fn back(&self) -> &T {
        todo!()
    }

    /// Access the last element exclusively.
    #[must_use]
    #[inline]
    pub fn back_mut(&mut self) -> &mut T {
        todo!()
    }

    // Iterators
    //pub fn iter(&self) ->
    //pub fn into_iter(self) ->
    //pub fn iter_mut(&mut self) ->

    // Capacity opertations

    /// Returns the number of elements in list.
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Check whether the list is empty.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Modifiers

    /// Clear the contents.
    ///
    /// Erases all elements from the list.
    /// After calling this function, size of list is zero.
    pub fn clear(&mut self) {
        self.len = 0;
        todo!()
    }

    pub fn insert(&mut self) {}

    pub fn push_front() {}

    pub fn push_back(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    pub fn pop_front() {}

    pub fn pop_back() {}

    /// Swap the contents.
    pub fn swap(&mut self) {}

    // Operations

    /// Merges two sorted lists.
    pub fn merge(&mut self, other: &mut Self) {
        todo!()
    }

    /// Append all elements in other list to self.
    pub fn appnend(&mut self, other: &mut Self) {
        todo!()
    }

    /// Move elements from another list.
    pub fn splice(&mut self, other: &mut Self) {
        todo!()
    }

    /// Removes elements satisfying specific condition.
    pub fn remove(&mut self, value: &T)
    where
        T: PartialEq<T>,
    {
        todo!()
    }

    /// Removes elements satisfying specific condition.
    pub fn remove_if(&mut self) {
        todo!()
    }

    /// Reverses the order of the elements.
    pub fn reverse(&mut self) {
        todo!()
    }

    /// Removes consecutive duplicate elements.
    pub fn unique(&mut self) {
        todo!()
    }
    pub fn sort(&mut self) {
        todo!()
    }

    //pub fn sort_by(&mut self) { }
    //pub fn sort_by_key(&mut self) { }
}

// Private or unsafe functions for list.
impl<T> LinkedList<T> {
    unsafe fn push_back_node(&mut self, node_ptr: NonNull<Node<T>>) {
        (*node_ptr.as_ptr()).next = None;
        (*node_ptr.as_ptr()).prev = self.tail;
        let node = Some(node_ptr);

        match self.tail {
            Some(tail) => (*tail.as_ptr()).next = node,
            None => self.head = node,
        }

        self.tail = node;
        self.len += 1;
    }

    //unsafe fn push_front_node(&mut self, node_ptr: NonNull<Node
}

impl<T> Node<T> {
    #[must_use]
    #[inline]
    fn new(value: T) -> Self {
        Self {
            prev: None,
            next: None,
            value,
        }
    }

    #[must_use]
    #[inline]
    fn into_value(self) -> T {
        self.value
    }
}

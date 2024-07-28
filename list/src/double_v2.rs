// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

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
        let mut other = Self::new();
        mem::swap(self, &mut other);
        drop(other);
    }

    pub fn insert(&mut self) {}

    /// Add an element to the beginning of list.
    pub fn push_front(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node_ptr = NonNull::from(Box::leak(node));
        self.push_back_node(node_ptr);
    }

    /// Add an element to the end of list.
    pub fn push_back(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node_ptr = NonNull::from(Box::leak(node));
        self.push_back_node(node_ptr);
    }

    /// Remove the first element in the list.
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_inner)
    }

    /// Remove the last element in the list.
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_inner)
    }

    /// Change number of elements stored.
    ///
    /// If the current size is greater than `new_size`, extra elements will
    /// be removed.
    /// If current size is less than `new_size`, more elements with default
    /// value are appended.
    fn resize(&mut self, new_size: usize)
    where
        T: Default,
    {
        match self.len.cmp(&new_size) {
            Ordering::Equal => (),
            Ordering::Less => {
                for _ in 0..(new_size - self.len) {
                    self.push_back(T::default());
                }
            }
            Ordering::Greater => {
                for _ in 0..(self.len - new_size) {
                    let _node = self.pop_back_node();
                }
            }
        }
    }

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
    fn push_front_node(&mut self, node_ptr: NonNull<Node<T>>) {
        unsafe {
            (*node_ptr.as_ptr()).next = self.head;
            (*node_ptr.as_ptr()).prev = None;
        }
        let node = Some(node_ptr);

        match self.head {
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
            None => self.tail = node,
        }

        self.head = node;
        self.len += 1;
    }

    fn push_back_node(&mut self, node_ptr: NonNull<Node<T>>) {
        unsafe {
            (*node_ptr.as_ptr()).next = None;
            (*node_ptr.as_ptr()).prev = self.tail;
        }
        let node = Some(node_ptr);

        match self.tail {
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
            None => self.head = node,
        }

        self.tail = node;
        self.len += 1;
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|old_head| {
            let old_head = unsafe { Box::from_raw(old_head.as_ptr()) };
            self.head = old_head.next;

            match self.head {
                Some(head) => unsafe { (*head.as_ptr()).prev = None },
                None => self.tail = None,
            }

            self.len -= 1;
            old_head
        })
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|old_tail| {
            let old_tail = unsafe { Box::from_raw(old_tail.as_ptr()) };
            self.tail = old_tail.prev;

            match self.tail {
                Some(tail) => unsafe { (*tail.as_ptr()).next = None },
                None => self.head = None,
            }

            self.len -= 1;
            old_tail
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_node) = self.pop_front_node() {
            // Do nothing
        }
    }
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
    fn into_inner(self: Box<Self>) -> T {
        self.value
    }
}

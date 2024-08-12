// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::{fmt, mem};
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct DoublyLinkedList<T> {
    len: usize,
    head: NodePtr<T>,
    tail: NodePtr<T>,
    _marker: PhantomData<Box<Node<T>>>,
}

type NodePtr<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    prev: NodePtr<T>,
    next: NodePtr<T>,
    value: T,
}

// Public functions for list.
impl<T> DoublyLinkedList<T> {
    /// Create an empty list.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    // Element access

    /// Access the first node.
    #[must_use]
    #[inline]
    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.as_ref().value)
        }
    }

    /// Access the first node exclusively.
    #[must_use]
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.as_mut().value)
        }
    }

    /// Access the last node.
    #[must_use]
    #[inline]
    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.tail.as_ref().map(|node| &node.as_ref().value)
        }
    }

    /// Access the last node exclusively.
    #[must_use]
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.tail.as_mut().map(|node| &mut node.as_mut().value)
        }
    }

    // Capacity operations

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

    // Iterators
    //pub fn iter(&self) ->
    //pub fn into_iter(self) ->
    //pub fn iter_mut(&mut self) ->


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

    pub fn insert(&mut self, value: T) {
        todo!()
    }

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
impl<T> DoublyLinkedList<T> {
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

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_node) = self.pop_front_node() {
            todo!()
        }
    }
}

impl<T> Default for DoublyLinkedList<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<T: Clone> Clone for DoublyLinkedList<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T: PartialEq> PartialEq for DoublyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<T: Eq> Eq for DoublyLinkedList<T> {}

impl<T: Hash> Hash for DoublyLinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<T> Node<T> {
    #[must_use]
    #[inline]
    const fn new(value: T) -> Self {
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

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;

    #[test]
    fn test_is_empty() {
        let list = DoublyLinkedList::<i32>::new();
        assert!(list.is_empty());
    }
}
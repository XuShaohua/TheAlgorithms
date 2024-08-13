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
    head: NodePtr<T>,
    tail: NodePtr<T>,
    len: usize,
    _marker: PhantomData<Box<Node<T>>>,
}

type NodePtr<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    prev: NodePtr<T>,
    next: NodePtr<T>,
    value: T,
}

pub struct IntoIter<T>(DoublyLinkedList<T>);

pub struct Iter<'a, T: 'a> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
    len: usize,
    _marker: PhantomData<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
    len: usize,
    _marker: PhantomData<&'a mut Node<T>>,
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
        unsafe { self.head.as_ref().map(|node| &node.as_ref().value) }
    }

    /// Access the first node exclusively.
    #[must_use]
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().value) }
    }

    /// Access the last node.
    #[must_use]
    #[inline]
    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().value) }
    }

    /// Access the last node exclusively.
    #[must_use]
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().value) }
    }

    pub fn contains(&self, _value: &T) -> bool
    where
        T: PartialEq<T>,
    {
        todo!()
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
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: Default::default(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: Default::default(),
        }
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

    pub fn insert(&mut self, _pos: usize, _value: T) {
        todo!()
    }

    pub fn insert_range(&mut self, _pos: usize) {
        todo!()
    }

    pub fn erase(&mut self, _pos: usize) -> bool {
        todo!()
    }

    pub fn erase_range(&mut self, _start: usize, _end: usize) -> usize {
        todo!()
    }

    /// Add an element to the beginning of list.
    pub fn push_front(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node_ptr = NonNull::from(Box::leak(node));
        self.push_front_node(node_ptr);
    }

    /// Remove the first node in the list.
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_inner)
    }

    /// Add an element to the end of list.
    pub fn push_back(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node_ptr = NonNull::from(Box::leak(node));
        self.push_back_node(node_ptr);
    }

    /// Remove the last node in the list.
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_inner)
    }

    /// Append all elements in another list to self.
    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            Some(mut tail) => {
                // connect tail of self to head of other.
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }

                    self.tail = other.tail.take();
                    self.len += other.len();
                    other.len = 0;
                }
            }
            None => {
                // self is empty.
                mem::swap(self, other);
            }
        }
    }

    /// Prepend all elements in another list to self.
    #[inline]
    pub fn prepend(&mut self, other: &mut Self) {
        other.append(self);
        self.swap(other);
    }

    /// Change number of elements stored.
    ///
    /// If the current size is greater than `new_size`, extra elements will
    /// be removed.
    /// If current size is less than `new_size`, more elements with default
    /// value are appended.
    pub fn resize(&mut self, new_size: usize)
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

    /// Change number of elements stored.
    pub fn resize_with(&mut self, new_size: usize, value: T)
    where
        T: Clone,
    {
        match self.len.cmp(&new_size) {
            Ordering::Equal => (),
            Ordering::Less => {
                for _ in 0..(new_size - self.len) {
                    self.push_back(value.clone());
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
    #[inline]
    pub fn swap(&mut self, other: &mut Self) {
        mem::swap(self, other);
    }

    // Operations

    /// Merges two sorted lists.
    pub fn merge(&mut self, _other: &mut Self)
    where
        T: PartialOrd<T>,
    {
        todo!()
    }

    // pub fn merge_by(&mut self, other: &mut Self, predict: P)
    // where
    //     P: PartialOrd<T>,
    // {
    //     todo!()
    // }

    /// Move elements from another list.
    pub fn splice(&mut self, _other: &mut Self) {
        todo!()
    }

    /// Removes all elements equals specific value.
    pub fn remove(&mut self, _value: &T)
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
            // TODO(Shaohua):
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
        f.debug_list().entries(self).finish()
    }
}

impl<T: Clone> Clone for DoublyLinkedList<T> {
    fn clone(&self) -> Self {
        let mut list = Self::new();
        list.extend(self.iter().cloned());
        list
    }
}

impl<T: PartialEq> PartialEq for DoublyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }

    fn ne(&self, other: &Self) -> bool {
        self.len != other.len || self.iter().ne(other.iter())
    }
}

impl<T: Eq> Eq for DoublyLinkedList<T> {}

impl<T: Hash> Hash for DoublyLinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // state.write_length_prefix(self.len);
        state.write_usize(self.len);
        for element in self {
            element.hash(state);
        }
    }
}

impl<T> Extend<T> for DoublyLinkedList<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|value| self.push_back(value));
    }

    // #[inline]
    // fn extend_one(&mut self, item: T) {
    //     self.push_back(item);
    // }
}

impl<T> FromIterator<T> for DoublyLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DoublyLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node: &Node<T> = node.as_ref();
                self.len -= 1;
                self.head = node.next;
                &node.value
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let node: &Node<T> = node.as_ref();
                self.tail = node.prev;
                self.len -= 1;
                &node.value
            })
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|mut node| unsafe {
                let node: &mut Node<T> = node.as_mut();
                self.len -= 1;
                self.head = node.next;
                &mut node.value
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|mut node| unsafe {
                let node: &mut Node<T> = node.as_mut();
                self.tail = node.prev;
                self.len -= 1;
                &mut node.value
            })
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

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

    #[test]
    fn test_push() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(3);
        list.push_front(5);
        list.push_front(7);
        list.push_front(11);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_pop_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(3);
        list.push_front(5);
        list.push_front(7);
        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(3));
        assert!(list.is_empty());
    }

    #[test]
    fn test_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(3);
        list.push_back(5);
        list.push_back(7);
        assert_eq!(list.pop_back(), Some(7));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(3));
        assert!(list.is_empty());
    }

    #[test]
    fn test_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(5);
        list.push_back(7);
        assert_eq!(list.back(), Some(&7));
        assert_eq!(list.front(), Some(&5));
    }

    #[test]
    fn test_back_mut() {
        let mut list = DoublyLinkedList::new();
        list.push_back(5);
        list.push_back(7);
        if let Some(value) = list.back_mut() {
            *value = 11;
        }
        assert_eq!(list.back(), Some(&11));
    }

    #[test]
    fn test_drop() {
        let mut list = DoublyLinkedList::new();
        for i in 0..(128 * 200) {
            list.push_front(i);
        }
        drop(list);
    }

    #[test]
    fn test_into_iter() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(3);
        list.push_front(5);
        list.push_front(7);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_append() {}

    #[test]
    fn test_prepend() {}

    #[test]
    fn test_extend() {
        let mut list = DoublyLinkedList::new();
        let numbers = [1, 2, 3, 4, 5];
        list.extend(numbers.clone());
        assert_eq!(list, DoublyLinkedList::from_iter(numbers));
    }
}

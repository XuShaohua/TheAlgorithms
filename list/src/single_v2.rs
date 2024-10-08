// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

type ListNodePtr<T> = Option<Rc<RefCell<ListNode<T>>>>;

pub struct LinkedList<T> {
    length: usize,
    head: ListNodePtr<T>,
    tail: ListNodePtr<T>,
}

struct ListNode<T> {
    value: T,
    next: ListNodePtr<T>,
}

//pub struct Iter<'a, T> {
//    next: ListNodePtr<T>,
//    phantom_data: PhantomData<&'a T>,
//}

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

impl<T> Default for LinkedList<T> {
    #[must_use]
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    #[must_use]
    #[inline]
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
        self.head.take().and_then(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                self.head = Some(new_head);
            } else {
                // Reset tail to None if both head and tail points to the same node.
                self.tail.take();
            }
            self.length -= 1;

            // Extract value from head if it has only one strong reference.
            Rc::try_unwrap(old_head)
                .ok()
                .map(|head| head.into_inner().value)
        })
    }

    /// Remove a node from tail of list.
    ///
    /// Time is O(n).
    fn pop_back() -> Option<T> {
        unimplemented!()
    }

    /// Reverse orders of node in list.
    pub fn reverse(&mut self) {}

    /// Get reference of value in head node of list.
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.head
                .as_ref()
                .and_then(|node: &Rc<RefCell<ListNode<T>>>| node.try_borrow_unguarded().ok())
                .map(|node| &node.value)
        }
    }

    /// Get mutable reference of value in head node of list.
    #[must_use]
    pub fn front_mut(&mut self) -> Option<RefMut<T>> {
        unimplemented!()
    }

    /// Get reference of value in tail node of list.
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.tail
                .as_ref()
                .and_then(|node: &Rc<RefCell<ListNode<T>>>| node.try_borrow_unguarded().ok())
                .map(|node| &node.value)
        }
    }

    /// Get mutable reference of value in tail node of list.
    #[must_use]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    // TODO(Shaohua): Impl Iter
    //    #[must_use]
    //    pub fn iter(&self) -> Iter<T> {
    //        Iter {
    //            next: self.head.clone(),
    //            phantom_data: PhantomData,
    //        }
    //    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut list = Self::new();
        for item in iter {
            list.push_front(item);
        }
        list
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Reset tail first.
        self.tail.take();

        let mut node = self.head.take();
        while let Some(node_rc) = node {
            node = Rc::try_unwrap(node_rc)
                .ok()
                .and_then(|node| node.borrow_mut().next.take());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_new() {
        let list = LinkedList::<i32>::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push_front(2);
        list.push_front(3);
        list.push_front(5);
        list.push_front(7);
        list.push_front(11);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(5);
        list.push_front(7);
        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(3));
        println!("len of list: {}", list.len());
        assert!(list.is_empty());
    }

    #[test]
    fn test_drop() {
        let mut list = LinkedList::new();
        for i in 0..(128 * 200) {
            list.push_front(i);
        }
        drop(list);
    }
}

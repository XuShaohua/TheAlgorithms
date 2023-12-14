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

    /// Check wether current node is the the last one in list.
    #[must_use]
    pub const fn is_last(&self) -> bool {
        self.next.is_none()
    }
}

impl<T> LinkedListV1<T> {
    #[must_use]
    pub const fn new() -> Self {
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
        let head = ListNode::with_next(value, self.head.take());
        self.head = Some(head);
        self.length += 1;
    }

    /// Insert the value at specific position in list.
    ///
    /// Time is O(n).
    pub fn insert_at(&mut self, _value: &T, pos: usize) {
        debug_assert!(pos < self.length);
    }

    /// Remove the head node and return the value.
    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                let value = Some(head.value);
                self.head = head.next;
                self.length -= 1;
                value
            }
            None => None,
        }
    }

    /// Remove a node at position.
    pub fn remove_at(&mut self, pos: usize) {
        debug_assert!(pos < self.length);
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

#[cfg(test)]
mod tests {
    use super::LinkedListV1;

    #[test]
    fn test_new() {
        let list = LinkedListV1::<i32>::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_push() {
        let mut list = LinkedListV1::new();
        list.push(2);
        list.push(3);
        list.push(5);
        list.push(7);
        list.push(11);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedListV1::new();
        list.push(5);
        list.push(7);
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.len(), 1);
        let _ = list.pop();
        assert!(list.is_empty());
    }

    #[test]
    fn test_drop() {
        // The default recursive limit rustc-v1.74 is 128.
        // See https://doc.rust-lang.org/reference/attributes/limits.html
        let mut list = LinkedListV1::new();
        for i in 0..(128 * 8) {
            list.push(i);
        }
        drop(list);
    }
}

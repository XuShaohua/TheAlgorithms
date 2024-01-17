// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

type ListNodePtr<T> = Option<Box<ListNode<T>>>;

pub struct LinkedListV1<T> {
    length: usize,
    head: ListNodePtr<T>,
}

struct ListNode<T> {
    value: T,
    next: ListNodePtr<T>,
}

pub struct IntoIter<T>(LinkedListV1<T>);

pub struct Iter<'a, T> {
    next: Option<&'a ListNode<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut ListNode<T>>,
}

impl<T> ListNode<T> {
    #[must_use]
    #[allow(clippy::unnecessary_box_returns)]
    pub fn new(value: T) -> Box<Self> {
        Box::new(Self { value, next: None })
    }

    #[must_use]
    #[allow(clippy::unnecessary_box_returns)]
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

    /// Remove the head node and return the value.
    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            self.length -= 1;
            head.value
        })
    }

    /// Insert the value at specific position in list.
    ///
    /// Time is O(n).
    pub fn insert_at(&mut self, _value: &T, _pos: usize) -> Option<usize> {
        unimplemented!()
    }

    /// Remove a node at position.
    pub fn remove_at(&mut self, _pos: usize) -> Option<T> {
        unimplemented!()
    }

    /// Get reference of value in head node in list.
    #[must_use]
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.value)
    }

    /// Get mutable reference of value in head node in list.
    #[must_use]
    pub fn head_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|head| &mut head.value)
    }

    #[must_use]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for LinkedListV1<T> {
    fn drop(&mut self) {
        let mut node = self.head.take();
        while let Some(mut boxed_node) = node {
            node = boxed_node.next.take();
            // No need to drop boxed_node explicitly.
            //drop(boxed_node);
        }
    }
}

// Or use move
//impl<T> Drop for LinkedListV1<T> {
//    fn drop(&mut self) {
//        let mut node = self.head.take();
//        while let Some(boxed_node) = node {
//            // Partial move.
//            node = boxed_node.next;
//        }
//    }
//}

impl<T> LinkedListV1<T>
where
    T: PartialEq,
{
    /// Returns position of value in list.
    ///
    /// Returns None if not found.
    pub fn find(&self, _value: &T) -> Option<usize> {
        unimplemented!()
    }

    /// Delete a node from list with same value.
    pub fn remove(&mut self, _value: &T) -> Option<usize> {
        unimplemented!()
    }
}

impl<T> IntoIterator for LinkedListV1<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> IntoIterator for &'a LinkedListV1<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedListV1<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
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
        for i in 0..(128 * 200) {
            list.push(i);
        }
        drop(list);
    }

    #[test]
    fn test_head() {
        let mut list = LinkedListV1::new();
        list.push(5);
        list.push(7);
        assert_eq!(list.head(), Some(&7));
    }

    #[test]
    fn test_head_mut() {
        let mut list = LinkedListV1::new();
        list.push(5);
        list.push(7);
        list.head_mut().map(|value| *value = 11);
        // Option::replace() will not work, as it requires `&mut Option<T>`.
        // list.head_mut().replace(11);
        assert_eq!(list.head(), Some(&11));
    }

    #[test]
    fn test_into_iter() {
        let mut list = LinkedListV1::new();
        list.push(2);
        list.push(3);
        list.push(5);
        list.push(7);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedListV1::new();
        list.push(2);
        list.push(3);
        list.push(5);
        list.push(7);
        let nums = &[7, 5, 3, 2];
        for (val, num) in list.iter().zip(nums) {
            assert_eq!(val, num);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedListV1::new();
        list.push(2);
        list.push(3);
        list.push(5);
        list.push(7);
        for val in list.iter_mut() {
            *val *= 2;
        }
        let nums = &[14, 10, 6, 4];
        for (val, num) in list.iter().zip(nums) {
            assert_eq!(val, num);
        }
    }
}

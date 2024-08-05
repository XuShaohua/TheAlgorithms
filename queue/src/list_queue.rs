// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::collections::LinkedList;
use std::fmt;
use std::hash::{Hash, Hasher};

#[allow(clippy::linkedlist)]
pub struct ListQueue<T> (LinkedList<T>);

impl<T> Default for ListQueue<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ListQueue<T> {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self(LinkedList::new())
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push_back(value);
    }

    #[must_use]
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.0.front()
    }

    #[must_use]
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.0.front_mut()
    }

    #[must_use]
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.0.back()
    }

    #[must_use]
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.0.back_mut()
    }
}

impl<T: PartialEq> PartialEq for ListQueue<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl<T: Eq> Eq for ListQueue<T> {}

impl<T: PartialOrd> PartialOrd for ListQueue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl<T: Ord> Ord for ListQueue<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

impl<T: Hash> Hash for ListQueue<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.0, state);
    }
}

impl<T> FromIterator<T> for ListQueue<T> {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let list = iter.into_iter().collect();
        Self(list)
    }
}

impl<T: fmt::Debug> fmt::Debug for ListQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::ListQueue;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<ListQueue::<i32>>(), 24);
    }

    #[test]
    fn test_new() {
        let queue = ListQueue::<i32>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_push() {
        let mut queue = ListQueue::new();
        queue.push(0);
        assert_eq!(queue.front().copied(), Some(0));
        assert_eq!(queue.len(), 1);

        queue.push(1);
        assert_eq!(queue.front().copied(), Some(0));
        assert_eq!(queue.len(), 2);

        queue.push(2);
        queue.push(3);
    }

    #[test]
    fn test_pop() {
        let mut queue = ListQueue::new();
        queue.push(0);
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.len(), 3);

        let ret = queue.pop();
        assert_eq!(ret, Some(0));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.front().copied(), Some(1));

        queue.push(3);
        assert_eq!(queue.len(), 3);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert!(queue.is_empty());
    }
}
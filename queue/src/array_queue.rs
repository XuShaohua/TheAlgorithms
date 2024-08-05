// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct ArrayQueue<T> {
    len: usize,
    buf: Box<[Option<T>]>,
}

impl<T> ArrayQueue<T> {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let values: Vec<Option<T>> = (0..capacity).map(|_| None).collect();

        Self {
            len: 0,
            buf: values.into_boxed_slice(),
        }
    }

    /// # Errors
    ///
    /// 当栈已满时再将元素入队, 就会返回错误, 以及原有的元素 `value`.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == self.buf.len() {
            return Err(value);
        }

        self.buf[self.len] = Some(value);
        self.len += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let front = self.buf[0].take();
            for i in 1..self.len {
                self.buf.swap(i - 1, i);
            }
            self.len -= 1;
            front
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.buf.len()
    }

    #[must_use]
    #[inline]
    pub const fn front(&self) -> Option<&T> {
        if self.len > 0 {
            self.buf[0].as_ref()
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            self.buf[0].as_mut()
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub const fn back(&self) -> Option<&T> {
        if self.len > 0 {
            self.buf[self.len - 1].as_ref()
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            self.buf[self.len - 1].as_mut()
        } else {
            None
        }
    }
}

impl<T: PartialEq> PartialEq for ArrayQueue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && PartialEq::eq(&self.buf, &other.buf)
    }
}

impl<T: Eq> Eq for ArrayQueue<T> {}

impl<T: PartialOrd> PartialOrd for ArrayQueue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.buf, &other.buf)
    }
}

impl<T: Ord> Ord for ArrayQueue<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.buf, &other.buf)
    }
}

impl<T: Hash> Hash for ArrayQueue<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.buf, state);
    }
}

impl<T> FromIterator<T> for ArrayQueue<T> {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let vec: Vec<Option<T>> = iter.into_iter().map(|item| Some(item)).collect();
        Self {
            len: vec.len(),
            buf: vec.into_boxed_slice(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for ArrayQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.buf, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::array_queue::ArrayQueue;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<ArrayQueue::<i32>>(), 24);
    }

    #[test]
    fn test_new() {
        let queue = ArrayQueue::<i32>::new(5);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_push() {
        let mut queue = ArrayQueue::new(3);
        let ret = queue.push(0);
        assert!(ret.is_ok());
        assert_eq!(queue.front().copied(), Some(0));
        assert_eq!(queue.len(), 1);

        let ret = queue.push(1);
        assert!(ret.is_ok());
        assert_eq!(queue.front().copied(), Some(0));
        assert_eq!(queue.len(), 2);

        let ret = queue.push(2);
        assert!(ret.is_ok());
        let ret = queue.push(3);
        assert_eq!(ret, Err(3));
    }

    #[test]
    fn test_pop() {
        let mut queue = ArrayQueue::new(3);
        assert!(queue.push(0).is_ok());
        assert!(queue.push(1).is_ok());
        assert!(queue.push(2).is_ok());
        assert_eq!(queue.len(), 3);

        let ret = queue.pop();
        assert_eq!(ret, Some(0));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.front().copied(), Some(1));

        let ret = queue.push(3);
        assert!(ret.is_ok());
        assert_eq!(queue.len(), 3);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert!(queue.is_empty());
    }
}
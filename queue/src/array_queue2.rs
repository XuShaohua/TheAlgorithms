// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::{fmt, ptr};
use std::alloc::{alloc, Layout};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

pub struct ArrayQueue2<T> {
    len: usize,
    buf: Box<[T]>,
}

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum AllocError {
    CapacityOverflow,
    AllocateError,
}

impl<T> ArrayQueue2<T> {
    /// # Panics
    ///
    /// Raise panic if failed to allocate memory.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let raw_vec = RawVec::<T>::try_allocate(capacity).expect("Failed to allocate buffer");
        let buf: Box<[T]> = unsafe { raw_vec.into_box() };
        Self {
            len: 0,
            buf,
        }
    }

    /// # Errors
    ///
    /// 当栈已满时再将元素入队, 就会返回错误, 以及原有的元素 `value`.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == self.buf.len() {
            return Err(value);
        }

        self.buf[self.len] = value;
        self.len += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            // Take the first value, without calling drop method.
            let front = unsafe {
                Some(ptr::read(self.buf.as_ptr()))
            };
            // Move memory.
            unsafe {
                ptr::copy(self.buf.as_ptr().wrapping_add(1), self.buf.as_mut_ptr(), self.len - 1);
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
            Some(&self.buf[0])
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            Some(&mut self.buf[0])
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub const fn back(&self) -> Option<&T> {
        if self.len > 0 {
            Some(&self.buf[self.len - 1])
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            Some(&mut self.buf[self.len - 1])
        } else {
            None
        }
    }
}

impl<T: PartialEq> PartialEq for ArrayQueue2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && PartialEq::eq(&self.buf, &other.buf)
    }
}

impl<T: Eq> Eq for ArrayQueue2<T> {}

impl<T: PartialOrd> PartialOrd for ArrayQueue2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.buf, &other.buf)
    }
}

impl<T: Ord> Ord for ArrayQueue2<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.buf, &other.buf)
    }
}

impl<T: Hash> Hash for ArrayQueue2<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.buf, state);
    }
}

impl<T> FromIterator<T> for ArrayQueue2<T> {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let vec: Vec<T> = iter.into_iter().collect();
        Self {
            len: vec.len(),
            buf: vec.into_boxed_slice(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for ArrayQueue2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.buf, f)
    }
}

impl<T> RawVec<T> {
    fn try_allocate(
        capacity: usize,
    ) -> Result<Self, AllocError> {
        debug_assert!(capacity > 0);
        let Ok(layout) = Layout::array::<T>(capacity) else {
            return Err(AllocError::CapacityOverflow);
        };

        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(AllocError::AllocateError);
        }
        let ptr = unsafe {
            NonNull::new_unchecked(ptr.cast::<T>())
        };

        Ok(Self { ptr, cap: capacity })
    }

    unsafe fn into_box(self) -> Box<[T]> {
        let me = ManuallyDrop::new(self);
        unsafe {
            let slice = ptr::slice_from_raw_parts_mut(me.ptr.as_ptr(), me.cap);
            Box::from_raw(slice)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayQueue2;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<ArrayQueue2::<i32>>(), 24);
    }

    #[test]
    fn test_new() {
        let queue = ArrayQueue2::<i32>::new(5);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_push() {
        let mut queue = ArrayQueue2::new(3);
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
        let mut queue = ArrayQueue2::new(3);
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
// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::{fmt, ptr};
use std::alloc::{alloc, Layout};
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

/// 使用数组实现静态栈结构
pub struct ArrayStack2<T> {
    top: usize,
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

impl<T> ArrayStack2<T> {
    /// # Panics
    ///
    /// Raise panic if failed to allocate memory.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        debug_assert!(capacity > 0);

        let raw_vec = RawVec::<T>::try_allocate(capacity).expect("Failed to allocate buffer");
        let buf: Box<[T]> = unsafe { raw_vec.into_box() };

        Self {
            top: 0,
            buf,
        }
    }

    /// # Errors
    ///
    /// 当栈已满时再将元素入栈, 就会返回错误, 以及原有的元素 `value`.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.top >= self.buf.len() {
            return Err(value);
        }
        self.buf[self.top] = value;
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top > 0 {
            self.top -= 1;
            unsafe {
                Some(ptr::read(self.buf.as_ptr().wrapping_add(self.top)))
            }
        } else {
            None
        }
    }

    #[must_use]
    pub const fn top(&self) -> Option<&T> {
        if self.top > 0 {
            Some(&self.buf[self.top - 1])
        } else {
            None
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.top == 0
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.top
    }

    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.buf.len()
    }
}

impl<T: PartialEq> PartialEq for ArrayStack2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top && PartialEq::eq(&self.buf, &other.buf)
    }
}

impl<T: Eq> Eq for ArrayStack2<T> {}

impl<T: PartialOrd> PartialOrd for ArrayStack2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.buf, &other.buf)
    }
}

impl<T: Ord> Ord for ArrayStack2<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.buf, &other.buf)
    }
}

impl<T: Hash> Hash for ArrayStack2<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.buf, state);
    }
}

impl<T> FromIterator<T> for ArrayStack2<T> {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let vec: Vec<T> = iter.into_iter().collect();
        Self {
            top: vec.len(),
            buf: vec.into_boxed_slice(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for ArrayStack2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    use std::mem::size_of;

    use super::ArrayStack2;

    #[test]
    fn test_size_of_stack() {
        assert_eq!(size_of::<ArrayStack2<i32>>(), 24);
    }

    #[test]
    fn test_array_stack() {
        let mut stack: ArrayStack2<i32> = ArrayStack2::new(4);
        assert!(stack.is_empty());
        let ret = stack.push(10);
        assert!(ret.is_ok());
        let ret = stack.push(20);
        assert!(ret.is_ok());
        let ret = stack.push(30);
        assert!(ret.is_ok());
        let ret = stack.push(40);
        assert!(ret.is_ok());

        let ret = stack.push(50);
        assert_eq!(ret, Err(50));

        assert_eq!(stack.capacity(), stack.len());

        let ret = stack.pop();
        assert_eq!(ret, Some(40));
        assert!(!stack.is_empty());
        let ret = stack.pop();
        assert_eq!(ret, Some(30));
        let ret = stack.pop();
        assert_eq!(ret, Some(20));
        let ret = stack.pop();
        assert_eq!(ret, Some(10));

        assert!(stack.is_empty());

        let ret = stack.pop();
        assert_eq!(ret, None);
    }

    #[test]
    fn test_iter() {
        let numbers = [1, 1, 2, 3, 5, 8];
        let mut stack = ArrayStack2::from_iter(numbers);
        assert_eq!(stack.len(), numbers.len());

        let ret = stack.pop();
        assert_eq!(ret, Some(8));
    }

    #[test]
    fn test_eq() {
        let numbers = [1, 1, 2, 3, 5, 8];
        let stack1 = ArrayStack2::from_iter(numbers);
        let stack2 = ArrayStack2::from_iter(numbers);
        assert_eq!(stack1, stack2);
    }
}
// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::{mem, ops, ptr, slice};
use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct RingBuffer<T: Sized> {
    start: usize,
    len: usize,
    cap: usize,
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T: Sized> RingBuffer<T> {
    /// # Panics
    ///
    /// 分配内存失败时直接返回 panic
    #[must_use]
    #[inline]
    pub fn new(capacity: usize) -> Self {
        // 为了方便处理, 我们强制要求 capacity 是正数, 并且目前还没有考虑 ZST (zero sized type).
        assert!(capacity > 0);

        let layout = Layout::array::<T>(capacity).expect("Layout error");
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).expect("Failed to alloc");

        Self {
            start: 0,
            len: 0,
            cap: capacity,
            ptr: ptr.cast(),
            _marker: PhantomData,
        }
    }

    #[must_use]
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[must_use]
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    #[must_use]
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    #[must_use]
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    /// # Errors
    ///
    /// 当缓冲区已满时返回 `Err(value)`
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            Err(value)
        } else {
            unsafe {
                // 计算新元素的指针位置
                let end = (self.start + self.len) % self.cap;
                let end_ptr = self.as_mut_ptr().add(end);
                self.len += 1;
                ptr::write(end_ptr, value);
            }
            Ok(())
        }
    }

    /// 从缓冲区消费元素, 如果缓冲区已空, 就返回 `None`
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                // 计算起始元素的地址
                let start_ptr = self.as_ptr().add(self.start);
                self.start = (self.start + 1) % self.cap;
                self.len -= 1;
                Some(ptr::read(start_ptr))
            }
        }
    }

    /// 返回当前缓冲区中的元素个数
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.cap
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    // 计算当前的内存结构
    fn current_memory(&self) -> (NonNull<u8>, Layout) {
        assert_eq!(mem::size_of::<T>() % mem::align_of::<T>(), 0);
        unsafe {
            let align = mem::align_of::<T>();
            let size = mem::size_of::<T>().unchecked_mul(self.cap);
            let layout = Layout::from_size_align_unchecked(size, align);
            (self.ptr.cast(), layout)
        }
    }
}

/// 释放堆内存
impl<T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        let (ptr, layout) = self.current_memory();
        unsafe { dealloc(ptr.as_ptr(), layout) }
    }
}

/// 实现 `Deref` 和 `DerefMut` traits.
impl<T> ops::Deref for RingBuffer<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T> ops::DerefMut for RingBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}

/// 支持从迭代器初始化.
impl<T> FromIterator<T> for RingBuffer<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        // 为了实现简单, 我们重用了 vec 的 `FromIterator` 实现.
        let vec: Vec<T> = iter.into_iter().collect();
        let len = vec.len();
        let cap = vec.capacity();
        let boxed = vec.into_boxed_slice();
        let ptr = Box::leak(boxed);
        let ptr = NonNull::new(ptr.as_mut_ptr()).unwrap();
        Self {
            start: 0,
            len,
            cap,
            ptr,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::RingBuffer;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<RingBuffer<i32>>(), 32);
        assert_eq!(size_of::<Vec<i32>>(), 24);
    }

    #[test]
    fn test_ring_buffer() {
        let mut ring = RingBuffer::<i32>::new(3);
        let ret = ring.push(1);
        assert_eq!(ret, Ok(()));
        assert_eq!(ring.len(), 1);
        let ret = ring.push(2);
        assert_eq!(ret, Ok(()));
        assert_eq!(ring.len(), 2);
        let ret = ring.push(3);
        assert_eq!(ret, Ok(()));
        assert_eq!(ring.len(), 3);

        let ret = ring.push(4);
        assert_eq!(ret, Err(4));
        assert!(ring.is_full());

        let ret = ring.pop();
        assert_eq!(ret, Some(1));
        let ret = ring.pop();
        assert_eq!(ret, Some(2));
        let ret = ring.pop();
        assert_eq!(ret, Some(3));
        assert!(ring.is_empty());
    }
}

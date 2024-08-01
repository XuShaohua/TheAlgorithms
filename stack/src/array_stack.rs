// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// 使用数组实现静态栈结构
pub struct ArrayStack<T: Sized> {
    top: usize,
    buf: Box<[Option<T>]>,
}

impl<T> ArrayStack<T> {
    /// 初始化栈, 指定栈的容量
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let values: Vec<Option<T>> = (0..capacity).map(|_| None).collect();

        Self {
            top: 0,
            buf: values.into_boxed_slice(),
        }
    }

    /// 将元素入栈
    ///
    /// # Errors
    ///
    /// 当栈已满时再将元素入栈, 就会返回错误, 以及原有的元素 `value`.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.top >= self.buf.len() {
            return Err(value);
        }
        self.buf[self.top] = Some(value);
        self.top += 1;
        Ok(())
    }

    /// 将栈顶元素出栈
    ///
    /// 当栈已经空时, 返回 `None`
    pub fn pop(&mut self) -> Option<T> {
        if self.top > 0 {
            self.top -= 1;
            self.buf[self.top].take()
        } else {
            None
        }
    }

    /// 返回栈顶元素
    #[must_use]
    pub const fn top(&self) -> Option<&T> {
        if self.top > 0 {
            self.buf[self.top].as_ref()
        } else {
            None
        }
    }

    /// 检查栈是否空
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// 返回当前栈中的元素个数
    #[must_use]
    pub const fn len(&self) -> usize {
        self.top
    }

    /// 返回栈的容量
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.buf.len()
    }
}

impl<T: PartialEq> PartialEq for ArrayStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top && PartialEq::eq(&self.buf, &other.buf)
    }
}

impl<T: Eq> Eq for ArrayStack<T> {}

impl<T: PartialOrd> PartialOrd for ArrayStack<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.buf, &other.buf)
    }
}

impl<T: Ord> Ord for ArrayStack<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.buf, &other.buf)
    }
}

impl<T: Hash> Hash for ArrayStack<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.buf, state);
    }
}

impl<T> FromIterator<T> for ArrayStack<T> {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let vec: Vec<Option<T>> = iter.into_iter().map(|item| Some(item)).collect();
        Self {
            top: vec.len(),
            buf: vec.into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::array_stack::ArrayStack;

    #[test]
    fn test_size_of_stack() {
        assert_eq!(size_of::<ArrayStack<i32>>(), 24);
    }

    #[test]
    fn test_array_stack() {
        let mut stack: ArrayStack<i32> = ArrayStack::new(4);
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
}
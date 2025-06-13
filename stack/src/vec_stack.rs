// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

pub struct VecStack<T: Sized>(Vec<T>);

impl<T> Default for VecStack<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> VecStack<T> {
    /// 初始化栈, 默认的容量为 0
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// 初始化栈, 指定栈的容量, 但可以自动扩容.
    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// 将元素入栈
    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// 将栈顶元素出栈
    ///
    /// 当栈已经空时, 返回 `None`
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// 返回栈顶元素
    #[must_use]
    #[inline]
    pub fn top(&self) -> Option<&T> {
        self.0.last()
    }

    /// 检查栈是否空
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 返回当前栈中的元素个数
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// 返回栈的容量
    #[must_use]
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<T: PartialEq> PartialEq for VecStack<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl<T: Eq> Eq for VecStack<T> {}

impl<T: PartialOrd> PartialOrd for VecStack<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl<T: Ord> Ord for VecStack<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

impl<T: Hash> Hash for VecStack<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.0, state);
    }
}

impl<T> FromIterator<T> for VecStack<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let vec: Vec<T> = iter.into_iter().collect();
        Self(vec)
    }
}

impl<T: fmt::Debug> fmt::Debug for VecStack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_stack::VecStack;

    #[test]
    fn test_stack_size() {
        assert_eq!(size_of::<VecStack<i32>>(), 24);
    }

    #[test]
    fn test_array_stack() {
        let mut stack: VecStack<i32> = VecStack::with_capacity(4);
        assert!(stack.is_empty());
        stack.push(10);
        stack.push(20);
        stack.push(30);
        stack.push(40);
        stack.push(50);
        assert_eq!(stack.len(), 5);

        let ret = stack.pop();
        assert_eq!(ret, Some(50));
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
        let mut stack = VecStack::from_iter(numbers);
        assert_eq!(stack.len(), numbers.len());

        let ret = stack.pop();
        assert_eq!(ret, Some(8));
    }

    #[test]
    fn test_eq() {
        let numbers = [1, 1, 2, 3, 5, 8];
        let stack1 = VecStack::from_iter(numbers);
        let stack2 = VecStack::from_iter(numbers);
        assert_eq!(stack1, stack2);
    }
}

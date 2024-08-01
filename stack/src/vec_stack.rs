// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

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
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 返回当前栈中的元素个数
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 返回栈的容量
    #[must_use]
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
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
}
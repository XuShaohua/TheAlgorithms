// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! 使用数组实现栈结构

use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct ArrayStack<T: Sized> {
    capacity: usize,
    top: usize,
    buf: Box<[Option<T>]>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StackError {
    StackEmpty,
    StackFull,
}

impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::StackEmpty => "Stack is empty",
            Self::StackFull => "Stack is full",
        };
        write!(f, "{s}")
    }
}

impl Error for StackError {}

impl<T> ArrayStack<T> {
    /// 初始化栈, 指定栈的容量
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let values: Vec<Option<T>> = (0..capacity).map(|_| None).collect();

        Self {
            capacity,
            top: 0,
            buf: values.into_boxed_slice(),
        }
    }

    /// 将元素入栈
    ///
    /// # Errors
    /// 当栈已满时再将元素入栈, 就会返回 `StackFull` 错误.
    pub fn push(&mut self, value: T) -> Result<(), StackError> {
        if self.top >= self.capacity {
            return Err(StackError::StackFull);
        }
        self.buf[self.top] = Some(value);
        self.top += 1;
        Ok(())
    }

    /// 将栈顶元素出栈
    ///
    /// # Errors
    /// 当栈已经空时, 返回 `StackEmpty` 的错误
    pub fn pop(&mut self) -> Result<T, StackError> {
        if self.top > 0 {
            self.top -= 1;
            let value: T = self.buf[self.top].take().ok_or(StackError::StackEmpty)?;
            Ok(value)
        } else {
            Err(StackError::StackEmpty)
        }
    }

    /// 返回栈顶元素
    #[must_use]
    pub const fn peek(&self) -> Option<&T> {
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

    /// 检查栈是否已满
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.top == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use crate::array_stack::{ArrayStack, StackError};

    #[test]
    fn test_array_stack() {
        let mut stack: ArrayStack<i32> = ArrayStack::with_capacity(4);
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
        assert_eq!(ret, Err(StackError::StackFull));

        assert!(stack.is_full());
        assert_eq!(stack.len(), 4);

        let ret = stack.pop();
        assert_eq!(ret, Ok(40));
        assert!(!stack.is_full());
        let ret = stack.pop();
        assert_eq!(ret, Ok(30));
        let ret = stack.pop();
        assert_eq!(ret, Ok(20));
        let ret = stack.pop();
        assert_eq!(ret, Ok(10));

        assert!(!stack.is_full());
        assert!(stack.is_empty());

        let ret = stack.pop();
        assert_eq!(ret, Err(StackError::StackEmpty));
    }
}
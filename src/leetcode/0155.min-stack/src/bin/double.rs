// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 用两个栈来实现

#[derive(Debug, Clone)]
pub struct MinStack {
    // 正常的栈, 按入栈顺序存储.
    stack: Vec<i32>,
    // 最小栈, 里面存储当前位置的最小元素, 最小元素可能是有重复的,
    // 其长度与 `stack` 相同.
    min_stack: Vec<i32>,
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MinStack {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            stack: Vec::new(),

            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if let Some(top) = self.min_stack.last() {
            self.min_stack.push(*top.min(&val));
        } else {
            self.min_stack.push(val);
        }
    }

    pub fn pop(&mut self) {
        let _ = self.stack.pop();
        let _ = self.min_stack.pop();
    }

    #[must_use]
    pub fn top(&self) -> i32 {
        self.stack.last().copied().unwrap_or_default()
    }

    #[must_use]
    pub fn get_min(&self) -> i32 {
        self.min_stack.last().copied().unwrap_or_default()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

fn check_solution() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_min_stack() {
        check_solution();
    }
}

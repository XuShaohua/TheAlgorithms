// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 用一个栈来实现
#[derive(Debug, Clone)]
pub struct MinStack {
    // 元组: (当前的元素, 当前最小的元素)
    stack: Vec<(i32, i32)>,
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
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        if let Some(top) = self.stack.last() {
            let min = top.1.min(val);
            self.stack.push((val, min));
        } else {
            self.stack.push((val, val));
        }
    }

    pub fn pop(&mut self) {
        let _ = self.stack.pop().unwrap();
    }

    #[must_use]
    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    #[must_use]
    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
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

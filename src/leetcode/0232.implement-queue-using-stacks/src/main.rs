// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl Default for MyQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl MyQueue {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    // 将元素入栈
    pub fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    // 如果 stack_out 不为空, 从 stack_out 出栈
    // 否则, 将 stack_in 中的所有元素出栈, 并依次入栈到 stack_out
    #[must_use]
    pub fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while let Some(val) = self.stack_in.pop() {
                self.stack_out.push(val);
            }
        }

        self.stack_out.pop().unwrap()
    }

    #[must_use]
    pub fn peek(&self) -> i32 {
        if let Some(last) = self.stack_out.last() {
            return *last;
        }
        *self.stack_in.first().unwrap()
    }

    #[must_use]
    pub fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

fn check_solution() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(obj.pop(), 1);
    assert_eq!(obj.peek(), 2);
    assert!(!obj.empty());
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_queue() {
        check_solution();
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

impl Default for MyStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MyStack {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }

    // 时间复杂度: O(n)
    // 空间复杂度: O(1)
    pub fn push(&mut self, x: i32) {
        if self.queue1.is_empty() {
            self.queue1.push_back(x);
            while let Some(val) = self.queue2.pop_front() {
                self.queue1.push_back(val);
            }
        } else {
            self.queue2.push_back(x);
            while let Some(val) = self.queue1.pop_front() {
                self.queue2.push_back(val);
            }
        }
    }

    // 时间复杂度: O(1)
    // 空间复杂度: O(1)
    #[must_use]
    pub fn pop(&mut self) -> i32 {
        if self.queue1.is_empty() {
            self.queue2.pop_front().unwrap()
        } else {
            self.queue1.pop_front().unwrap()
        }
    }

    // 时间复杂度: O(1)
    // 空间复杂度: O(1)
    #[must_use]
    pub fn top(&self) -> i32 {
        if self.queue1.is_empty() {
            *self.queue2.front().unwrap()
        } else {
            *self.queue1.front().unwrap()
        }
    }

    // 时间复杂度: O(1)
    // 空间复杂度: O(1)
    #[must_use]
    pub fn empty(&self) -> bool {
        self.queue1.is_empty() && self.queue2.is_empty()
    }
}

fn check_solution() {
    let mut obj = MyStack::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(obj.pop(), 2);
    assert_eq!(obj.top(), 1);
    assert!(!obj.empty());
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_stack() {
        check_solution();
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MyStack {
    // 使用一个LIFO 队列
    queue: VecDeque<i32>,
}

impl Default for MyStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MyStack {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    // 时间复杂度: O(n)
    // 空间复杂度: O(1)
    //
    // 1. 将新的元素加入到队列尾部
    // 2. 旋转队列中的元素, 直到新元素位于队列的首部
    pub fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        let len = self.queue.len();

        for _i in 0..(len - 1) {
            if let Some(val) = self.queue.pop_front() {
                self.queue.push_back(val);
            }
        }
    }

    #[must_use]
    pub fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    #[must_use]
    pub fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    #[must_use]
    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn check_solution() {
    let mut obj = MyStack::new();
    obj.push(1);
    obj.push(2);
    obj.push(3);
    assert_eq!(obj.pop(), 3);
    assert_eq!(obj.top(), 2);
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

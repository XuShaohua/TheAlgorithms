// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RecentCounter {
    queue: VecDeque<i32>,
}

impl Default for RecentCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl RecentCounter {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    #[must_use]
    pub fn ping(&mut self, t: i32) -> i32 {
        const SPAN: i32 = 3000;
        self.queue.push_back(t);
        while let Some(front) = self.queue.front() {
            if front + SPAN < t {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}

fn check_solution() {
    let mut obj = RecentCounter::new();
    assert_eq!(obj.ping(1), 1);
    assert_eq!(obj.ping(100), 2);
    assert_eq!(obj.ping(3001), 3);
    assert_eq!(obj.ping(3002), 3);
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_counter() {
        check_solution();
    }
}

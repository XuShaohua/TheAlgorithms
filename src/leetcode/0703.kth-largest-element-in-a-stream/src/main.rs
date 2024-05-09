// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    size: usize,
}

impl KthLargest {
    /// Initializes the object with the integer k and the stream of integers nums
    #[must_use]
    #[inline]
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        assert!(k > 0);

        let size = k as usize;
        let mut heap = BinaryHeap::with_capacity(size + 1);
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > size {
                heap.pop();
            }
        }
        Self { heap, size }
    }

    /// Appends the integer val to the stream and returns the element representing the kth largest element in the stream.
    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.size {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

fn check_solution() {
    let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(obj.add(3), 4);
    assert_eq!(obj.add(5), 5);
    assert_eq!(obj.add(10), 5);
    assert_eq!(obj.add(9), 8);
    assert_eq!(obj.add(4), 8);
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_priority_queue() {
        check_solution();
    }
}

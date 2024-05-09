// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Sort
pub fn find_kth_largest1(nums: Vec<i32>, k: i32) -> i32 {
    assert!(k >= 1);
    let k = k as usize;
    assert!(nums.len() >= k);

    let mut nums = nums;
    nums.sort();
    nums[nums.len() - k]
}

// Priority Queue
pub fn find_kth_largest2(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut queue = BinaryHeap::with_capacity(k + 1);
    for num in nums {
        queue.push(Reverse(num));
        if queue.len() > k {
            queue.pop();
        }
    }
    queue.peek().unwrap().0
}

// TODO(Shaohua): Quick select

// TODO(Shaohua): Devide and conquer

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    assert_eq!(func(nums, k), 5);

    let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let k = 4;
    assert_eq!(func(nums, k), 4);
}

fn main() {
    check_solution(find_kth_largest1);
    check_solution(find_kth_largest2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_kth_largest1, find_kth_largest2};

    #[test]
    fn test_find_kth_largest1() {
        check_solution(find_kth_largest1);
    }

    #[test]
    fn test_find_kth_largest2() {
        check_solution(find_kth_largest2);
    }
}

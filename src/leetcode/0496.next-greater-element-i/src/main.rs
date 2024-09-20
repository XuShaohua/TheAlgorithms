// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

/// Brute force
pub fn next_greater_element1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 用于记录每次遍历时的状态.
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
    enum State {
        /// 没有找到相等的值
        #[default]
        NotFound,
        /// 找到了相等的值
        FoundEqual,
        /// 找到了相等的值, 同时在它的右侧也找到了更大的值
        FoundGreater,
    }

    let mut out = Vec::with_capacity(nums1.len());
    for x in nums1 {
        let mut state = State::NotFound;
        for &y in &nums2 {
            if state == State::NotFound && y == x {
                state = State::FoundEqual;
            }
            if state == State::FoundEqual && y > x {
                out.push(y);
                state = State::FoundGreater;
            }
        }
        if state != State::FoundGreater {
            out.push(-1);
        }
    }
    out
}

/// 使用单调栈
pub fn next_greater_element2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut monotonic_stack = Vec::with_capacity(nums2.len());
    assert!(!nums2.is_empty());
    let mut max_num = i32::MAX;
    let mut map = HashMap::new();

    // 构造递增式单调栈
    for &num in &nums2 {
        if num < max_num {
            // 将较小的元素入栈
            max_num = num;
            monotonic_stack.push(num);
        } else {
            // 将较小的元素出栈
            while !monotonic_stack.is_empty() && monotonic_stack[monotonic_stack.len() - 1] < num {
                let top = monotonic_stack.pop().unwrap();
                map.insert(top, num);
            }
            // 将当前元素入栈
            monotonic_stack.push(num);
        }
    }

    let out = nums1
        .iter()
        .map(|num1| map.get(num1).copied().unwrap_or(-1))
        .collect();

    out
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2];
    assert_eq!(func(nums1, nums2), [-1, 3, -1]);

    let nums1 = vec![2, 4];
    let nums2 = vec![1, 2, 3, 4];
    assert_eq!(func(nums1, nums2), [3, -1]);

    let nums1 = vec![1, 3, 5, 2, 4];
    let nums2 = vec![6, 5, 4, 3, 2, 1, 7];
    assert_eq!(func(nums1, nums2), [7, 7, 7, 7, 7]);
}

fn main() {
    check_solution(next_greater_element1);
    check_solution(next_greater_element2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, next_greater_element1, next_greater_element2};

    #[test]
    fn test_next_greater_element1() {
        check_solution(next_greater_element1);
    }

    #[test]
    fn test_next_greater_element2() {
        check_solution(next_greater_element2);
    }
}

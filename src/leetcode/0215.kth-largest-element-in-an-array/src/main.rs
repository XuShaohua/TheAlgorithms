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

// Quick select
// TIDO(Shaohua): Implement quick select
pub fn find_kth_largest3(nums: Vec<i32>, k: i32) -> i32 {
    // 与 Quicksort 类似的分区过程.
    // 选择最后一个元素为 pivot, 比它小的都放左边, 比它大的都放右边.
    fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
        let pivot_value = arr[right];
        let mut store_index = left;
        for i in left..right {
            // 放左边
            if arr[i] < pivot_value {
                arr.swap(store_index, i);
                store_index += 1;
            }
        }

        // 把 pivot 移到最终位置.
        arr.swap(store_index, right);
        left
    }

    fn kth_smallest(arr: &mut [i32], left: usize, right: usize, k: usize) -> i32 {
        assert!(k <= right - left + 1);
        if left == right {
            return arr[left];
        }

        let pivot_index: usize = partition(arr, left, right);

        // 如果分区索引等于 k, 就返回.
        //if pivot_index - left == k - 1 {
        if pivot_index == k {
            return arr[pivot_index];
        }

        //        if pivot_index - left > k - 1 {
        //            // 去访问数组的左部分
        //            return kth_smallest(arr, left, pivot_index - 1, k);
        //        } else {
        //            // 去访问数组的右部分
        //            return kth_smallest(arr, pivot_index + 1, right, k + left - 1 - pivot_index);
        //        }

        if k < pivot_index {
            kth_smallest(arr, left, pivot_index - 1, k)
        } else {
            kth_smallest(arr, pivot_index + 1, right, k)
        }
    }

    let mut nums = nums;
    let len = nums.len();
    let k = k as usize;
    let k = len - k + 1;
    kth_smallest(&mut nums, 0, len - 1, k)
}

// TODO(Shaohua): Devide and conquer

// 使用 slice::select_nth_unstable()
pub fn find_kth_largest4(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let k_small: usize = nums.len() - k as usize;
    *nums.select_nth_unstable(k_small).1
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    assert_eq!(func(nums, k), 5);

    let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let k = 4;
    assert_eq!(func(nums, k), 4);

    let nums = vec![3, 1, 2, 4];
    let k = 2;
    assert_eq!(func(nums, k), 3);
}

fn main() {
    check_solution(find_kth_largest1);
    check_solution(find_kth_largest2);
    check_solution(find_kth_largest4);
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

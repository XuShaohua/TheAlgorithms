// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute force
pub fn pivot_index1(nums: Vec<i32>) -> i32 {
    // 第一步: 计算数组中所有元素的和
    let sum: i32 = nums.iter().sum();

    // 第二步: 从左侧遍历数组, 并计算数组的前缀和 prefix sum
    // 如果前缀和 * 2 + 当前的元组, 其和等于所有元素之和,
    // 那么当前元素所在位置就是 pivot index.
    let mut prefix_sum: i32 = 0;
    for (index, num) in nums.iter().enumerate() {
        if prefix_sum * 2 + num == sum {
            return index as i32;
        }
        prefix_sum += num;
    }
    -1
}

// Prefix Sum
// 直接计算 prefix sum 和 suffix sum
pub fn pivot_index2(nums: Vec<i32>) -> i32 {
    // 第一步: 计算数组中所有元素的和, 并作为 suffix sum
    let mut suffix_sum: i32 = nums.iter().sum();

    // 第二步: 从左侧遍历数组, 并调整 prefix_sum 和 suffix_sum 的值
    // 如果它们相等了, 就终止循环
    let mut prefix_sum: i32 = 0;
    for (index, num) in nums.iter().enumerate() {
        suffix_sum -= num;
        if prefix_sum == suffix_sum {
            return index as i32;
        }
        prefix_sum += num;
    }
    -1
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 7, 3, 6, 5, 6];
    assert_eq!(func(nums), 3);

    let nums = vec![1, 2, 3];
    assert_eq!(func(nums), -1);

    let nums = vec![2, 1, -1];
    assert_eq!(func(nums), 0);

    let nums = vec![1, 0];
    assert_eq!(func(nums), 0);
}

fn main() {
    check_solution(pivot_index1);
    check_solution(pivot_index2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, pivot_index1, pivot_index2};

    #[test]
    fn test_solution1() {
        check_solution(pivot_index1);
    }

    #[test]
    fn test_solution2() {
        check_solution(pivot_index2);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute force
pub fn sorted_squares1(nums: Vec<i32>) -> Vec<i32> {
    let mut list = nums.iter().map(|n| n.pow(2)).collect::<Vec<_>>();
    list.sort();
    list
}

// 原地处理
pub fn sorted_squares2(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    for num in nums.iter_mut() {
        *num = num.pow(2);
    }
    nums.sort();
    nums
}

// 靠拢型双指针
pub fn sorted_squares3(nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut out = Vec::with_capacity(nums.len());

    // 遍历数组中所有的元素, 注意这里是 `<=` 符号.
    while left <= right {
        let left_val = nums[left].abs();
        let right_val = nums[right].abs();
        // 只移动其中的一个指针
        if left_val < right_val {
            out.push(right_val * right_val);
            right -= 1;
        } else {
            out.push(left_val * left_val);
            left += 1;
        }
    }
    // 以上是从大到小排列的, 现在进行反转
    out.into_iter().rev().collect()
}

// 优化靠拢型双指针
pub fn sorted_squares4(nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let len = nums.len();
    let mut right = len - 1;
    let mut out = vec![0; len];

    // 遍历数组中所有的元素
    for i in (0..len).rev() {
        let left_val = nums[left].abs();
        let right_val = nums[right].abs();
        // 只移动其中的一个指针
        if left_val < right_val {
            out[i] = right_val.pow(2);
            right -= 1;
        } else {
            out[i] = left_val.pow(2);
            left += 1;
        }
    }

    out
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![-4, -1, 0, 3, 10];
    assert_eq!(func(nums), vec![0, 1, 9, 16, 100]);

    let nums = vec![-7, -3, 2, 3, 11];
    assert_eq!(func(nums), vec![4, 9, 9, 49, 121]);

    let nums = vec![1];
    assert_eq!(func(nums), vec![1]);
}

fn main() {
    check_solution(sorted_squares1);
    check_solution(sorted_squares2);
    check_solution(sorted_squares3);
    check_solution(sorted_squares4);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, sorted_squares1, sorted_squares2, sorted_squares3};

    #[test]
    fn test_sorted_squares1() {
        check_solution(sorted_squares1);
    }

    #[test]
    fn test_sorted_squares2() {
        check_solution(sorted_squares2);
    }

    #[test]
    fn test_sorted_squares3() {
        check_solution(sorted_squares3);
    }
}

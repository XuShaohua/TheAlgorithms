// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 滑动窗口
pub fn longest_ones1(nums: Vec<i32>, k: i32) -> i32 {
    // 窗口右侧经过的0 的个数, 减去窗口左侧经过的0的个数, 就是需要翻转为1的个数

    let mut left = 0;
    let mut right = 0;
    let mut num_zero = 0;
    let k = k as usize;
    let mut longest_ones = 0;

    while right < nums.len() {
        // 需要翻转
        if nums[right] == 0 {
            num_zero += 1;
        }
        // 保证最大翻转次数不大于 k
        while num_zero > k {
            // 窗口左侧右移
            if nums[left] == 0 {
                num_zero -= 1;
            }
            left += 1;
        }

        // 注意边界情况.
        longest_ones = longest_ones.max(right - left + 1);

        // 窗口右侧向右移
        right += 1;
    }

    longest_ones as i32
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;
    assert_eq!(func(nums, k), 6);

    let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k = 3;
    assert_eq!(func(nums, k), 10);
}

fn main() {
    check_solution(longest_ones1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, longest_ones1};

    #[test]
    fn test_longest_ones1() {
        check_solution(longest_ones1);
    }
}

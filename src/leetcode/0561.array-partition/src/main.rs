// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Greedy
pub fn array_pair_sum1(nums: Vec<i32>) -> i32 {
    // 先对数组进行排序.
    let mut nums = nums;
    nums.sort_unstable();

    // 然后从头开始, 相邻的两个整数组成一对, 其结果应该就是最好的.
    nums.iter().step_by(2).sum()
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 4, 3, 2];
    assert_eq!(func(nums), 4);

    let nums = vec![6, 2, 6, 5, 1, 2];
    assert_eq!(func(nums), 9);
}

fn main() {
    check_solution(array_pair_sum1);
}

#[cfg(test)]
mod tests {
    use super::{array_pair_sum1, check_solution};

    #[test]
    fn test_array_pair_sum1() {
        check_solution(array_pair_sum1);
    }
}

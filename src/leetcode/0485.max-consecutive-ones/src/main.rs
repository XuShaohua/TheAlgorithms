// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Sliding window
pub fn find_max_consecutive_ones1(nums: Vec<i32>) -> i32 {
    let mut max_count = 0;
    let mut count = 0;
    for &num in &nums {
        if num != 1 {
            max_count = max_count.max(count);
            count = 0;
        } else {
            count += 1;
        }
    }
    max_count.max(count)
}

// 函数式风格
pub fn find_max_consecutive_ones2(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0, 0), |(accum, result), num| {
            let accum = accum * num + num;
            let result = result.max(accum);
            (accum, result)
        })
        .1
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 1, 0, 1, 1, 1];
    assert_eq!(func(nums), 3);
    let nums = vec![1, 0, 1, 1, 0, 1];
    assert_eq!(func(nums), 2);
}

fn main() {
    check_solution(find_max_consecutive_ones1);
    check_solution(find_max_consecutive_ones2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_max_consecutive_ones1, find_max_consecutive_ones2};

    #[test]
    fn test_find_max_consecutive_ones1() {
        check_solution(find_max_consecutive_ones1);
    }

    #[test]
    fn test_find_max_consecutive_ones2() {
        check_solution(find_max_consecutive_ones2);
    }
}

// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// num ^ num = 0
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in &nums {
        ans ^= num;
    }
    ans
}

pub fn single_number2(nums: Vec<i32>) -> i32 {
    const DIGIT_LEN: usize = 32;

    let mut ans = 0;
    for i in 0..DIGIT_LEN {
        let sum = nums.iter().map(|num| num >> i & 1).sum::<i32>();
        ans |= (sum % 2) << i;
    }
    ans
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 2, 1];
    assert_eq!(func(nums), 1);

    let nums = vec![4, 1, 2, 1, 2];
    assert_eq!(func(nums), 4);

    let nums = vec![1];
    assert_eq!(func(nums), 1);
}

fn main() {
    check_solution(single_number);
    check_solution(single_number2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, single_number, single_number2};

    #[test]
    fn test_single_number1() {
        check_solution(single_number);
    }

    #[test]
    fn test_single_number2() {
        check_solution(single_number2);
    }
}

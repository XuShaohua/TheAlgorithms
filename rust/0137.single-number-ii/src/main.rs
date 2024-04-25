// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// bit vector
pub fn single_number(nums: Vec<i32>) -> i32 {
    const DIGIT_LEN: usize = 32;

    let mut ans = 0;
    // 遍历所有比特位
    for i in 0..DIGIT_LEN {
        // 计数所有整数在该比特位处的和
        let sum = nums.iter().map(|num| num >> i & 1).sum::<i32>();
        // bit 的值就是落单的数在该比特位处的比特值.
        let bit = sum % 3;
        ans |= bit << i;
    }
    ans
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 2, 3, 2];
    assert_eq!(func(nums), 3);

    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    assert_eq!(func(nums), 99);
}

fn main() {
    check_solution(single_number);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, single_number};

    #[test]
    fn test_single_number() {
        check_solution(single_number);
    }
}
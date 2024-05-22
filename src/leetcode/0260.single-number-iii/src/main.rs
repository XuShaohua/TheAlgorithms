// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// bit-manipulation
// XOR
pub fn single_number1(nums: Vec<i32>) -> Vec<i32> {
    // xor 是 a1 和 a2 共有的比特位
    let mut xor: i32 = 0;
    for &num in &nums {
        xor ^= num;
    }

    let mut k: i32 = 1;
    loop {
        if (xor & k) != 0 {
            break;
        }
        k *= 2;
    }

    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    for &num in &nums {
        if (num & k) == 0 {
            a1 ^= num;
        } else {
            a2 ^= num;
        }
    }

    // 排序
    if a1 > a2 {
        (a1, a2) = (a2, a1);
    }
    vec![a1, a2]
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 1, 3, 2, 5];
    assert_eq!(func(nums), [3, 5]);

    let nums = vec![-1, 0];
    assert_eq!(func(nums), [-1, 0]);

    let nums = vec![0, 1];
    assert_eq!(func(nums), [0, 1]);
}

fn main() {
    check_solution(single_number1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, single_number1};

    #[test]
    fn test_single_number1() {
        check_solution(single_number1);
    }
}

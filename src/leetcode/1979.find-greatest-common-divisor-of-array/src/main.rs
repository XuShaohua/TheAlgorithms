// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

fn gcd(a: i32, b: i32) -> i32 {
    debug_assert!(a > b && b >= 0);
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn find_gcd1(nums: Vec<i32>) -> i32 {
    debug_assert!(nums.len() >= 2);
    let mut min_val = nums[0];
    let mut max_val = nums[0];
    for num in nums {
        if num < min_val {
            min_val = num;
        }
        if num > max_val {
            max_val = num;
        }
    }
    gcd(max_val, min_val)
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 5, 6, 9, 10];
    assert_eq!(func(nums), 2);
}

fn main() {
    check_solution(find_gcd1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_gcd1};

    #[test]
    fn test_find_gcd1() {
        check_solution(find_gcd1);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub struct Solution {
    bad_version: i32,
}

impl Solution {
    fn new(bad_version: i32) -> Self {
        Self { bad_version }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        self.bad_version == version
    }

    // Binary Search
    pub fn first_bad_version1(&self, n: i32) -> i32 {
        debug_assert!(n >= 1);
        let mut left = 1;
        let mut right = n;

        while left <= right {
            let middle = left + (right - left) / 2;
            if self.isBadVersion(middle) {
                // [middle..right] 区间都是有问题的版本
                right = middle - 1;
            } else {
                // [left..middle] 区间都是没有问题的版本
                left = middle + 1;
            }
        }
        left
    }
}

pub type SolutionFn = fn(&Solution, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let n = 5;
    let bad = 4;
    let solution = Solution::new(bad);
    assert_eq!(solution.first_bad_version1(n), bad);

    let n = 1;
    let bad = 1;
    let solution = Solution::new(bad);
    assert_eq!(func(&solution, n), bad);
}

fn main() {
    check_solution(Solution::first_bad_version1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, Solution};

    #[test]
    fn test_first_bad_version1() {
        check_solution(Solution::first_bad_version1);
    }
}

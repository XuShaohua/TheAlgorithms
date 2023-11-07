// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Maximum of subsequence sum problem:
//!
//! Given (possibly negative) integers a1 , a2, . . . , an , find the maximum value of
//! maximum subsequence sum is 0 if all the integers are negative.)
//!
//! Example:
//!
//! For input -2, 11, -4, 13, -5, -2, the answer is 20 (a 2 through a4).

fn method1(nums: &[i32]) -> i32 {
    let mut max_sum = 0;
    let len = nums.len();
    for i in 0..len {
        for j in i..len {
            let this_sum = nums[i..=j].iter().sum();
            max_sum = max_sum.max(this_sum);
        }
    }

    max_sum
}

fn main() {
    const NUMS: &[i32] = &[4, -3, 5, -2, -1, 2, 6, -2];
    let r = method1(NUMS);
    println!("best sum: {r}");
}

#[cfg(test)]
mod tests {
    use super::method1;

    const NUMS: &[i32] = &[4, -3, 5, -2, -1, 2, 6, -2];

    #[test]
    fn test_method1() {
        let r = method1(NUMS);
        assert_eq!(r, 11);
    }
}

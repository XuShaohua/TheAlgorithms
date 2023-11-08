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

fn method1(nums: &[i32]) -> (i32, usize, usize) {
    let mut max_sum = 0;
    let mut best_i = 0;
    let mut best_j = 0;
    let len = nums.len();
    for i in 0..len {
        for j in i..len {
            let this_sum = nums[i..=j].iter().sum();
            max_sum = max_sum.max(this_sum);
            if this_sum > max_sum {
                max_sum = this_sum;
                best_i = i;
                best_j = j;
            }
        }
    }

    (max_sum, best_i, best_j)
}

fn method2(nums: &[i32]) -> (i32, usize, usize) {
    let mut max_sum = 0;
    let mut best_i = 0;
    let mut best_j = 0;
    let len = nums.len();
    for i in 0..len {
        let mut this_sum = 0;
        for j in i..len {
            this_sum += nums[j];
            if this_sum > max_sum {
                max_sum = this_sum;
                best_i = i;
                best_j = j;
            }
        }
    }

    (max_sum, best_i, best_j)
}

// devide and conquer
fn method3(nums: &[i32]) -> (i32, usize, usize) {
    method3_helper(nums, 0, nums.len() - 1)
}

fn method3_helper(nums: &[i32], left: usize, right: usize) -> (i32, usize, usize) {
    if left == right {
        if nums[left] > 0 {
            return (nums[left], left, left);
        } else {
            return (0, left, left);
        }
    }

    // FIXME(Shaohua): Overflow
    let center_index = (left + right) / 2;
    let (max_left_sum, _, _) = method3_helper(nums, left, center_index);
    let (max_right_sum, _, _) = method3_helper(nums, center_index + 1, right);

    let mut max_left_border_sum = 0;
    let mut left_border_sum = 0;
    let mut left_border = 0;
    for i in (left..=center_index).rev() {
        left_border_sum += nums[i];
        if left_border_sum > max_left_border_sum {
            max_left_border_sum = left_border_sum;
            left_border = i;
        }
    }

    let mut max_right_border_sum = 0;
    let mut right_border_sum = 0;
    let mut right_border = 0;
    for i in center_index + 1..=right {
        right_border_sum += nums[i];
        if right_border_sum > max_right_border_sum {
            max_right_border_sum = right_border_sum;
            right_border = i;
        }
    }
    let border_sum = max_left_border_sum + max_right_border_sum;
    let new_sum: i32 = max_left_sum.max(max_right_sum.max(border_sum));

    if new_sum == max_left_sum {
        (new_sum, left, center_index)
    } else if new_sum == max_right_sum {
        (new_sum, center_index + 1, right)
    } else {
        (new_sum, left_border, right_border)
    }
}

// Online algorithm
fn method4(nums: &[i32]) -> (i32, usize, usize) {
    let mut max_sum = 0;
    let mut best_i = 0;
    let mut best_j = 0;
    let len = nums.len();

    let mut this_sum = 0;
    let mut i = 0;
    for j in 0..len {
        this_sum += nums[j];
        if this_sum > max_sum {
            max_sum = this_sum;
            best_i = i;
            best_j = j;
        } else if this_sum < 0 {
            i = j + 1;
            this_sum = 0;
        }
    }
    (max_sum, best_i, best_j)
}

fn main() {
    const NUMS: &[i32] = &[4, -3, 5, -2, -1, 2, 6, -2];
    let (r, best_i, best_j) = method1(NUMS);
    println!("best sum: {r}, ({best_i}, {best_j})");
    let (r, best_i, best_j) = method2(NUMS);
    println!("best sum: {r}, ({best_i}, {best_j})");
    let (r, best_i, best_j) = method3(NUMS);
    println!("best sum: {r}, ({best_i}, {best_j})");
    let (r, best_i, best_j) = method4(NUMS);
    println!("best sum: {r}, ({best_i}, {best_j})");
}

#[cfg(test)]
mod tests {
    use super::method1;

    const NUMS: &[i32] = &[4, -3, 5, -2, -1, 2, 6, -2];

    #[test]
    fn test_method1() {
        let (r, _, _) = method1(NUMS);
        assert_eq!(r, 11);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct NumArray {
    prefix_sum: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        assert!(!nums.is_empty());
        let mut prefix_sum = vec![0; nums.len()];
        prefix_sum[0] = nums[0];

        for i in 1..nums.len() {
            prefix_sum[i] = nums[i] + prefix_sum[i - 1];
        }
        Self { prefix_sum }
    }

    #[must_use]
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        debug_assert!(left <= right);

        if left == 0 {
            self.prefix_sum[right]
        } else {
            self.prefix_sum[right] - self.prefix_sum[left - 1]
        }
    }
}

fn check_solution() {
    let na = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(na.sum_range(0, 2), 1);
    assert_eq!(na.sum_range(2, 5), -1);
    assert_eq!(na.sum_range(0, 5), -3);
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_num_array() {
        check_solution();
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::ptr_arg)]

// 快慢型双指针
// 类似于 Bubble sort
pub fn move_zeroes1(nums: &mut Vec<i32>) {
    let len = nums.len();
    let mut left = 0;
    let mut right = 0;
    // 把数组分成两个区, 左侧是不含0的, 右侧是原有的顺序
    while left < len && right < len {
        if nums[left] == 0 {
            // 找到右侧第一个不为0的数
            while right < len && nums[right] == 0 {
                right += 1;
            }
            if right < len {
                nums.swap(left, right);
            } else {
                break;
            }
            left += 1;
        } else {
            if left == right {
                right += 1;
            }
            left += 1;
        }
    }
}

pub type SolutionFn = fn(&mut Vec<i32>);

fn check_solution(func: SolutionFn) {
    let mut nums = vec![0, 1, 0, 3, 12];
    func(&mut nums);
    assert_eq!(nums, &[1, 3, 12, 0, 0]);

    let mut nums = vec![0];
    func(&mut nums);
    assert_eq!(nums, &[0]);

    let mut nums = vec![0, 0, 1];
    func(&mut nums);
    assert_eq!(nums, &[1, 0, 0]);

    let mut nums = vec![1, 0];
    func(&mut nums);
    assert_eq!(nums, &[1, 0]);
}

fn main() {
    check_solution(move_zeroes1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, move_zeroes1};

    #[test]
    fn test_move_zeroes1() {
        check_solution(move_zeroes1);
    }
}

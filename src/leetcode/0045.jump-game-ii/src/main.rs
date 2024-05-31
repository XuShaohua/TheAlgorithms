// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Greedy
pub fn jump1(nums: Vec<i32>) -> i32 {
    debug_assert!((1..=10_000).contains(&nums.len()));

    // 当前最远的跳跃距离
    let mut max_pos: usize = 0;

    // 当前的达到的位置
    let mut current_pos: usize = 0;

    // 跳跃次数
    let mut jump_count: i32 = 0;

    for (index, &num) in nums.iter().enumerate() {
        // 如果已经到达了终点, 就终止循环.
        if current_pos + 1 >= nums.len() {
            break;
        }

        // 更新最远跳跃距离.
        max_pos = max_pos.max(index + num as usize);

        if index >= current_pos {
            current_pos = max_pos;
            jump_count += 1;
        }
    }

    jump_count
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 3, 1, 1, 4];
    assert_eq!(func(nums), 2);

    let nums = vec![2, 3, 0, 1, 4];
    assert_eq!(func(nums), 2);

    let nums = vec![1];
    assert_eq!(func(nums), 0);

    let nums = vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3];
    assert_eq!(func(nums), 2);
}

fn main() {
    check_solution(jump1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, jump1};

    #[test]
    fn test_jump1() {
        check_solution(jump1);
    }
}

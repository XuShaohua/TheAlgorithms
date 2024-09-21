// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::io::{stdin, BufRead};

// 滑动窗口
fn solution() {
    // 读取输入
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let nums: Vec<i32> = line.trim().split(',').map(|x| x.parse().unwrap()).collect();

    line.clear();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let expected_sum: i32 = line.trim().parse().unwrap();

    // 最长子序的长度
    let mut subarray_max_len: usize = 0;
    // 用于控制窗口左侧两侧的位置
    let mut left: usize = 0;
    let mut right: usize = 0;
    // 计算当前子序列的和
    let mut current_sum: i32 = 0;

    while left <= right && right < nums.len() {
        match current_sum.cmp(&expected_sum) {
            Ordering::Less => {
                // 子序列的和太小, 将窗口右侧向右移
                current_sum += nums[right];
                right += 1;
            }
            Ordering::Equal => {
                // 和相等, 计算当前的子序列长度
                let current_length = right - left;
                subarray_max_len = subarray_max_len.max(current_length);
                // 并把窗口右侧向右移
                current_sum += nums[right];
                right += 1;
            }
            Ordering::Greater => {
                // 子序列的和太大, 将窗口左侧向右移
                current_sum -= nums[left];
                left += 1;
            }
        }
    }

    // 打印结果
    println!("{subarray_max_len}");
}

fn main() {
    solution();
}

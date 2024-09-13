// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 动态规划
pub fn solution1(m: i32, n: i32) -> i32 {
    let rows = m as usize;
    let columns = n as usize;
    // 创建二维数组, 用于记录每个坐标点可能的路径数量
    let mut dp = vec![vec![0_i32; columns]; rows];

    // 横向移动的方法只有一种
    for column in 0..columns {
        dp[0][column] = 1;
    }

    // 纵向移动的方法只有一种
    #[allow(clippy::needless_range_loop)]
    for row in 0..rows {
        dp[row][0] = 1;
    }

    for row in 1..rows {
        for column in 1..columns {
            // 观察规律, 找到以下关系:
            // 第 (row, column)  坐标点, 可能严自 (row, column - 1) 向右移;
            // 也可能来自 (row - 1, column) 向下移.
            dp[row][column] = dp[row][column - 1] + dp[row - 1][column];
        }
    }

    dp[rows - 1][columns - 1]
}

pub type SolutionFn = fn(i32, i32) -> i32;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(3, 7), 28);
    assert_eq!(func(3, 2), 3);
    assert_eq!(func(7, 3), 28);
    assert_eq!(func(3, 3), 6);
}

fn main() {
    check_solution(solution1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, solution1};

    #[test]
    fn test_solution1() {
        check_solution(solution1);
    }
}

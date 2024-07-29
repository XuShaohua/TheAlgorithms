// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Prefix Sum
pub fn pivot_integer1(n: i32) -> i32 {
    let len = (n + 1) as usize;

    // 构造前缀和数组
    let mut prefix_sum = vec![0_i32; len];
    prefix_sum[0] = 0;
    prefix_sum[1] = 1;
    for i in 2..len {
        prefix_sum[i] = prefix_sum[i - 1] + i as i32;
    }

    let total_sum = prefix_sum[len - 1];

    // 遍历前缀和数组
    for i in 0..len {
        // 满足左侧之和等于右侧之后
        if prefix_sum[i] * 2 == total_sum + i as i32 {
            return i as i32;
        }
    }

    -1
}

// Two pointers
pub fn pivot_integer2(n: i32) -> i32 {
    let mut left_sum = 0;
    let mut right_sum = n * (n + 1) / 2;

    // 注意遍历的范围, 是 [1..=n]
    for i in 1..=n {
        if left_sum + i == right_sum {
            return i;
        }
        left_sum += i;
        right_sum -= i;
    }
    -1
}

// Sqrt
pub fn pivot_integer3(n: i32) -> i32 {
    let sum = n * (n + 1) / 2;
    let sqrt = (sum as f64).sqrt() as i32;
    if sqrt * sqrt == sum {
        sqrt
    } else {
        -1
    }
}

pub type SolutionFn = fn(i32) -> i32;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(8), 6);
    assert_eq!(func(1), 1);
    assert_eq!(func(4), -1);
}

fn main() {
    check_solution(pivot_integer1);
    check_solution(pivot_integer2);
    check_solution(pivot_integer3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, pivot_integer1, pivot_integer2, pivot_integer3};

    #[test]
    fn test_pivot_integer1() {
        check_solution(pivot_integer1);
    }

    #[test]
    fn test_pivot_integer2() {
        check_solution(pivot_integer2);
    }

    #[test]
    fn test_pivot_integer3() {
        check_solution(pivot_integer3);
    }
}

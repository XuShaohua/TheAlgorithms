// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute force
pub fn daily_temperatures1(temperatures: Vec<i32>) -> Vec<i32> {
    assert!(!temperatures.is_empty());

    let mut out = Vec::new();
    let len = temperatures.len();
    for i in 0..len {
        let mut diff = 0;
        for j in i..len {
            if temperatures[j] > temperatures[i] {
                diff = j - i;
                break;
            }
        }
        out.push(diff as i32);
    }
    out
}

// Stack
// 单调递增栈
pub fn daily_temperatures2(temperatures: Vec<i32>) -> Vec<i32> {
    assert!(!temperatures.is_empty());

    let len = temperatures.len();
    let mut out = vec![0; len];
    // 存储温度值所在数组的下标
    let mut stack: Vec<usize> = Vec::new();

    // 遍历数组
    for (index, &num) in temperatures.iter().enumerate() {
        // 如果栈不为空, 且当前温度比栈顶的温度值高, 则先将栈顶元素出栈.
        while !stack.is_empty() && num > temperatures[*stack.last().unwrap()] {
            let top_index = stack.pop().unwrap();
            // 计算下标的偏移量.
            out[top_index] = (index - top_index) as i32;
        }
        // 将当前温度值的下标入栈
        stack.push(index);
    }

    out
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    assert_eq!(&func(temperatures), &[1, 1, 4, 2, 1, 1, 0, 0]);

    let temperatures = vec![30, 40, 50, 60];
    assert_eq!(&func(temperatures), &[1, 1, 1, 0]);

    let temperatures = vec![30, 60, 90];
    assert_eq!(&func(temperatures), &[1, 1, 0]);
}

fn main() {
    check_solution(daily_temperatures1);
    check_solution(daily_temperatures2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, daily_temperatures1, daily_temperatures2};

    #[test]
    fn test_daily_temperatures1() {
        check_solution(daily_temperatures1);
    }

    #[test]
    fn test_daily_temperatures2() {
        check_solution(daily_temperatures2);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashSet;

// Brute force
pub fn count_triples1(n: i32) -> i32 {
    debug_assert!((1..=250).contains(&n));

    // 用于缓存平方
    let set: HashSet<i32> = (1..=n).map(|x| x * x).collect();
    let mut count: i32 = 0;

    // a^2 + b^2 = c^2, a <= b < c
    for a in 1..n {
        for b in 1..n {
            let square = a * a + b * b;
            if set.contains(&square) {
                count += 1;
            }
        }
    }

    count
}

// 缓存所有平方数
pub fn count_triples2(n: i32) -> i32 {
    debug_assert!((1..=250).contains(&n));

    // 用于缓存平方
    let vec: Vec<i32> = (0..=n).map(|x| x * x).collect();
    let set: HashSet<i32> = vec.iter().copied().collect();
    let max_square: i32 = vec[vec.len() - 1];
    let mut count: i32 = 0;

    // a^2 + b^2 = c^2, a <= b < c
    for a in 1..n {
        for b in 1..n {
            let square = vec[a as usize] + vec[b as usize];
            if square > max_square {
                break;
            }
            if set.contains(&square) {
                count += 1;
            }
        }
    }

    count
}

// 减少计算量
pub fn count_triples3(n: i32) -> i32 {
    let mut count: i32 = 0;

    // 先从 c 开始遍历, 1..4 被跳过
    for c in 5..=n {
        let squared_c: i32 = c * c;

        let half_squared_c: i32 = squared_c / 2;

        // 遍历 a
        for a in 1..n {
            let squared_a: i32 = a * a;
            // 我们只计算 a <= b < c, 否则就重复计数了.
            if squared_a <= half_squared_c {
                let squared_b: i32 = squared_c - squared_a;

                // 这里, 只有 b 是整数时, 才成立.
                let b: i32 = (squared_b as f64).sqrt() as i32;
                if b * b == squared_b {
                    // 一次加入两组数, (a, b, c), (b, a, c)
                    count += 2;
                }
            } else {
                // 太大了, 忽略后面的 a
                break;
            }
        }
    }

    count
}

pub type SolutionFn = fn(i32) -> i32;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(5), 2);
    assert_eq!(func(10), 4);
}

fn main() {
    check_solution(count_triples1);
    check_solution(count_triples2);
    check_solution(count_triples3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, count_triples1, count_triples2, count_triples3};

    #[test]
    fn test_count_triples1() {
        check_solution(count_triples1);
    }

    #[test]
    fn test_count_triples2() {
        check_solution(count_triples2);
    }

    #[test]
    fn test_count_triples3() {
        check_solution(count_triples3);
    }
}

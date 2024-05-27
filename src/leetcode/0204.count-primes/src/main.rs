// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Enumeration
// 筛选法求质数
pub fn count_primes1(n: i32) -> i32 {
    debug_assert!((0..=500_000).contains(&n));

    if n <= 1 {
        return 0;
    }

    let n = n as usize;
    let mut primes: Vec<bool> = vec![true; n + 1];

    //平方根
    let mut prime_count: i32 = 0;
    for i in 2..n {
        // 如果这个数值是和数, 就直接跳过它.
        if !primes[i] {
            continue;
        }
        prime_count += 1;

        let mut pos = i;
        while pos < n {
            primes[pos] = false;
            pos += i;
        }
    }

    prime_count
}

pub type SolutionFn = fn(i32) -> i32;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(10), 4);
    assert_eq!(func(0), 0);
    assert_eq!(func(1), 0);
    assert_eq!(func(2), 0);
    assert_eq!(func(3), 1);
}

fn main() {
    check_solution(count_primes1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, count_primes1};

    #[test]
    fn test_count_primes1() {
        check_solution(count_primes1);
    }
}

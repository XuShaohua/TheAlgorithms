// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute Force
pub fn count_bits1(n: i32) -> Vec<i32> {
    assert!(n >= 0);
    (0..=n).map(|i| i.count_ones() as i32).collect()
}

// Dynamic Programming
pub fn count_bits2(n: i32) -> Vec<i32> {
    assert!(n >= 0);
    let mut vec = vec![0; n as usize + 1];
    for i in 0..=n {
        let i_usize = i as usize;
        // f(n) = f(n/2) + lsb
        // 下面两行的写法是等效的:
        //vec[i_usize] = vec[i_usize / 2] + i % 2;
        vec[i_usize] = vec[i_usize >> 1] + (i & 1);
    }
    vec
}

pub type SolutionFn = fn(i32) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(2), vec![0, 1, 1]);
    assert_eq!(func(5), vec![0, 1, 1, 2, 1, 2]);
}

fn main() {
    check_solution(count_bits1);
    check_solution(count_bits2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, count_bits1, count_bits2};

    #[test]
    fn test_count_bits1() {
        check_solution(count_bits1);
    }

    #[test]
    fn test_count_bits2() {
        check_solution(count_bits2);
    }
}

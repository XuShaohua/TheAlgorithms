// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn count_bits(n: i32) -> Vec<i32> {
    assert!(n >= 0);
    (0..=n).map(|i| i.count_ones() as i32).collect()
}

pub type SolutionFn = fn(i32) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(2), vec![0, 1, 1]);
    assert_eq!(func(5), vec![0, 1, 1, 2, 1, 2]);
}

fn main() {
    check_solution(count_bits);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, count_bits};

    #[test]
    fn test_count_bits() {
        check_solution(count_bits);
    }
}

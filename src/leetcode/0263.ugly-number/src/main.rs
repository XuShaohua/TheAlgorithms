// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn is_ugly1(n: i32) -> bool {
    let mut n = n;
    while n != 0 && n % 2 == 0 {
        n /= 2;
    }
    while n != 0 && n % 3 == 0 {
        n /= 3;
    }
    while n != 0 && n % 5 == 0 {
        n /= 5;
    }
    n == 1
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    assert!(func(6));
    assert!(func(1));
    assert!(!func(14));
    assert!(!func(0));
}

fn main() {
    check_solution(is_ugly1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_ugly1};

    #[test]
    fn test_is_ugly1() {
        check_solution(is_ugly1);
    }
}

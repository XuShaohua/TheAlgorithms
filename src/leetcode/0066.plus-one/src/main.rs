// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn plus_one1(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut carry = 1;
    for digit in digits.iter_mut().rev() {
        *digit += carry;
        carry = *digit / 10;
        *digit %= 10;
    }
    if carry == 1 {
        digits.insert(0, carry);
    }
    digits
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let digits = vec![1, 2, 3];
    assert_eq!(func(digits), vec![1, 2, 4]);

    let digits = vec![4, 3, 2, 1];
    assert_eq!(func(digits), vec![4, 3, 2, 2]);

    let digits = vec![0];
    assert_eq!(func(digits), vec![1]);
}

fn main() {
    check_solution(plus_one1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, plus_one1};

    #[test]
    fn test_plus_one() {
        check_solution(plus_one1);
    }
}

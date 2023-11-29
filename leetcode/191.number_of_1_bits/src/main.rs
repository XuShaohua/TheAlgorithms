// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [Number of 1 bits](https://leetcode.com/problems/number-of-1-bits)

fn solution1(n: u32) -> i32 {
    let mut n = n;
    let mut count = 0;
    while n != 0 {
        count += n % 2;
        n /= 2;
    }
    count as i32
}

fn solution2(n: u32) -> i32 {
    n.count_ones() as i32
}

fn main() {
    let n = 0b00000000000000000000000000001011;
    assert_eq!(solution1(n), 3);

    let n = 0b00000000000000000000000010000000;
    assert_eq!(solution1(n), 1);

    let n = 0b11111111111111111111111111111101;
    assert_eq!(solution1(n), 31);

    let n = 0b11111111111111111111111111111101;
    assert_eq!(solution2(n), 31);
}

#[cfg(test)]
mod tests {
    use super::{solution1, solution2};

    #[test]
    fn test_solution1() {
        let n = 0b00000000000000000000000000001011;
        assert_eq!(solution1(n), 3);

        let n = 0b00000000000000000000000010000000;
        assert_eq!(solution1(n), 1);

        let n = 0b11111111111111111111111111111101;
        assert_eq!(solution1(n), 31);
    }

    #[test]
    fn test_solution2() {
        let n = 0b00000000000000000000000000001011;
        assert_eq!(solution2(n), 3);

        let n = 0b00000000000000000000000010000000;
        assert_eq!(solution2(n), 1);

        let n = 0b11111111111111111111111111111101;
        assert_eq!(solution2(n), 31);
    }
}

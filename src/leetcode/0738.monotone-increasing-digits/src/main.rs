// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Bruteforce
pub fn monotone_increasing_digits1(n: i32) -> i32 {
    #[must_use]
    fn is_monotone_incr_digits(mut num: i32) -> bool {
        debug_assert!(num >= 0);

        if num <= 9 {
            return true;
        }

        let mut last_digit: i32 = i32::MAX;
        while num != 0 {
            let digit: i32 = num % 10;
            if last_digit < digit {
                return false;
            }
            last_digit = digit;
            num /= 10;
        }
        true
    }

    debug_assert!(n >= 0);

    for num in (0..=n).rev() {
        if is_monotone_incr_digits(num) {
            return num;
        }
    }

    0
}

// Greedy
// 尽量跳过更多无用的数, 减少计算量.
#[allow(clippy::needless_range_loop)]
pub fn monotone_increasing_digits2(n: i32) -> i32 {
    #[must_use]
    fn from_digits(digits: &[i32]) -> i32 {
        let mut num = 0;
        for &digit in digits {
            num = num * 10 + digit;
        }
        num
    }

    fn to_digits(mut num: i32, digits: &mut Vec<i32>) {
        debug_assert!(num >= 0);
        if num <= 9 {
            digits.push(num);
            return;
        }

        while num != 0 {
            let digit: i32 = num % 10;
            digits.push(digit);
            num /= 10;
        }
        digits.reverse();
    }

    debug_assert!(n >= 0);

    // 转换成整数位.
    let mut digits: Vec<i32> = Vec::with_capacity(10);
    to_digits(n, &mut digits);

    // 如果当前整数位比后面的整数位大, 那就直接把当前整数位减去1, 然后重置后面的所有整数位为9.
    let mut index: usize = 0;
    let len: usize = digits.len();
    while index < len - 1 {
        if digits[index] > digits[index + 1] {
            // 把当前整数位减去1.
            digits[index] -= 1;

            // 将后面的所有整数位重置为9.
            for right in (index + 1)..len {
                digits[right] = 9;
            }

            // 重头再过一遍.
            index = 0;
        } else {
            index += 1;
        }
    }

    //println!("digits: {digits:?}");
    // 重新构造整数
    from_digits(&digits)
}

pub type SolutionFn = fn(i32) -> i32;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(10), 9);
    assert_eq!(func(1234), 1234);
    assert_eq!(func(332), 299);
    assert_eq!(func(982736877), 899999999);
}

fn main() {
    //check_solution(monotone_increasing_digits1);
    check_solution(monotone_increasing_digits2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, monotone_increasing_digits1, monotone_increasing_digits2};

    #[test]
    fn test_monotone_increasing_digits1() {
        check_solution(monotone_increasing_digits1);
    }

    #[test]
    fn test_monotone_increasing_digits2() {
        check_solution(monotone_increasing_digits2);
    }
}

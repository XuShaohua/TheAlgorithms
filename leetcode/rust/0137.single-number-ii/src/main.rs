// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

const DIGIT_LEN: usize = 32;
pub type Digits = [i32; DIGIT_LEN];

fn to_digits(mut num: i32) -> Digits {
    let mut digits = [0; DIGIT_LEN];
    for digit in digits.iter_mut() {
        *digit = num % 2;
        num /= 2;
    }
    digits
}

fn from_digits(digits: Digits) -> i32 {
    let mut num = 0;
    for i in (0..DIGIT_LEN).rev() {
        num = num * 2 + digits[i];
    }
    num
}

// bit vector
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut all_digits: Digits = [0; DIGIT_LEN];
    for num in &nums {
        let num_digits = to_digits(*num);
        for i in 0..DIGIT_LEN {
            all_digits[i] = (all_digits[i] + num_digits[i]) % 3;
        }
    }
    from_digits(all_digits)
}

fn main() {
    let nums = vec![2, 2, 3, 2];
    assert_eq!(single_number(nums), 3);

    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    assert_eq!(single_number(nums), 99);
}

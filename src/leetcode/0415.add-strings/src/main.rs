// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 先转换成整数
// 这里会有整数溢出问题.
pub fn add_strings1(num1: String, num2: String) -> String {
    debug_assert!(!num1.is_empty());
    debug_assert!(!num2.is_empty());

    let num1_i64: i64 = num1.parse().unwrap();
    let num2_i64: i64 = num2.parse().unwrap();
    let sum: i64 = num1_i64 + num2_i64;
    sum.to_string()
}

// 遍历两个字符串
// 这里, 使用了三个数组.
pub fn add_strings2(num1: String, num2: String) -> String {
    debug_assert!(!num1.is_empty());
    debug_assert!(!num2.is_empty());

    let mut digits: Vec<u8> = Vec::with_capacity(num1.len());
    // 先把两个字符串转成数组, 并进行反转.
    let mut num1_bytes: Vec<u8> = num1.into_bytes();
    num1_bytes.reverse();
    let mut num2_bytes: Vec<u8> = num2.into_bytes();
    num2_bytes.reverse();

    let mut carry: u8 = 0;
    let max_len: usize = num1_bytes.len().max(num2_bytes.len());

    // 遍历两个字符串.
    for i in 0..max_len {
        // 从每个数组中的当前索引位置读取数值, 并计算和
        let mut digit: u8 = carry;
        if let Some(byte1) = num1_bytes.get(i) {
            digit += byte1 - b'0';
        }
        if let Some(byte2) = num2_bytes.get(i) {
            digit += byte2 - b'0';
        }

        // 处理进位.
        carry = digit / 10;
        let remainder: u8 = digit % 10;
        digits.push(remainder + b'0');
    }
    // 别忘了最后一个进位.
    if carry != 0 {
        digits.push(carry + b'0');
    }

    // 反转
    digits.reverse();

    String::from_utf8(digits).unwrap()
}

pub type SolutionFn = fn(String, String) -> String;

fn check_solution(func: SolutionFn) {
    let num1 = "11".to_owned();
    let num2 = "123".to_owned();
    assert_eq!(func(num1, num2), "134".to_owned());

    let num1 = "456".to_owned();
    let num2 = "77".to_owned();
    assert_eq!(func(num1, num2), "533".to_owned());

    let num1 = "0".to_owned();
    let num2 = "0".to_owned();
    assert_eq!(func(num1, num2), "0".to_owned());

    let num1 = "6913259244".to_owned();
    let num2 = "71103343".to_owned();
    assert_eq!(func(num1, num2), "6984362587".to_owned());

    let num1 = "1".to_owned();
    let num2 = "9".to_owned();
    assert_eq!(func(num1, num2), "10".to_owned());
}

fn main() {
    check_solution(add_strings1);
    check_solution(add_strings2);
}

#[cfg(test)]
mod tests {
    use super::{add_strings1, add_strings2, check_solution};

    #[test]
    fn test_add_strings1() {
        check_solution(add_strings1);
    }

    #[test]
    fn test_add_strings2() {
        check_solution(add_strings2);
    }
}

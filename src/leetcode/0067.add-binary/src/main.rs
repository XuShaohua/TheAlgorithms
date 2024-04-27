// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::iter::Rev;
use std::slice::Iter;

pub fn add_binary1(a: String, b: String) -> String {
    let mut a_iter: Rev<Iter<u8>> = a.as_bytes().iter().rev();
    let mut b_iter = b.as_bytes().iter().rev();
    let mut result = Vec::with_capacity(a_iter.len().max(b_iter.len()) + 1);
    // 进位项
    let mut carry: u8 = 0;

    loop {
        let next_a = a_iter.next();
        let next_b = b_iter.next();
        // 循环的中止条件是, 所有字符串中的比特位都处理完, 且没有进位项.
        if carry == 0 && next_a.is_none() && next_b.is_none() {
            break;
        }

        // 计算当前比特位的值
        let mut bit = carry;
        if let Some(next_a) = next_a {
            bit += next_a - b'0';
        }
        if let Some(next_b) = next_b {
            bit += next_b - b'0';
        }
        // 更新进位项
        carry = bit / 2;
        bit %= 2;
        // 转换成 char 类型
        let bit_char = char::from(bit + b'0');
        result.push(bit_char);
    }

    // 逆序地将比特位转换成字符串
    result.iter().rev().collect()
}

pub type SolutionFn = fn(String, String) -> String;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(&str, &str, &str)] = &[("11", "1", "100"), ("1010", "1011", "10101")];

    for record in RECORDS {
        assert_eq!(
            func(record.0.to_owned(), record.1.to_owned()),
            record.2.to_owned()
        );
    }
}

fn main() {
    check_solution(add_binary1);
}

#[cfg(test)]
mod tests {
    use super::{add_binary1, check_solution};

    #[test]
    fn test_add_binary1() {
        check_solution(add_binary1);
    }
}

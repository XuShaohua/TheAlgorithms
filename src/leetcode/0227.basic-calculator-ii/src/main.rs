// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Stack
pub fn calculate1(s: String) -> i32 {
    assert!(!s.is_empty());

    let mut stack: Vec<i32> = Vec::new();
    let mut op: char = '+';
    let mut index = 0;
    let chars: Vec<char> = s.chars().collect();

    while index < chars.len() {
        let chr = chars[index];
        match chr {
            // 忽略空格
            ' ' => (),
            '+' | '-' | '*' | '/' => op = chr,
            '0'..='9' => {
                // 组装整数.
                let digit: u32 = chr.to_digit(10).unwrap();
                let mut num: i32 = digit as i32;
                while index + 1 < chars.len() && chars[index + 1].is_ascii_digit() {
                    let digit: u32 = chars[index + 1].to_digit(10).unwrap();
                    num = num * 10 + digit as i32;
                    index += 1;
                }

                // 与运算符结合, 进行计算.
                match op {
                    '+' => stack.push(num),
                    '-' => stack.push(-num),
                    '*' => {
                        let peek = stack.pop().unwrap();
                        stack.push(peek * num);
                    }
                    '/' => {
                        let peek = stack.pop().unwrap();
                        assert!(num != 0);
                        stack.push(peek / num);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        index += 1;
    }

    stack.iter().sum()
}

// No Stack
pub fn calculate2(s: String) -> i32 {
    assert!(!s.is_empty());

    let mut ans = 0;
    let mut last_num: i32 = 0;
    let mut index = 0;
    let mut op: char = '+';
    let chars: Vec<char> = s.chars().collect();

    while index < chars.len() {
        let chr = chars[index];
        match chr {
            // 忽略空格
            ' ' => (),
            '+' | '-' | '*' | '/' => op = chr,
            '0'..='9' => {
                // 组装整数
                let mut num: i32 = chr.to_digit(10).unwrap() as i32;
                while index + 1 < chars.len() && chars[index + 1].is_ascii_digit() {
                    let digit = chars[index + 1].to_digit(10).unwrap() as i32;
                    num = num * 10 + digit;
                    index += 1;
                }

                // 开始运算
                match op {
                    '+' => {
                        ans += last_num;
                        last_num = num;
                    }
                    '-' => {
                        ans += last_num;
                        last_num = -num;
                    }
                    '*' => last_num *= num,
                    '/' => last_num /= num,
                    _ => (),
                }
            }
            _ => (),
        }
        index += 1;
    }

    ans + last_num
}

pub type SolutionFn = fn(String) -> i32;

fn check_solution(func: SolutionFn) {
    let s = " 101 + 102 ".to_owned();
    assert_eq!(func(s), 203);

    let s = " 3/2 ".to_owned();
    assert_eq!(func(s), 1);

    let s = "3+2*2".to_owned();
    assert_eq!(func(s), 7);

    let s = " 3+5 / 2 ".to_owned();
    assert_eq!(func(s), 5);
}

fn main() {
    check_solution(calculate1);
    check_solution(calculate2);
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2, check_solution};

    #[test]
    fn test_calculate1() {
        check_solution(calculate1);
    }

    #[test]
    fn test_calculate2() {
        check_solution(calculate2);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Stack
// 后缀表达式
pub fn eval_rpn1(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens.iter() {
        match token.as_str() {
            // 匹配运算符.
            op @ ("+" | "-" | "*" | "/") => {
                assert!(stack.len() >= 2);
                // 要注意栈中两个整数的顺序.
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                let res = match op {
                    "+" => num2 + num1,
                    "-" => num2 - num1,
                    "*" => num2 * num1,
                    "/" => num2 / num1,
                    // unreachable!()
                    _ => 0,
                };
                // 将计算结果入栈.
                stack.push(res);
            }
            num_str => {
                //let num: i32 = i32::from_str(num_str).unwrap();
                //let num: i32 = i32::from_str_radix(num_str, 10).unwrap();
                let num: i32 = num_str.parse().unwrap();
                stack.push(num);
            }
        }
    }
    // 栈顶的元素就是计算结果.
    stack.pop().unwrap()
}

// Stack
// 后缀表达式
// - 优化模式匹配
// - 优化出栈操作
pub fn eval_rpn2(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());

    for token in tokens.iter() {
        match token.as_str() {
            // 匹配运算符.
            "+" => {
                debug_assert!(stack.len() >= 2);
                let num1 = stack.pop().unwrap();
                let num2 = stack.last_mut().unwrap();
                *num2 += num1;
            }
            "-" => {
                debug_assert!(stack.len() >= 2);
                let num1 = stack.pop().unwrap();
                let num2 = stack.last_mut().unwrap();
                *num2 -= num1;
            }
            "*" => {
                debug_assert!(stack.len() >= 2);
                let num1 = stack.pop().unwrap();
                let num2 = stack.last_mut().unwrap();
                *num2 *= num1;
            }
            "/" => {
                debug_assert!(stack.len() >= 2);
                let num1 = stack.pop().unwrap();
                let num2 = stack.last_mut().unwrap();
                *num2 /= num1;
            }
            num_str => {
                //let num: i32 = i32::from_str(num_str).unwrap();
                //let num: i32 = i32::from_str_radix(num_str, 10).unwrap();
                let num: i32 = num_str.parse().unwrap();
                stack.push(num);
            }
        }
    }
    // 栈顶的元素就是计算结果.
    stack.pop().unwrap()
}

pub type SolutionFn = fn(Vec<String>) -> i32;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(&[&str], i32)] = &[
        (&["2", "1", "+", "3", "*"], 9),
        (&["4", "13", "5", "/", "+"], 6),
        (
            &[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
            ],
            22,
        ),
    ];

    for record in RECORDS {
        let token: Vec<String> = record.0.iter().map(|val| val.to_string()).collect();
        let output = record.1;
        assert_eq!(func(token), output);
    }
}

fn main() {
    check_solution(eval_rpn1);
    check_solution(eval_rpn2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, eval_rpn1, eval_rpn2};

    #[test]
    fn test_eval_rpn1() {
        check_solution(eval_rpn1);
    }

    #[test]
    fn test_eval_rpn2() {
        check_solution(eval_rpn2);
    }
}

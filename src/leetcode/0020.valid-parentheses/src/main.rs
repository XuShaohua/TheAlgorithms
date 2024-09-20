// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Stack
pub fn is_valid1(s: String) -> bool {
    let mut stack = Vec::<char>::new();
    for bracket in s.chars() {
        // 如果是左侧括号, 就直接入栈.
        if bracket == '(' || bracket == '[' || bracket == '{' {
            stack.push(bracket);
            continue;
        }

        // 查看栈顶是否有元素, 如果有元素, 是否跟当前的括号配对.
        if let Some(last_bracket) = stack.last() {
            match (last_bracket, bracket) {
                ('(', ')') | ('[', ']') | ('{', '}') => stack.pop(),
                _ => return false,
            };
        } else {
            return false;
        }
    }

    stack.is_empty()
}

// Stack
// 做一些优化, 逻辑更简单
pub fn is_valid2(s: String) -> bool {
    let mut stack = Vec::<char>::new();
    for bracket in s.chars() {
        match bracket {
            // 先匹配左侧的括号, 并把与之成对的右侧括号入栈.
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            // 如果遇到右侧括号, 就把它与栈顶元素比较, 看是否相同.
            // 使用 match-guard
            ')' | ']' | '}' if Some(bracket) != stack.pop() => return false,
            _ => (),
        }
    }

    stack.is_empty()
}

pub type SolutionFn = fn(String) -> bool;

fn check_solution(func: SolutionFn) {
    let s = "()".to_owned();
    assert!(func(s));

    let s = "()[]{}".to_owned();
    assert!(func(s));

    let s = "(]".to_owned();
    assert!(!func(s));
}

fn main() {
    check_solution(is_valid1);
    check_solution(is_valid2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_valid1, is_valid2};

    #[test]
    fn test_is_valid1() {
        check_solution(is_valid1);
    }

    #[test]
    fn test_is_valid2() {
        check_solution(is_valid2);
    }
}

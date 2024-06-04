// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn generate_parenthesis1(_n: i32) -> Vec<String> {
    todo!();
}

pub type SolutionFn = fn(i32) -> Vec<String>;

fn check_solution(func: SolutionFn) {
    let n = 3;
    let expected = vec![
        "((()))".to_owned(),
        "(()())".to_owned(),
        "(())()".to_owned(),
        "()(())".to_owned(),
        "()()()".to_owned(),
    ];
    assert_eq!(func(n), expected);

    let n = 1;
    let expected = vec!["()".to_owned()];
    assert_eq!(func(n), expected);
}

fn main() {
    check_solution(generate_parenthesis1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, generate_parenthesis1};

    #[test]
    fn test_generate_parenthesis1() {
        check_solution(generate_parenthesis1);
    }
}

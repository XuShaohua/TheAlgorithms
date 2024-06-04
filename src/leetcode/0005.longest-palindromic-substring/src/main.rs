// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 远离型双指针
pub fn longest_palindrome1(_s: String) -> String {
    todo!()
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let s = "babad".to_owned();
    let exp = "bab".to_owned();
    assert_eq!(func(s), exp);

    let s = "cbbd".to_owned();
    let exp = "bb".to_owned();
    assert_eq!(func(s), exp);
}

fn main() {
    check_solution(longest_palindrome1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, longest_palindrome1};

    #[test]
    fn test_longest_palindrome1() {
        check_solution(longest_palindrome1);
    }
}

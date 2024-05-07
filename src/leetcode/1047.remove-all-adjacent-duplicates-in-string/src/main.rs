// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Stack
pub fn remove_duplicates1(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();

    for chr in s.chars() {
        let last_chr = stack.last();
        match last_chr {
            Some(last_chr) if *last_chr == chr => {
                let _ = stack.pop();
            }
            _ => stack.push(chr),
        }
    }
    stack.iter().collect()
}

// Stack
// 优化了一些
pub fn remove_duplicates2(s: String) -> String {
    let mut stack: Vec<char> = Vec::with_capacity(s.len());

    for chr in s.chars() {
        if stack.last() == Some(&chr) {
            stack.pop();
        } else {
            stack.push(chr);
        }
    }

    stack.into_iter().collect()
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let s = "abbaca".to_owned();
    assert_eq!(func(s), "ca".to_owned());

    let s = "azxxzy".to_owned();
    assert_eq!(func(s), "ay".to_owned());
}

fn main() {
    check_solution(remove_duplicates1);
    check_solution(remove_duplicates2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, remove_duplicates1, remove_duplicates2};

    #[test]
    fn test_remove_duplicates1() {
        check_solution(remove_duplicates1);
    }

    #[test]
    fn test_remove_duplicates2() {
        check_solution(remove_duplicates2);
    }
}

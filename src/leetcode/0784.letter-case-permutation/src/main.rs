// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn letter_case_permutation1(s: String) -> Vec<String> {
    fn backtracking(chars: &[char], index: usize, path: &mut Vec<char>, res: &mut Vec<String>) {
        // 终止条件就是所有字符都已经访问过.
        if index == chars.len() {
            res.push(String::from_iter(path.iter()));
            return;
        }

        // 访问当前元素.
        let chr: char = chars[index];
        // 如果当前字符是数字, 就直接访问下个元素.
        path.push(chr);
        backtracking(chars, index + 1, path, res);
        path.pop();

        // 如果当前字符是字母, 就要处理大写和小写.
        if chr.is_ascii_alphabetic() {
            path.push(chr.to_ascii_uppercase());
            backtracking(chars, index + 1, path, res);
            path.pop();
        }
    }

    let chars: Vec<char> = s.to_ascii_lowercase().chars().collect();

    // 存储所有结果.
    let mut res: Vec<String> = Vec::new();
    // 存储当前的路径.
    let mut path: Vec<char> = Vec::new();
    backtracking(&chars, 0, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(String) -> Vec<String>;

fn check_solution(func: SolutionFn) {
    let s = "a1b2".to_owned();
    let expected = vec![
        "a1b2".to_owned(),
        "a1B2".to_owned(),
        "A1b2".to_owned(),
        "A1B2".to_owned(),
    ];
    assert_eq!(func(s), expected);

    let s = "3z4".to_owned();
    let expected = vec!["3z4".to_owned(), "3Z4".to_owned()];
    assert_eq!(func(s), expected);
}

fn main() {
    check_solution(letter_case_permutation1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, letter_case_permutation1};

    #[test]
    fn test_letter_case_permutation1() {
        check_solution(letter_case_permutation1);
    }
}

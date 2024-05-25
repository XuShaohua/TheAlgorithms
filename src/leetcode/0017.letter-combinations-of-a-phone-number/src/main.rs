// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn letter_combinations1(digits: String) -> Vec<String> {
    /// 数字到字母的映射.
    #[must_use]
    const fn digit_map(digit: char) -> &'static [char] {
        match digit {
            '2' => &['a', 'b', 'c'],
            '3' => &['d', 'e', 'f'],
            '4' => &['g', 'h', 'i'],
            '5' => &['j', 'k', 'l'],
            '6' => &['m', 'n', 'o'],
            '7' => &['p', 'q', 'r', 's'],
            '8' => &['t', 'u', 'v'],
            '9' => &['w', 'x', 'y', 'z'],
            _ => &[],
        }
    }

    fn backtracking(digits: &[char], index: usize, path: &mut Vec<char>, res: &mut Vec<String>) {
        // 终止条件就是所有字符都已经处理完毕.
        if index == digits.len() {
            res.push(String::from_iter(path.iter()));
            return;
        }

        // 遍历当前数字对应的所有字符.
        let chars = digit_map(digits[index]);
        for &chr in chars {
            // 访问字符
            path.push(chr);

            // 递归搜索
            backtracking(digits, index + 1, path, res);

            // 撤销访问
            path.pop();
        }
    }

    if digits.is_empty() {
        return Vec::new();
    }

    // 保存所有结果.
    let mut res: Vec<String> = Vec::new();
    // 保存当前路径上的字符.
    let mut path: Vec<char> = Vec::new();
    let digits: Vec<char> = digits.chars().collect();
    backtracking(&digits, 0, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(String) -> Vec<String>;

fn check_solution(func: SolutionFn) {
    let digits = "23".to_owned();
    let expected = vec![
        "ad".to_owned(),
        "ae".to_owned(),
        "af".to_owned(),
        "bd".to_owned(),
        "be".to_owned(),
        "bf".to_owned(),
        "cd".to_owned(),
        "ce".to_owned(),
        "cf".to_owned(),
    ];
    assert_eq!(func(digits), expected);

    let digits = "".to_owned();
    let expected: Vec<String> = Vec::new();
    assert_eq!(func(digits), expected);

    let digits = "2".to_owned();
    let expected = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
    assert_eq!(func(digits), expected);
}

fn main() {
    check_solution(letter_combinations1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, letter_combinations1};

    #[test]
    fn test_letter_combinations1() {
        check_solution(letter_combinations1);
    }
}

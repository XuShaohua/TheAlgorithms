// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 远离型双指针
pub fn longest_palindrome1(s: String) -> String {
    fn two_sides_equal(s: &str, s_len: i64, left: i64, right: i64) -> bool {
        // 先检查两个索引值的有效性, 再比较它们指向的字符是否相等.
        if left >= 0
            && right < s_len
            && s.as_bytes().get(left as usize) == s.as_bytes().get(right as usize)
        {
            return true;
        } else {
            return false;
        }
    }

    let mut longest_palindrome_len: i64 = 0;
    let mut longest_palindrome_start: i64 = 0;

    let s_len = s.len() as i64;

    // 遍历所有字符
    for middle in 0..s_len {
        // 以 (middle) 为对称点, substring 有奇数个字符
        let mut left = middle;
        let mut right = middle;
        while two_sides_equal(&s, s_len, left, right) {
            let length = right - left + 1;
            if longest_palindrome_len < length {
                longest_palindrome_len = length;
                longest_palindrome_start = left;
            }
            left -= 1;
            right += 1;
        }

        // 以 (middle, middle+1) 作为对称点, substring 有偶数个字符
        left = middle;
        right = middle + 1;
        while two_sides_equal(&s, s_len, left, right) {
            let length = right - left + 1;
            if longest_palindrome_len < length {
                longest_palindrome_len = length;
                longest_palindrome_start = left;
            }
            left -= 1;
            right += 1;
        }
    }

    // 返回结果
    let longest_palindrome_start = longest_palindrome_start as usize;
    let longest_palindrome_len = longest_palindrome_len as usize;
    s[longest_palindrome_start..longest_palindrome_start + longest_palindrome_len].to_owned()
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

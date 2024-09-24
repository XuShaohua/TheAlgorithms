// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 双指针法
pub fn compress_string1(s: String) -> String {
    let mut out = String::new();

    // 遍历所有字符
    let mut left = 0;
    let bytes = s.as_bytes();
    let len = bytes.len();
    while left < len {
        // 注意边界值
        let mut right = left;
        while right < len && bytes[left] == bytes[right] {
            right += 1;
        }
        out.push(char::from(bytes[left]));
        out.push_str(&(right - left).to_string());

        // 将左侧边界向右移
        left = right;
    }

    if out.len() < s.len() {
        out
    } else {
        s
    }
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    {
        let s1 = "aabcccccaaa".to_owned();
        let expected = "a2b1c5a3";
        let s2 = func(s1);
        assert_eq!(s2, expected);
    }

    {
        let s1 = "abbccd".to_owned();
        let expected = "abbccd";
        let s2 = func(s1);
        assert_eq!(s2, expected);
    }
}

fn main() {
    check_solution(compress_string1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, compress_string1};

    #[test]
    fn test_compress_string1() {
        check_solution(compress_string1);
    }
}

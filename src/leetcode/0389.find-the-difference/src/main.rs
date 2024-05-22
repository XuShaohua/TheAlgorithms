// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 排序
pub fn find_the_difference1(s: String, t: String) -> char {
    debug_assert!(s.len() + 1 == t.len());

    // 先排序
    let mut s_chars: Vec<char> = s.chars().collect();
    s_chars.sort_unstable();
    let mut t_chars: Vec<char> = t.chars().collect();
    t_chars.sort_unstable();

    // TODO(Shaohua): binary search

    // 再依次遍历
    for (s_chr, t_chr) in s_chars.iter().zip(t_chars.iter()) {
        if s_chr != t_chr {
            return *t_chr;
        }
    }

    t_chars.last().copied().unwrap()
}

// 转换成整数, 求和
pub fn find_the_difference2(s: String, t: String) -> char {
    let sum_s: u32 = s.chars().map(|chr| chr as u32).sum();
    let sum_t: u32 = t.chars().map(|chr| chr as u32).sum();
    char::from_u32(sum_t - sum_s).unwrap()
}

pub type SolutionFn = fn(String, String) -> char;

fn check_solution(func: SolutionFn) {
    let s = "abcd".to_owned();
    let t = "abcde".to_owned();
    assert_eq!(func(s, t), 'e');

    let s = "".to_owned();
    let t = "y".to_owned();
    assert_eq!(func(s, t), 'y');
}

fn main() {
    check_solution(find_the_difference1);
    check_solution(find_the_difference2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_the_difference1, find_the_difference2};

    #[test]
    fn test_find_the_difference1() {
        check_solution(find_the_difference1);
    }

    #[test]
    fn test_find_the_difference2() {
        check_solution(find_the_difference2);
    }
}

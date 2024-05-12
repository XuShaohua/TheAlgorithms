// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Bruteforce
// 时间复杂度: O(n^2)
pub fn longest_common_prefix1(strs: Vec<String>) -> String {
    debug_assert!(!strs.is_empty());

    let mut index: usize = 0;
    let mut stop: bool = false;
    // 使用第一个字符串作为参考, 因为输出结果不可能比第一个字符串长.
    let first_bytes: &[u8] = strs[0].as_bytes();

    while !stop {
        // 如果第
        if index == first_bytes.len() {
            break;
        }

        // 遍历其余的字符串, 然后依次对比当前位置的字符是否相等.
        for s in strs.iter().skip(1) {
            let bytes: &[u8] = s.as_bytes();
            if index >= bytes.len() || first_bytes[index] != bytes[index] {
                stop = true;
                break;
            }
        }

        if !stop {
            index += 1;
        }
    }

    // 重新组装字符串.
    let vec: Vec<u8> = first_bytes[..index].to_vec();
    String::from_utf8(vec).unwrap()
}

// TODO(Shaohua): Trie

pub type SolutionFn = fn(Vec<String>) -> String;

fn check_solution(func: SolutionFn) {
    let strs = vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()];
    assert_eq!(func(strs), "fl".to_owned());

    let strs = vec!["dog".to_owned(), "racecar".to_owned(), "car".to_owned()];
    assert_eq!(func(strs), "".to_owned());
}

fn main() {
    check_solution(longest_common_prefix1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, longest_common_prefix1};

    #[test]
    fn test_longest_common_prefix1() {
        check_solution(longest_common_prefix1);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Stack
pub fn decode_string1(s: String) -> String {
    assert!(!s.is_empty());

    // 用于存储 '[' 之前的数字, 可能是多位数.
    let mut num_stack: Vec<i32> = Vec::new();
    // 存储 '[' 之前的字符串.
    let mut str_stack: Vec<Vec<char>> = Vec::new();

    let chars: Vec<char> = s.chars().collect();
    // 存放当前的字符串.
    let mut parts: Vec<char> = Vec::new();
    // 存放当前的数字.
    let mut num: i32 = 0;

    for chr in chars {
        match chr {
            chr if chr.is_ascii_digit() => {
                // 处理数字
                let digit = chr.to_digit(10).unwrap() as i32;
                num = num * 10 + digit;
            }
            '[' => {
                // 将 '[' 之前的数字和字符串入栈.
                num_stack.push(num);
                str_stack.push(parts.clone());

                // 并重置它们.
                parts.clear();
                num = 0;
            }
            ']' => {
                // 收网, 从两个栈中分别取出整数和字符串, 进行一次拼装,
                // 然后将拼装结果入字符串栈.
                //
                // curr_num 是当前字符串重复次数.
                let curr_num = num_stack.pop().unwrap();
                // last_parts 是 '[' 之前的字符串, 相当于当前字符串的前缀.
                let last_parts = str_stack.pop().unwrap();
                // 合成的新的字符串.
                // parts = last_parts + curr_parts * curr_num.
                let curr_parts = parts;
                parts = last_parts;

                for _i in 0..curr_num {
                    parts.extend_from_slice(&curr_parts);
                }
            }
            letter => {
                // 拾取所有字符
                parts.push(letter);
            }
        }
    }

    // 组装最后的字符串
    parts.into_iter().collect()
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let s = "3[a]2[bc]".to_owned();
    assert_eq!(&func(s), "aaabcbc");

    let s = "3[a2[c]]".to_owned();
    assert_eq!(&func(s), "accaccacc");

    let s = "2[abc]3[cd]ef".to_owned();
    assert_eq!(&func(s), "abcabccdcdcdef");

    let s = "100[leetcode]".to_owned();
    assert_eq!(func(s).len(), 800);

    let s = "3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_owned();
    assert_eq!(
        &func(s),
        "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef"
    );
}

fn main() {
    check_solution(decode_string1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, decode_string1};

    #[test]
    fn test_decode_string1() {
        check_solution(decode_string1);
    }
}

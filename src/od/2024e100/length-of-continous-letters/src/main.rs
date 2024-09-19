// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    // 读取输入
    let mut s = String::new();
    let ret = stdin().lock().read_line(&mut s);
    assert!(ret.is_ok());
    let mut k_str = String::new();
    let ret = stdin().lock().read_line(&mut k_str);
    assert!(ret.is_ok());
    let k: usize = k_str.trim().parse().unwrap();

    // 先遍历字符串, 分隔出连续相同字符, 然后统计其个数, 存放到计数字典中
    let mut current_char: char = 'A';
    let mut current_char_count: usize = 0;
    let mut char_dict: HashMap<char, usize> = HashMap::new();
    for chr in s.trim().chars() {
        // 如果当前的字符与上个字符不相同
        if current_char != chr {
            // 保存到字典中
            if current_char_count > 0 {
                // 如果该字符在字典中已经存在, 则只保存最大连续数
                if let Some(last_count) = char_dict.get_mut(&current_char) {
                    *last_count = current_char_count.max(*last_count);
                } else {
                    char_dict.insert(current_char, current_char_count);
                }
            }

            // 重置上个字符及其计数
            current_char = chr;
            current_char_count = 1;
        } else {
            current_char_count += 1;
        }
    }

    // 处理最后一个字符
    if current_char_count > 0 {
        // 如果该字符在字典中已经存在, 则只保存最大连续数
        if let Some(last_count) = char_dict.get_mut(&current_char) {
            *last_count = current_char_count.max(*last_count);
        } else {
            char_dict.insert(current_char, current_char_count);
        }
    }

    // 将字典转换成列表
    let mut word_list: Vec<(char, usize)> = char_dict.into_iter().collect();
    // 基于最大连续数进行排序, 从高到低
    word_list.sort_by_key(|pair| Reverse(pair.1));
    //println!("{word_list:?}");

    // 并找到第 k 个字符, 注意下标从0开始计数, 而k是从1开始的
    if k <= word_list.len() {
        println!("{}", word_list[k - 1].1);
    } else {
        println!("-1");
    }
}

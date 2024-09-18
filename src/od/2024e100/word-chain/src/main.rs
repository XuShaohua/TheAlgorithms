// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{stdin, BufRead};

fn main() {
    // 读取输入
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let first_word_index: usize = line.trim().parse().unwrap();

    line.clear();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let word_count: usize = line.trim().parse().unwrap();
    assert!(first_word_index < word_count);

    let mut words: Vec<String> = Vec::with_capacity(word_count);
    for _i in 0..word_count {
        line.clear();
        let ret = stdin().lock().read_line(&mut line);
        assert!(ret.is_ok());
        words.push(line.trim().to_owned());
    }

    // 将第一个单词从字典中去除, 然后重整字典
    let mut ans: String = words.remove(first_word_index).to_owned();

    // 开始接龙, 如果字典已经空了, 就终止
    while !words.is_empty() {
        let next_char: char = ans.chars().last().unwrap();
        let mut last_word_index = usize::MAX;
        let mut last_word = String::new();

        // 遍历字典, 找到以 next_char 开头的单词
        for (index, word) in words.iter().enumerate() {
            // 将当前词更新为 last_word 的条件有:
            // - 当前词的长度比上个词长
            // - 或者当前词的字典序小于上个词
            if word.starts_with(next_char) {
                if word.len() > last_word.len()
                    || (word.len() == last_word.len() && *word < last_word)
                {
                    last_word = word.to_string();
                    last_word_index = index;
                }
            }
        }
        // 没有找到合适的单词, 终止循环
        if last_word_index == usize::MAX {
            break;
        }

        // 接龙, 并将该单词从字典中移除
        ans.push_str(&last_word);
        words.remove(last_word_index);
    }

    // 打印结果
    println!("{ans}");
}

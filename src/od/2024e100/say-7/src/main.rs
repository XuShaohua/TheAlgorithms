// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{stdin, BufRead};

fn number_digits_contains(mut num: u32, digit: u8) -> bool {
    while num > 0 {
        let d = (num % 10) as u8;
        if d == digit {
            return true;
        }
        num /= 10;
    }
    false
}

fn solution() {
    // 读取输入
    // 确定有人数 n
    // 确定总共喊了多少声
    // 基于此, 就可以确定喊的顺序
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let pass_list: Vec<u32> = line
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // 得到人数
    let num_people = pass_list.len();
    assert!(0 < num_people);
    // 喊过的总次数
    let mut total_passes: u32 = pass_list.iter().sum();

    // 喊过的条件是:
    // 1. 当前数字是7的倍数
    // 2. 当前数字中包含7

    // 每个人真正喊过的次数
    let mut real_pass_list: Vec<usize> = vec![0; num_people];

    // 当前的喊的数字 k, 注意数字是从1开始
    let mut current_num = 1;
    // 当前喊数的人在队列中的位置, 从0开始计数
    let mut current_person = 0;

    // 一直循环, 直到喊过的次数用完了
    while total_passes > 0 {
        if current_num % 7 == 0 && number_digits_contains(current_num, 7) {
            // 这个人要喊过
            real_pass_list[current_person] += 1;
            total_passes -= 1;
        }
        current_num += 1;
        current_person = (current_person + 1) % num_people;
    }

    // 打印结果
    let s = real_pass_list
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{s}");
}

fn main() {
    solution();
}
